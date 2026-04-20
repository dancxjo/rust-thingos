#![no_std]
#![no_main]
extern crate alloc;

use abi::syscall::{PollHandle, poll_flags};
use stem::syscall::message::{KindId, msg_inbox_open_self, msg_send};
use stem::syscall::vfs::*;

#[stem::main]
fn main(_arg: usize) -> ! {
    stem::println!("--- poll_mux start ---");

    // 1. Create a pipe
    let mut pipefds = [0u32; 2];
    pipe(&mut pipefds).expect("pipe failed");
    let (pr, pw) = (pipefds[0], pipefds[1]);
    stem::println!("Pipe created: read={}, write={}", pr, pw);

    // 2. Create a port and bridge it to a VFS fd
    let (c_write, c_read) = stem::syscall::port_create(1024).expect("port create failed");
    let c_read_fd = vfs_handle_from_port(c_read).expect("vfs_handle_from_port failed");
    stem::println!("Port created: write={}, read={}, bridged_fd={}", c_write, c_read, c_read_fd);

    // 3. Open a regular device file (/dev/null is always present).
    //    Regular VFS files report POLLIN|POLLOUT immediately (they are
    //    always-ready, matching POSIX semantics for non-socket file descriptors).
    let dev_null_fd =
        vfs_open("/dev/null", abi::syscall::vfs_flags::O_RDWR).expect("open /dev/null failed");
    stem::println!("VFS file opened: fd={}", dev_null_fd);

    // 4. Open inbox FD through procfs path-open model.
    let inbox_fd = msg_inbox_open_self().expect("open /proc/self/inbox failed");
    stem::println!("Inbox opened: fd={}", inbox_fd);

    // 5. Test timeout — pipe, port, and inbox are all empty so poll should expire.
    let mut fds = [
        PollHandle { handle: pr as i32, events: poll_flags::POLLIN, revents: 0 },
        PollHandle { handle: c_read_fd as i32, events: poll_flags::POLLIN, revents: 0 },
        PollHandle { handle: inbox_fd as i32, events: poll_flags::POLLIN, revents: 0 },
    ];
    stem::println!("Polling pipe+port+inbox for 100ms (should timeout)...");
    let start = stem::syscall::monotonic_ns();
    let n = vfs_poll(&mut fds, 100).expect("poll failed");
    let end = stem::syscall::monotonic_ns();
    stem::println!("Poll returned {} entries, took {} ms", n, (end - start) / 1_000_000);
    assert!(n == 0, "Expected timeout, got {}", n);

    // 6. VFS regular files are always POLLIN-ready.
    let mut vfs_fds = [PollHandle {
        handle: dev_null_fd as i32,
        events: poll_flags::POLLIN | poll_flags::POLLOUT,
        revents: 0,
    }];
    stem::println!("Polling VFS file (should be immediately ready)...");
    let n = vfs_poll(&mut vfs_fds, 0).expect("poll vfs failed");
    stem::println!("Poll returned {} entries", n);
    assert!(n == 1, "Expected 1 ready entry for VFS file, got {}", n);
    assert!(vfs_fds[0].revents & poll_flags::POLLIN != 0, "Expected POLLIN on VFS file");
    assert!(vfs_fds[0].revents & poll_flags::POLLOUT != 0, "Expected POLLOUT on VFS file");

    // 7. Test pipe readiness (write before poll)
    vfs_write(pw, b"hello").expect("write to pipe failed");
    stem::println!("Wrote to pipe, polling now...");
    fds[0].revents = 0;
    fds[1].revents = 0;
    fds[2].revents = 0;
    let n = vfs_poll(&mut fds, 100).expect("poll failed");
    stem::println!("Poll returned {} entries", n);
    assert!(n == 1, "Expected 1 ready entry, got {}", n);
    assert!(fds[0].revents & poll_flags::POLLIN != 0, "Expected POLLIN on pipe");

    // 8. Test port readiness (write before poll)
    stem::syscall::port_send(c_write, b"world").expect("send to port failed");
    stem::println!("Sent to port, polling now...");
    fds[0].revents = 0;
    fds[1].revents = 0;
    fds[2].revents = 0;
    let n = vfs_poll(&mut fds, 100).expect("poll failed");
    stem::println!("Poll returned {} entries", n);
    // Both should be ready now if order is preserved
    assert!(fds[0].revents & poll_flags::POLLIN != 0, "Expected POLLIN on pipe");
    assert!(fds[1].revents & poll_flags::POLLIN != 0, "Expected POLLIN on port");

    // 9. Add one message to our own inbox and verify inbox FD readiness.
    let inbox_test_kind = KindId([0u8; 16]);
    msg_send(stem::syscall::getpid(), inbox_test_kind, b"inbox").expect("msg_send self failed");

    // 10. Mixed poll: pipe + port + inbox + VFS file all at once.
    //     Pipe (index 0), port (index 1), and inbox (index 2) have unread data;
    //     VFS file (index 3) is always ready.  All four should fire.
    stem::println!("Mixed poll: pipe + port + inbox + VFS file...");
    let mut mixed = [
        PollHandle { handle: pr as i32, events: poll_flags::POLLIN, revents: 0 },
        PollHandle { handle: c_read_fd as i32, events: poll_flags::POLLIN, revents: 0 },
        PollHandle { handle: inbox_fd as i32, events: poll_flags::POLLIN, revents: 0 },
        PollHandle {
            handle: dev_null_fd as i32,
            events: poll_flags::POLLIN | poll_flags::POLLOUT,
            revents: 0,
        },
    ];
    let n = vfs_poll(&mut mixed, 0).expect("mixed poll failed");
    stem::println!("Mixed poll returned {} entries", n);
    assert!(n == 4, "Expected all 4 fds ready (pipe + port + inbox + VFS file), got {}", n);
    assert!(mixed[0].revents & poll_flags::POLLIN != 0, "Expected POLLIN on pipe in mixed");
    assert!(mixed[1].revents & poll_flags::POLLIN != 0, "Expected POLLIN on port in mixed");
    assert!(mixed[2].revents & poll_flags::POLLIN != 0, "Expected POLLIN on inbox in mixed");
    assert!(
        mixed[3].revents & (poll_flags::POLLIN | poll_flags::POLLOUT) != 0,
        "Expected readiness on VFS file in mixed"
    );

    stem::println!("--- poll_mux success ---");
    stem::syscall::exit(0)
}
