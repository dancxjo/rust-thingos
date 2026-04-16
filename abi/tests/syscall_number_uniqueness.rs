#[cfg(test)]
mod tests {
    use abi::syscall::*;
    use std::collections::BTreeSet;

    #[test]
    fn fs_syscall_numbers_are_unique() {
        let fs_syscalls = [
            SYS_FS_OPEN,
            SYS_FS_CLOSE,
            SYS_FS_READ,
            SYS_FS_WRITE,
            SYS_FS_SEEK,
            SYS_FS_STAT,
            SYS_FS_READDIR,
            SYS_FS_MKDIR,
            SYS_FS_UNLINK,
            SYS_FS_MOUNT,
            SYS_FS_UMOUNT,
            SYS_FS_POLL,
            SYS_FS_DUP,
            SYS_FS_DUP2,
            SYS_FS_WATCH_THING,
            SYS_FS_WATCH_PATH,
            SYS_FS_RENAME,
            SYS_FS_DEVICE_CALL,
            SYS_FS_CHDIR,
            SYS_FS_GETCWD,
            SYS_THING_FROM_CHANNEL,
            SYS_FS_NOTIFY,
            SYS_FS_REALPATH,
            SYS_FS_SYNC,
            SYS_FS_FCNTL,
            SYS_FS_SYMLINK,
            SYS_FS_READLINK,
            SYS_FS_FTRUNCATE,
            SYS_FS_CHMOD,
            SYS_FS_FCHMOD,
            SYS_FS_UTIMES,
            SYS_FS_FUTIMES,
            SYS_FS_LSTAT,
            SYS_FS_READV,
            SYS_FS_WRITEV,
            SYS_FS_LINK,
            SYS_FS_FLOCK,
            SYS_FS_LUTIMES,
            SYS_FS_ISATTY,
        ];
        let mut seen = BTreeSet::new();
        for number in fs_syscalls {
            assert!(seen.insert(number), "duplicate FS syscall number: 0x{number:04X}");
        }
    }

    #[test]
    fn fs_isatty_and_lstat_do_not_collide() {
        assert_ne!(SYS_FS_ISATTY, SYS_FS_LSTAT);
    }
}
