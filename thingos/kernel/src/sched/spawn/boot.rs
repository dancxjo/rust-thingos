use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::super::SCHEDULER;
use super::process_info::{boot_module_matches, inherit_process_info};
use super::stdio::{StdioSpec, setup_stdio_fds};
use super::types::SpawnExResult;
use crate::sched::types::Scheduler;
use crate::task::{ProcessInfo, StartupArg, TaskId};
use crate::{BootRuntime, BootTasking};

/// Boot-only helper: spawn a process from a boot module by name at normal priority.
///
/// This is a **boot-time convenience**.  It locates the named module in the
/// static boot module table (`BootRuntime::modules`) and launches it.  It must
/// **not** be used for runtime process creation; call
/// [`spawn_process_from_path`] instead.
pub unsafe fn boot_spawn_process<R: BootRuntime>(name: &str, arg: StartupArg) -> Option<TaskId> {
    unsafe { boot_spawn_process_with_priority::<R>(name, arg, crate::task::TaskPriority::Normal) }
}

/// Boot-only helper: spawn a process from a boot module at a given priority.
///
/// Same as [`boot_spawn_process`] but with an explicit priority.  Scoped to
/// boot use; runtime callers should use [`spawn_process_from_path`].
pub unsafe fn boot_spawn_process_with_priority<R: BootRuntime>(
    name: &str,
    arg: StartupArg,
    priority: crate::task::TaskPriority,
) -> Option<TaskId> {
    let rt = crate::runtime::<R>();
    let current_cpu = crate::sched::current_cpu_index::<R>();
    let modules = rt.modules();
    let basename = name.rsplit('/').next().unwrap_or(name);
    let module = modules
        .iter()
        .find(|m| m.name == basename)
        .or_else(|| {
            modules.iter().find(|m| m.name.rsplit('/').next().unwrap_or(m.name) == basename)
        })
        .or_else(|| modules.iter().find(|m| m.name.contains(basename)))?;

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info, regions, aux_info) =
        crate::task::loader::load_module(rt, aspace, module)?;
    entry.arg0 = arg.to_raw();

    let _irq = rt.irq_disable();

    let affinity = if name.contains("virtio_sound") || name.contains("chime") {
        crate::task::Affinity::Pinned(0)
    } else if name == "bloom" {
        if rt.cpu_total_count() > 1 {
            crate::task::Affinity::Pinned(1)
        } else {
            crate::task::Affinity::Any
        }
    } else if name.contains("/sh") {
        if rt.cpu_total_count() > 1 {
            crate::task::Affinity::Pinned(rt.cpu_total_count() - 1)
        } else {
            crate::task::Affinity::Any
        }
    } else {
        crate::task::Affinity::Any
    };

    // Phase 1: minimal SCHEDULER critical section — allocate TID and register
    // scheduler-internal state. REGISTRY insertion is deferred until unlock. All
    // post-spawn setup (process_info, name, FDs) happens outside the lock to
    // avoid long SCHEDULER hold times under SMP contention.
    let (id, deferred_registry_inserts) = {
        let lock = SCHEDULER.lock();
        crate::sched::set_sched_lock_tracking::<R>(current_cpu);
        let ptr = lock.expect("Scheduler not initialized");
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let id = sched
            .spawn_user_task_deferred(entry, aspace, stack_info, regions, priority, affinity)?;
        let deferred_registry_inserts = sched.drain_pending_registry_inserts();
        crate::sched::clear_sched_lock_tracking::<R>();
        drop(lock);
        (id, deferred_registry_inserts)
    };
    crate::sched::apply_deferred_registry_inserts::<R>(deferred_registry_inserts);

    // Phase 2: post-spawn setup — REGISTRY lock only, no SCHEDULER held.
    // The task is Blocked and cannot be scheduled until wake_task(id) is called.

    // Determine parent PID from the current task's ProcessInfo.
    let ppid = {
        let tid = rt.current_tid();
        crate::task::registry::get_task::<R>(tid)
            .and_then(|t| t.process_info.clone())
            .map(|pi| pi.lock().pid)
            .unwrap_or(0)
    };

    // Retrieve the mappings Arc from the task that was just created so the
    // Process and Thread share the same underlying MappingList.
    let task_mappings =
        crate::task::registry::get_task::<R>(id).map(|t| t.mappings.clone()).unwrap_or_else(|| {
            alloc::sync::Arc::new(spin::Mutex::new(crate::memory::mappings::MappingList::new()))
        });

    // Derive the process-owned address-space token (raw u64) from the handle.
    let aspace_raw = rt.tasking().aspace_to_raw(aspace);

    // Create per-process identity.
    let pinfo = inherit_process_info::<R>(
        id as u32,
        ppid,
        crate::task::ProcessAddressSpace::from_parts(task_mappings, aspace_raw),
    );
    {
        let page_size = rt.page_size() as u64;
        let mut lock = pinfo.lock();
        lock.unix_compat.set_spawn_context(
            alloc::vec![module.name.as_bytes().to_vec()],
            crate::task::exec::build_auxv(&aux_info, page_size),
        );
        lock.exec_path = alloc::format!("/boot/{}", module.name);
    }

    // Store name, process_info, and initial TLS thread pointer on the task struct.
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
        let bytes = module.name.as_bytes();
        let len = bytes.len().min(32);
        task.name[..len].copy_from_slice(&bytes[..len]);
        task.name_len = len as u8;
        task.process_info = Some(pinfo);
        // Apply initial TLS base (FS_BASE on x86_64) for the new process's main thread.
        // Zero means no PT_TLS segment was found; FS_BASE starts at its default state.
        task.user_fs_base = aux_info.tls_tp;
    }
    crate::kdebug!("SCHED: TID {} → task '{}' (pid={} from boot module)", id, module.name, id);

    // Phase 3: make the task runnable.  wake_task acquires SCHEDULER briefly
    // to transition Blocked → Runnable and enqueue the task.  During bringup
    // the nudge IPI is harmless; wake_task handles it correctly.
    crate::sched::blocking::wake_task::<R>(id);

    rt.irq_restore(_irq);
    Some(id)
}

