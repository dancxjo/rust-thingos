use std::fs;
use std::path::Path;

use abi::syscall;

#[test]
fn process_model_is_spawn_exec_without_fork_syscall() {
    assert_eq!(syscall::SYS_SPAWN_THREAD, 0x1004);
    assert_eq!(syscall::SYS_SPAWN_PROCESS, 0x1005);
    assert_eq!(syscall::SYS_SPAWN_PROCESS_EX, 0x1006);
    assert_eq!(syscall::SYS_TASK_EXEC, 0x100D);
    assert_eq!(syscall::SYS_WAITPID, 0x1011);

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let numbers = fs::read_to_string(repo_root.join("thingos/abi/src/numbers.rs"))
        .expect("read syscall number source");
    assert!(
        !numbers.contains("SYS_FORK"),
        "Thing-OS process creation is spawn+exec; do not add SYS_FORK"
    );
}

#[test]
fn device_primitives_stay_in_the_userland_driver_range() {
    let device_syscalls = [
        syscall::SYS_DEVICE_CLAIM,
        syscall::SYS_DEVICE_CALL,
        syscall::SYS_DEVICE_MAP_MMIO,
        syscall::SYS_DEVICE_ALLOC_DMA,
        syscall::SYS_DEVICE_DMA_PHYS,
        syscall::SYS_DEVICE_IOPORT_READ,
        syscall::SYS_DEVICE_IOPORT_WRITE,
        syscall::SYS_DEVICE_IRQ_SUBSCRIBE,
        syscall::SYS_DEVICE_IRQ_WAIT,
    ];

    assert_eq!(
        device_syscalls,
        [0x5000, 0x5001, 0x5002, 0x5003, 0x5004, 0x5005, 0x5006, 0x5007, 0x5008]
    );
}

#[test]
fn vfs_surface_stays_in_the_fs_range() {
    let fs_syscalls = [
        syscall::SYS_FS_OPEN,
        syscall::SYS_FS_CLOSE,
        syscall::SYS_FS_READ,
        syscall::SYS_FS_WRITE,
        syscall::SYS_FS_MOUNT,
        syscall::SYS_FS_POLL,
        syscall::SYS_FS_WATCH_PATH,
        syscall::SYS_FS_DEVICE_CALL,
        syscall::SYS_FS_MOUNT_EX,
        syscall::SYS_FS_BIND,
    ];

    for number in fs_syscalls {
        assert!(
            (0x4000..0x5000).contains(&number),
            "VFS syscall {number:#x} escaped the SYS_FS range"
        );
    }
}
