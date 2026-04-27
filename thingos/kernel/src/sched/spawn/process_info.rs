use crate::BootRuntime;
use crate::sched::types::Scheduler;
use crate::task::{ProcessInfo, TaskId};

pub(super) fn boot_module_matches(name: &str, module_name: &str) -> bool {
    if module_name == name {
        return true;
    }

    module_name.rsplit('/').next().unwrap_or(module_name) == name.rsplit('/').next().unwrap_or(name)
}

pub(super) fn current_parent_pid<R: BootRuntime>(sched: &Scheduler<R>) -> u32 {
    // Prefer runtime current TID; this remains authoritative in syscall/trap
    // context even when scheduler per-CPU `current` can be transiently stale.
    let rt = crate::runtime::<R>();
    let runtime_tid = rt.current_tid();

    crate::task::registry::get_task::<R>(runtime_tid)
        .and_then(|t| t.process_info.clone())
        .or_else(|| {
            let cpu_idx = crate::sched::current_cpu_index::<R>();
            sched
                .state
                .per_cpu
                .get(cpu_idx)
                .and_then(|pc| pc.current)
                .and_then(|ctid| crate::task::registry::get_task::<R>(ctid))
                .and_then(|t| t.process_info.clone())
        })
        .map(|pi| pi.lock().pid)
        .unwrap_or(0)
}

pub(super) fn default_process_info(
    pid: u32,
    ppid: u32,
    space: crate::task::ProcessAddressSpace,
) -> alloc::sync::Arc<spin::Mutex<ProcessInfo>> {
    let console_node: alloc::sync::Arc<dyn crate::vfs::VfsNode> =
        alloc::sync::Arc::new(crate::vfs::devfs::ConsoleNode);
    let mut handle_table = crate::vfs::handle_table::HandleTable::new();
    let _ = handle_table.insert_at(
        0,
        console_node.clone(),
        crate::vfs::OpenFlags::read_only(),
        "/dev/console".into(),
    );
    let _ = handle_table.insert_at(
        1,
        console_node.clone(),
        crate::vfs::OpenFlags::write_only(),
        "/dev/console".into(),
    );
    let _ = handle_table.insert_at(
        2,
        console_node,
        crate::vfs::OpenFlags::write_only(),
        "/dev/console".into(),
    );
    let is_session_leader = ppid == 0;
    alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
        pid,
        job: crate::task::ProcessLifecycle::new(ppid, pid as TaskId),
        unix_compat: crate::task::ProcessUnixCompat::isolated(pid, is_session_leader),
        handle_table,
        ipc_table: crate::ipc::IpcHandleTable::new(),
        namespace: crate::vfs::NamespaceRef::global(),
        cwd: alloc::string::String::from("/"),
        root: alloc::string::String::from("/"),
        exec_path: alloc::string::String::new(),
        authority: crate::task::ProcessAuthority::root(),
        space,
        service_loop: None,
    }))
}

pub(super) fn inherit_process_info<R: BootRuntime>(
    pid: u32,
    ppid: u32,
    space: crate::task::ProcessAddressSpace,
) -> alloc::sync::Arc<spin::Mutex<ProcessInfo>> {
    let tid = crate::runtime::<R>().current_tid();
    let current_pinfo =
        crate::task::registry::get_task::<R>(tid).and_then(|t| t.process_info.clone());

    if let Some(parent_pi) = current_pinfo {
        let parent = parent_pi.lock();
        // Create the first-class Space object from the ProcessAddressSpace fields.
        alloc::sync::Arc::new(spin::Mutex::new(ProcessInfo {
            pid,
            job: crate::task::ProcessLifecycle::new(ppid, pid as TaskId),
            unix_compat: crate::task::ProcessUnixCompat::inherit(&parent.unix_compat),
            handle_table: parent.handle_table.clone(),
            ipc_table: parent.ipc_table.clone(),
            namespace: parent.namespace.clone(),
            cwd: parent.cwd.clone(),
            root: parent.root.clone(),
            exec_path: alloc::string::String::new(),
            authority: crate::task::ProcessAuthority::inherit(parent.authority),
            space,
            service_loop: None,
        }))
    } else {
        default_process_info(pid, ppid, space)
    }
}