/// Boot-only helper: enhanced process spawn from a boot module with explicit
/// argv, env, and stdio piping.
///
/// This function looks up the executable in the boot module table.  It is
/// scoped to boot/module-launch use only.  For runtime process creation use
/// [`spawn_process_from_path`] instead.
///
/// # Safety
/// Must be called with scheduler lock expectations satisfied.
pub unsafe fn boot_spawn_process_ex<R: BootRuntime>(
    name: &str,
    argv: Vec<Vec<u8>>,
    env: BTreeMap<Vec<u8>, Vec<u8>>,
    stdin_spec: StdioSpec,
    stdout_spec: StdioSpec,
    stderr_spec: StdioSpec,
    boot_arg: u64,
    _inherited_handles: Vec<u64>,
    cwd: Option<alloc::string::String>,
    fd_remap: Vec<abi::types::HandleRemap>,
) -> Result<SpawnExResult, abi::errors::Errno> {
    let rt = crate::runtime::<R>();
    let current_cpu = crate::sched::current_cpu_index::<R>();
    let modules = rt.modules();
    let module = modules
        .iter()
        .find(|m| boot_module_matches(name, m.name))
        .ok_or(abi::errors::Errno::ENOENT)?;

    let aspace = rt.tasking().make_user_address_space();

    let (mut entry, stack_info, regions, aux_info) =
        crate::task::loader::load_module(rt, aspace, module).ok_or(abi::errors::Errno::ENOEXEC)?;
    entry.arg0 = boot_arg as usize;

    let _irq = rt.irq_disable();

    // Phase 1: minimal SCHEDULER critical section — allocate TID and register
    // scheduler-internal state. REGISTRY insertion is deferred until unlock.
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
    crate::sched::apply_deferred_registry_inserts::<R>(deferred_registry_inserts);

    // Phase 2: post-spawn setup — REGISTRY lock only, no SCHEDULER held.
    // The task is Blocked and cannot be scheduled until wake_task(id) is called.

    // Determine parent PID.
    let ppid = {
        let tid = rt.current_tid();
        crate::task::registry::get_task::<R>(tid)
            .and_then(|t| t.process_info.clone())
            .map(|pi| pi.lock().pid)
            .unwrap_or(0)
    };

    // Use provided argv, or fall back to module name
    let final_argv =
        if argv.is_empty() { alloc::vec![module.name.as_bytes().to_vec()] } else { argv };

    // Populate stdio fds in the child's handle_table.
    let tid = crate::runtime::<R>().current_tid();
    let parent_pinfo =
        crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());

    let mut handle_table = if let Some(parent_pi) = &parent_pinfo {
        parent_pi.lock().handle_table.clone()
    } else {
        crate::vfs::handle_table::HandleTable::new()
    };

    // Clear stdio slots inherited from parent so setup_stdio_fds's insert_at
    // calls don't fail silently with EBADF (slot already occupied).
    let _ = handle_table.close(0);
    let _ = handle_table.close(1);
    let _ = handle_table.close(2);

    let (stdin_pipe_id, stdout_pipe_id, stderr_pipe_id) =
        setup_stdio_fds::<R>(&mut handle_table, stdin_spec, stdout_spec, stderr_spec);

    // Step 6b: Apply explicit FD remappings.
    // These take precedence over stdio/inherited defaults for the same slots.
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

    // Open the parent-side pipe ends in the parent's fd table so the parent
    // can communicate with the child via normal file descriptors.
    //
    // For stdin PIPE:  parent holds the WRITE end (fd is returned as stdin_pipe).
    // For stdout PIPE: parent holds the READ end (fd is returned as stdout_pipe).
    // For stderr PIPE: parent holds the READ end (fd is returned as stderr_pipe).
    //
    // If there is no parent process, or an end cannot be opened, the pipe will
    // still work from the child's side (it will see EOF when the write end is
    // never written to / the read end is never read from).
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

    // Retrieve the mappings Arc from the task so Process and Thread share the
    // same underlying MappingList.
    let task_mappings =
        crate::task::registry::get_task::<R>(id).map(|t| t.mappings.clone()).unwrap_or_else(|| {
            alloc::sync::Arc::new(spin::Mutex::new(crate::memory::mappings::MappingList::new()))
        });

    // Derive the process-owned address-space token (raw u64) from the handle.
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

    // Create per-process identity with provided argv & env
    let pinfo = alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
        pid: id as u32,
        job: crate::task::ProcessLifecycle::new(ppid, id),
        unix_compat,
        handle_table,
        ipc_table: crate::ipc::IpcHandleTable::new(),
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
        exec_path: alloc::format!("/boot/{}", module.name),
        authority,
        space: crate::task::ProcessAddressSpace::from_parts(task_mappings, aspace_raw),
        service_loop: None,
    }));

    // Store name, process_info, and initial TLS thread pointer on the task struct.
    if let Some(mut task) = crate::task::registry::get_task_mut::<R>(id) {
        let bytes = module.name.as_bytes();
        let len = bytes.len().min(32);
        task.name[..len].copy_from_slice(&bytes[..len]);
        task.name_len = len as u8;
        task.process_info = Some(pinfo);
        // Apply initial TLS base (FS_BASE on x86_64) for the new process's main thread.
        task.user_fs_base = aux_info.tls_tp;
    }
    crate::kdebug!("SCHED: TID {} → task '{}' (pid={} from boot module)", id, module.name, id);

    // Phase 3: make the task runnable.  wake_task acquires SCHEDULER briefly
    // to transition Blocked → Runnable and enqueue the task.
    crate::sched::blocking::wake_task::<R>(id);

    rt.irq_restore(_irq);

    Ok(SpawnExResult {
        child_tid: id,
        child_pid: id as u32,
        stdin_pipe: parent_stdin_fd,
        stdout_pipe: parent_stdout_fd,
        stderr_pipe: parent_stderr_fd,
    })
}
