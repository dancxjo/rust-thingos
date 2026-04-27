use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::super::SCHEDULER;
use super::stdio::{StdioSpec, setup_stdio_fds};
use super::types::SpawnExResult;
use crate::task::ProcessInfo;
use crate::{BootRuntime, BootTasking};

/// General-purpose runtime process creation from a VFS path.
///
/// This is the **standard runtime process creation path** that follows the
/// ThingOS process model:
///
/// 1. Open the executable from the VFS (e.g. `/usr/bin/ls`).
/// 2. Build a new process object and initial thread.
/// 3. Apply inheritance/replacement for stdio, fds, cwd, and env.
/// 4. Schedule the new thread for execution.
///
/// Unlike the boot helpers ([`boot_spawn_process`], [`boot_spawn_process_ex`]),
/// this function does **not** consult the boot module table and has no
/// boot-specific assumptions.  It is the correct function to call for any
/// runtime `SYS_SPAWN_PROCESS_EX` invocation.
///
/// # Safety
/// Must be called with scheduler lock expectations satisfied.
pub unsafe fn spawn_process_from_path<R: BootRuntime>(
    path: &str,
    argv: Vec<Vec<u8>>,
    env: BTreeMap<Vec<u8>, Vec<u8>>,
    stdin_spec: StdioSpec,
    stdout_spec: StdioSpec,
    stderr_spec: StdioSpec,
    boot_arg: u64,
    inherited_handles: Vec<u64>,
    cwd: Option<alloc::string::String>,
    fd_remap: Vec<abi::types::HandleRemap>,
    entry_sym_override: Option<alloc::string::String>,
) -> Result<SpawnExResult, abi::errors::Errno> {
    #[cfg(feature = "spawn_timing")]
    let t_spawn_start = crate::trace::now();

    // Step 1: Open the executable from the VFS.
    #[cfg(feature = "spawn_timing")]
    let t_vfs_open_start = crate::trace::now();
    let node = crate::vfs::mount::lookup(path).map_err(|_| abi::errors::Errno::ENOENT)?;

    let stat = node.stat().map_err(|e| e)?;
    if !stat.is_reg() {
        return Err(abi::errors::Errno::EACCES);
    }

    let size = stat.size as usize;
    if size > 64 * 1024 * 1024 {
        return Err(abi::errors::Errno::EFBIG);
    }
    #[cfg(feature = "spawn_timing")]
    let t_vfs_open_end = crate::trace::now();

    // Step 2: Read the ELF bytes into kernel memory, consulting the page cache
    // first to avoid repeated IPC round-trips to iso9660d for the same binary.
    #[cfg(feature = "spawn_timing")]
    let t_elf_read_start = crate::trace::now();
    #[cfg(feature = "spawn_timing")]
    let mut elf_read_calls: u32 = 0;
    // `cached_buf` is declared here (outside the if-else) so that its lifetime
    // covers `buffer`, which borrows from it.
    let cached_buf: alloc::sync::Arc<alloc::vec::Vec<u8>>;
    let buffer: &[u8] = if let Some(cached) = crate::vfs::page_cache::get(path) {
        crate::kdebug!("SPAWN: page cache hit for '{}'", path);
        cached_buf = cached;
        &cached_buf[..]
    } else {
        let mut buf = alloc::vec![0u8; size];
        let read_n = node.read_all_into(&mut buf).map_err(|e| e)?;
        #[cfg(feature = "spawn_timing")]
        {
            elf_read_calls += 1;
        }
        if read_n < size {
            return Err(abi::errors::Errno::EIO);
        }
        crate::kdebug!("SPAWN: page cache miss for '{}', caching {} bytes", path, size);
        let arc_buf = alloc::sync::Arc::new(buf);
        crate::vfs::page_cache::put(path, arc_buf.clone());
        cached_buf = arc_buf;
        &cached_buf[..]
    };
    #[cfg(feature = "spawn_timing")]
    let t_elf_read_end = crate::trace::now();

    // Step 3: Load the ELF into a fresh address space.
    #[cfg(feature = "spawn_timing")]
    let t_aspace_start = crate::trace::now();
    let rt = crate::runtime::<R>();
    let current_cpu = crate::sched::current_cpu_index::<R>();
    let aspace = rt.tasking().make_user_address_space();
    #[cfg(feature = "spawn_timing")]
    let t_aspace_end = crate::trace::now();

    #[cfg(feature = "spawn_timing")]
    let t_elf_load_start = crate::trace::now();
    // SAFETY: `load_module` is synchronous and does not retain the reference
    // beyond the call.  The `cached_buf` Arc keeps the backing allocation alive
    // for the entire duration of this function, so extending the lifetime to
    // `'static` for the synchronous call is safe.
    let static_bytes: &'static [u8] = unsafe { core::mem::transmute(buffer) };
    let basename = path.rsplit('/').next().unwrap_or(path);
    // SAFETY: `load_module` is synchronous; `basename` outlives the call.
    let static_name: &'static str = unsafe { core::mem::transmute(basename) };
    let module_desc = crate::BootModuleDesc {
        name: static_name,
        cmdline: "",
        bytes: static_bytes,
        phys_start: 0,
        phys_end: 0,
        kind: crate::BootModuleKind::Elf,
    };

    let (mut entry, stack_info, regions, aux_info) =
        crate::task::loader::load_module(rt, aspace, &module_desc)
            .ok_or(abi::errors::Errno::ENOEXEC)?;
    entry.arg0 = boot_arg as usize;
    #[cfg(feature = "spawn_timing")]
    let t_elf_load_end = crate::trace::now();

    // If the caller requested a specific driver entrypoint symbol, resolve it
    // from the binary bytes and override the default ELF e_entry.  The symbol
    // value is the file-relative VMA; the load bias is (load_base - min_vaddr)
    // which `load_module` hard-codes to `0x200000 - min_vaddr`.  We derive it
    // as `aux_info.entry_vaddr - elf_e_entry`, but the simpler approach is to
    // re-read the min_vaddr directly from the binary.
    if let Some(sym_name) = &entry_sym_override {
        if let Some(sym_vaddr) =
            crate::task::loader::resolve_elf64_symbol(buffer, sym_name.as_str())
        {
            // Compute load bias: default load base is 0x200000; subtract min_vaddr.
            // The helper returns the file-relative VMA so we apply the same bias
            // that load_module_at used.
            let default_load_base: u64 = 0x200000;
            // We need min_vaddr.  Since we already loaded the binary and have
            // aux_info.entry_vaddr == elf_e_entry + bias, derive bias as:
            //   bias = entry_vaddr - (elf_e_entry from binary)
            // We don't have elf_e_entry separately, but we can re-read it cheaply.
            let elf_e_entry = crate::task::loader::read_elf64_entry(buffer).unwrap_or(0);
            let load_bias = if elf_e_entry != 0 {
                (aux_info.entry_vaddr as i64).wrapping_sub(elf_e_entry as i64) as u64
            } else {
                default_load_base
            };
            let resolved_pc = sym_vaddr.wrapping_add(load_bias) as usize;
            if resolved_pc < 0x1000 {
                crate::kerror!(
                    "SPAWN: entry symbol '{}' in '{}' resolved to invalid PC 0x{:x}",
                    sym_name,
                    path,
                    resolved_pc
                );
                return Err(abi::errors::Errno::ENOEXEC);
            }
            crate::kdebug!(
                "SPAWN: driver entrypoint override '{}' => VA 0x{:x} + bias 0x{:x} = PC 0x{:x}",
                sym_name,
                sym_vaddr,
                load_bias,
                resolved_pc
            );
            entry.entry_pc = resolved_pc;
            // Ensure user stack is 16-byte aligned.
            entry.user_sp = entry.user_sp & !0xF;
        } else {
            crate::kerror!("SPAWN: entry symbol '{}' not found in '{}'", sym_name, path);
            return Err(abi::errors::Errno::ENOEXEC);
        }
    }

    // Step 4: Create the scheduler task for the new process's initial thread.
    // Phase 1: minimal SCHEDULER critical section — allocate TID and register
    // scheduler-internal state. REGISTRY insertion is deferred until unlock.
    #[cfg(feature = "spawn_timing")]
    let t_task_create_start = crate::trace::now();
    let _irq = rt.irq_disable();

    crate::kdebug!("SPAWN_FROM_PATH: Starting Phase 1 for {}", path);
    let (id, deferred_registry_inserts) = {
        let lock = SCHEDULER.lock();
        let _tracking = crate::sched::sched_lock_tracking_guard::<R>(current_cpu);
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut crate::sched::types::Scheduler<R>) };
        let id = sched
            .spawn_user_task_deferred(
                entry,
                aspace,
                stack_info,
                regions,
                crate::task::TaskPriority::Normal,
                crate::task::Affinity::Any,
            )
            .ok_or(abi::errors::Errno::EAGAIN)?;
        let deferred_registry_inserts = sched.drain_pending_registry_inserts();
        (id, deferred_registry_inserts)
    };
    #[cfg(feature = "spawn_timing")]
    let t_task_create_end = crate::trace::now();
    crate::kdebug!("SPAWN_FROM_PATH: Phase 1 complete, ID={}, applying inserts", id);
    crate::sched::apply_deferred_registry_inserts::<R>(deferred_registry_inserts);
    crate::kdebug!("SPAWN_FROM_PATH: Inserts applied, entering Phase 2");

    // Phase 2: post-spawn setup — REGISTRY lock only, no SCHEDULER held.
    // The task is Blocked and cannot be scheduled until wake_task(id) is called.

    // Determine parent PID from the running task.
    let ppid = {
        let tid = rt.current_tid();
        crate::task::registry::get_task::<R>(tid)
            .and_then(|t| t.process_info.clone())
            .map(|pi| pi.lock().pid)
            .unwrap_or(0)
    };

    // Step 5: Resolve argv — fall back to the executable basename.
    let final_argv = if argv.is_empty() { alloc::vec![basename.as_bytes().to_vec()] } else { argv };

    // Step 6: Inherit and set up stdio fds in the child's handle_table.
    #[cfg(feature = "spawn_timing")]
    let t_stdio_setup_start = crate::trace::now();
    let parent_tid = rt.current_tid();
    let parent_pinfo =
        crate::task::registry::get_task::<R>(parent_tid).and_then(|t| t.process_info.clone());

    let (mut handle_table, ipc_table) = if let Some(parent_pi) = &parent_pinfo {
        let parent = parent_pi.lock();
        (parent.handle_table.clone(), parent.ipc_table.clone())
    } else {
        (crate::vfs::handle_table::HandleTable::new(), crate::ipc::IpcHandleTable::new())
    };

    // Clear stdio slots inherited from parent so setup_stdio_fds's insert_at
    // calls don't fail silently with EBADF (slot already occupied).
    let _ = handle_table.close(0);
    let _ = handle_table.close(1);
    let _ = handle_table.close(2);

    let (stdin_pipe_id, stdout_pipe_id, stderr_pipe_id) =
        setup_stdio_fds::<R>(&mut handle_table, stdin_spec, stdout_spec, stderr_spec);

    // Step 6b: Apply explicit FD remappings.
    for remap in fd_remap {
        if let Err(e) = handle_table.dup2(remap.src_handle, remap.dst_handle) {
            crate::kwarn!(
                "SPAWN: FD remap failed: {} -> {} (errno {:?})",
                remap.src_handle,
                remap.dst_handle,
                e
            );
        }
    }

    // Close any handles marked with CLOEXEC before the child takes ownership.
    handle_table.close_on_exec();

    // Open the parent-side pipe ends in the parent's handle_table.
    let mut parent_stdin_fd: u64 = 0;
    let mut parent_stdout_fd: u64 = 0;
    let mut parent_stderr_fd: u64 = 0;
    if let Some(parent_pi) = &parent_pinfo {
        let mut plk = parent_pi.lock();
        if stdin_pipe_id != 0 {
            if let Some(write_node) = crate::ipc::pipe::write_node_for_id(stdin_pipe_id) {
                if let Ok(fd) = plk.handle_table.open(
                    write_node,
                    crate::vfs::OpenFlags::write_only(),
                    alloc::format!("pipe:{}", stdin_pipe_id),
                ) {
                    parent_stdin_fd = fd as u64;
                }
            }
        }
        if stdout_pipe_id != 0 {
            if let Some(read_node) = crate::ipc::pipe::read_node_for_id(stdout_pipe_id) {
                if let Ok(fd) = plk.handle_table.open(
                    read_node,
                    crate::vfs::OpenFlags::read_only(),
                    alloc::format!("pipe:{}", stdout_pipe_id),
                ) {
                    parent_stdout_fd = fd as u64;
                }
            }
        }
        if stderr_pipe_id != 0 {
            if let Some(read_node) = crate::ipc::pipe::read_node_for_id(stderr_pipe_id) {
                if let Ok(fd) = plk.handle_table.open(
                    read_node,
                    crate::vfs::OpenFlags::read_only(),
                    alloc::format!("pipe:{}", stderr_pipe_id),
                ) {
                    parent_stderr_fd = fd as u64;
                }
            }
        }
    }

    // Share the mapping list between the Thread and the Process.
    #[cfg(feature = "spawn_timing")]
    let t_stdio_setup_end = crate::trace::now();
    let task_mappings =
        crate::task::registry::get_task::<R>(id).map(|t| t.mappings.clone()).unwrap_or_else(|| {
            alloc::sync::Arc::new(spin::Mutex::new(crate::memory::mappings::MappingList::new()))
        });

    let aspace_raw = rt.tasking().aspace_to_raw(aspace);

    let unix_compat = if let Some(parent_pi) = &parent_pinfo {
        let parent = parent_pi.lock();
        let mut uc = crate::task::ProcessUnixCompat::inherit(&parent.unix_compat);
        uc.set_spawn_context(
            final_argv,
            crate::task::exec::build_auxv(&aux_info, rt.page_size() as u64),
        );
        uc.env = env;
        uc
    } else {
        let mut uc = crate::task::ProcessUnixCompat::isolated(id as u32, true);
        uc.set_spawn_context(
            final_argv,
            crate::task::exec::build_auxv(&aux_info, rt.page_size() as u64),
        );
        uc.env = env;
        uc
    };
    let authority = if let Some(parent_pi) = &parent_pinfo {
        parent_pi.lock().authority
    } else {
        crate::task::ProcessAuthority::root()
    };

    // Step 7: Build the ProcessInfo for the new process.
    #[cfg(feature = "spawn_timing")]
    let t_proc_reg_start = crate::trace::now();
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
        pid: id as u32,
        job: crate::task::ProcessLifecycle::new(ppid, id),
        unix_compat,
        handle_table,
        ipc_table,
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: if let Some(explicit_cwd) = cwd {
            explicit_cwd
        } else if let Some(parent_pi) = &parent_pinfo {
            parent_pi.lock().cwd.clone()
        } else {
            alloc::string::String::from("/")
        },
        root: if let Some(parent_pi) = &parent_pinfo {
            parent_pi.lock().root.clone()
        } else {
            alloc::string::String::from("/")
        },
        exec_path: alloc::string::String::from(path),
        authority,
        space: crate::task::ProcessAddressSpace::from_parts(task_mappings, aspace_raw),
        service_loop: None,
    }));

    // Step 8: Attach the ProcessInfo to the new task and record its TLS base.
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
        let len = basename.len().min(32);
        task.name[..len].copy_from_slice(&basename.as_bytes()[..len]);
        task.name_len = len as u8;
        task.process_info = Some(pinfo);
        task.user_fs_base = aux_info.tls_tp;
    }

    // `inherited_handles` is reserved for future fd-inheritance; not yet wired.
    let _ = inherited_handles;
    #[cfg(feature = "spawn_timing")]
    let t_proc_reg_end = crate::trace::now();

    // Phase 3: make the task runnable.  wake_task acquires SCHEDULER briefly
    // to transition Blocked → Runnable and enqueue the task.
    crate::kdebug!("SPAWN_FROM_PATH: Phase 2 complete, waking task {}", id);
    crate::sched::blocking::wake_task::<R>(id);
    crate::kdebug!("SPAWN_FROM_PATH: Task {} woken, restoring IRQs", id);
    rt.irq_restore(_irq);
    crate::kdebug!("SPAWN_FROM_PATH: Done for {}", path);

    #[cfg(feature = "spawn_timing")]
    {
        let t_total_end = crate::trace::now();
        let total_us = t_total_end.saturating_sub(t_spawn_start) / 1_000;
        let vfs_open_us = t_vfs_open_end.saturating_sub(t_vfs_open_start) / 1_000;
        let elf_read_us = t_elf_read_end.saturating_sub(t_elf_read_start) / 1_000;
        let aspace_us = t_aspace_end.saturating_sub(t_aspace_start) / 1_000;
        let elf_load_us = t_elf_load_end.saturating_sub(t_elf_load_start) / 1_000;
        let task_create_us = t_task_create_end.saturating_sub(t_task_create_start) / 1_000;
        let stdio_us = t_stdio_setup_end.saturating_sub(t_stdio_setup_start) / 1_000;
        let proc_reg_us = t_proc_reg_end.saturating_sub(t_proc_reg_start) / 1_000;
        crate::kinfo!(
            "SPAWN_TIMING '{}': total={}µs | vfs_open={}µs elf_read={}µs({} reads, {} bytes) \
             aspace={}µs elf_load={}µs task_create={}µs stdio={}µs proc_reg={}µs",
            path,
            total_us,
            vfs_open_us,
            elf_read_us,
            elf_read_calls,
            size,
            aspace_us,
            elf_load_us,
            task_create_us,
            stdio_us,
            proc_reg_us,
        );
    }

    Ok(SpawnExResult {
        child_tid: id,
        child_pid: id as u32,
        stdin_pipe: parent_stdin_fd,
        stdout_pipe: parent_stdout_fd,
        stderr_pipe: parent_stderr_fd,
    })
}
