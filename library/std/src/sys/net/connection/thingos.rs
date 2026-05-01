use super::each_addr;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut};
use crate::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, ToSocketAddrs};
use crate::string::String;
use crate::sync::{Arc, Mutex};
use crate::time::Duration;
use crate::{fmt, thread, vec};

use crate::sys::pal::raw_syscall6;

use abi::syscall::*;

const O_RDONLY: u32 = 0x0000;
const O_WRONLY: u32 = 0x0001;
const O_RDWR: u32 = 0x0002;
const O_NONBLOCK: u32 = 0x0800;

const EAGAIN: i32 = 11;
const EINVAL: i32 = 22;
const EPIPE: i32 = 32;
const EAFNOSUPPORT: i32 = 97;
const ETIMEDOUT: i32 = 110;
const ECONNREFUSED: i32 = 111;
const ENOTSUP: i32 = 95;

const CONNECT_POLL_NS: u64 = 5_000_000;
const IO_POLL_NS: u64 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TcpState {
    Created,
    Bound,
    Connected,
    Closed,
    Other,
}

#[derive(Debug, Clone, Copy)]
struct StatusInfo {
    state: TcpState,
    local: Option<SocketAddr>,
    remote: Option<SocketAddr>,
}

impl Default for StatusInfo {
    fn default() -> Self {
        Self { state: TcpState::Other, local: None, remote: None }
    }
}

pub struct TcpStream {
    id: u32,
    data_fd: i32,
    ctl_fd: i32,
    peer_addr: SocketAddr,
    read_timeout: Arc<Mutex<Option<Duration>>>,
    write_timeout: Arc<Mutex<Option<Duration>>>,
    nonblocking: Arc<Mutex<bool>>,
}

impl TcpStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        each_addr(addr, |addr| Self::connect_inner(addr, None))
    }

    pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
        if timeout.is_zero() {
            return Err(io::const_error!(
                io::ErrorKind::InvalidInput,
                "cannot set a 0 duration timeout"
            ));
        }
        Self::connect_inner(addr, Some(timeout))
    }

    fn connect_inner(addr: &SocketAddr, timeout: Option<Duration>) -> io::Result<TcpStream> {
        let SocketAddr::V4(peer_v4) = addr else {
            return Err(io::Error::from_raw_os_error(EAFNOSUPPORT));
        };

        let id = allocate_tcp_socket()?;
        let ctl_path = socket_path(id, "ctl");
        let data_path = socket_path(id, "data");

        let ctl_fd = match vfs_open(&ctl_path, O_WRONLY) {
            Ok(fd) => fd,
            Err(err) => return Err(err),
        };
        let data_fd = match vfs_open(&data_path, O_RDWR | O_NONBLOCK) {
            Ok(fd) => fd,
            Err(err) => {
                let _ = vfs_close(ctl_fd);
                return Err(err);
            }
        };

        let stream = TcpStream {
            id,
            data_fd,
            ctl_fd,
            peer_addr: SocketAddr::V4(*peer_v4),
            read_timeout: Arc::new(Mutex::new(None)),
            write_timeout: Arc::new(Mutex::new(None)),
            nonblocking: Arc::new(Mutex::new(false)),
        };

        let peer_ip = peer_v4.ip().octets();
        let cmd = format!(
            "connect {}.{}.{}.{} {}",
            peer_ip[0],
            peer_ip[1],
            peer_ip[2],
            peer_ip[3],
            peer_v4.port()
        );
        if let Err(err) = vfs_write(stream.ctl_fd, cmd.as_bytes()) {
            return Err(err);
        }

        if let Err(err) = stream.wait_for_connected(timeout) {
            let _ = vfs_write(stream.ctl_fd, b"close");
            return Err(err);
        }

        Ok(stream)
    }

    pub fn set_read_timeout(&self, t: Option<Duration>) -> io::Result<()> {
        self.read_timeout.set(t).unwrap();
        Ok(())
    }

    pub fn set_write_timeout(&self, t: Option<Duration>) -> io::Result<()> {
        self.write_timeout.set(t).unwrap();
        Ok(())
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(self.read_timeout.get_cloned().unwrap())
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(self.write_timeout.get_cloned().unwrap())
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let nonblocking = self.nonblocking.get_cloned().unwrap();
        let timeout = self.read_timeout.get_cloned().unwrap();
        let deadline = timeout.map(|dur| monotonic_ns().saturating_add(duration_to_ns(dur)));

        loop {
            match vfs_read(self.data_fd, buf) {
                Ok(n) => return Ok(n),
                Err(err) if err.raw_os_error() == Some(EAGAIN) && nonblocking => {
                    return Err(io::Error::from_raw_os_error(EAGAIN));
                }
                Err(err) if err.raw_os_error() == Some(EAGAIN) => {
                    if deadline_expired(deadline) {
                        return Err(io::Error::from_raw_os_error(ETIMEDOUT));
                    }
                    sleep_ns(IO_POLL_NS);
                    thread::yield_now();
                }
                Err(err) => return Err(err),
            }
        }
    }

    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        crate::io::default_read_buf(|buf| self.read(buf), cursor)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        crate::io::default_read_vectored(|buf| self.read(buf), bufs)
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        let nonblocking = self.nonblocking.get_cloned().unwrap();
        let timeout = self.write_timeout.get_cloned().unwrap();
        let deadline = timeout.map(|dur| monotonic_ns().saturating_add(duration_to_ns(dur)));

        loop {
            match vfs_write(self.data_fd, buf) {
                Ok(n) if n > 0 => return Ok(n),
                Ok(_) if nonblocking => return Err(io::Error::from_raw_os_error(EAGAIN)),
                Ok(_) => {
                    if self.status_snapshot().state == TcpState::Closed {
                        return Err(io::Error::from_raw_os_error(EPIPE));
                    }
                    if deadline_expired(deadline) {
                        return Err(io::Error::from_raw_os_error(ETIMEDOUT));
                    }
                    sleep_ns(IO_POLL_NS);
                    thread::yield_now();
                }
                Err(err) => return Err(err),
            }
        }
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        crate::io::default_write_vectored(|buf| self.write(buf), bufs)
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.peer_addr)
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        if let Some(local) = self.status_snapshot().local {
            return Ok(local);
        }
        Err(io::Error::from_raw_os_error(EINVAL))
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        match how {
            Shutdown::Both => vfs_write(self.ctl_fd, b"close").map(|_| ()),
            Shutdown::Read | Shutdown::Write => Err(io::Error::from_raw_os_error(ENOTSUP)),
        }
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        Ok(())
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        Ok(true)
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn ttl(&self) -> io::Result<u32> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.nonblocking.set(nonblocking).unwrap();
        Ok(())
    }

    fn wait_for_connected(&self, timeout: Option<Duration>) -> io::Result<()> {
        let deadline = timeout.map(|dur| monotonic_ns().saturating_add(duration_to_ns(dur)));
        loop {
            let status = self.status_snapshot();
            match status.state {
                TcpState::Connected => return Ok(()),
                TcpState::Closed => return Err(io::Error::from_raw_os_error(ECONNREFUSED)),
                TcpState::Created | TcpState::Bound | TcpState::Other => {
                    if deadline_expired(deadline) {
                        return Err(io::Error::from_raw_os_error(ETIMEDOUT));
                    }
                    sleep_ns(CONNECT_POLL_NS);
                }
            }
        }
    }

    fn status_snapshot(&self) -> StatusInfo {
        read_status(self.id).unwrap_or_default()
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        let _ = vfs_write(self.ctl_fd, b"close");
        let _ = vfs_close(self.data_fd);
        let _ = vfs_close(self.ctl_fd);
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = self.status_snapshot();
        f.debug_struct("TcpStream")
            .field("id", &self.id)
            .field("peer", &status.remote.unwrap_or(self.peer_addr))
            .field("local", &status.local)
            .finish()
    }
}

pub struct TcpListener(!);

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(_: A) -> io::Result<TcpListener> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct UdpSocket(!);

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(_: A) -> io::Result<UdpSocket> {
        Err(io::Error::from_raw_os_error(ENOTSUP))
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        self.0
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn connect<A: ToSocketAddrs>(&self, _: A) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub type LookupHost = vec::IntoIter<SocketAddr>;

pub fn lookup_host(host: &str, port: u16) -> io::Result<LookupHost> {
    if let Ok(ip) = host.parse::<Ipv4Addr>() {
        return Ok(vec![SocketAddr::new(IpAddr::V4(ip), port)].into_iter());
    }
    Err(io::Error::from_raw_os_error(ENOTSUP))
}

fn allocate_tcp_socket() -> io::Result<u32> {
    let fd = vfs_open("/net/tcp/new", O_RDONLY)?;
    let mut buf = [0u8; 32];
    let n = vfs_read(fd, &mut buf)?;
    vfs_close(fd)?;
    let text =
        crate::str::from_utf8(&buf[..n]).map_err(|_| io::Error::from_raw_os_error(EINVAL))?.trim();
    text.parse::<u32>().map_err(|_| io::Error::from_raw_os_error(EINVAL))
}

fn socket_path(id: u32, subpath: &str) -> String {
    format!("/net/tcp/{id}/{subpath}")
}

fn read_status(id: u32) -> io::Result<StatusInfo> {
    let path = socket_path(id, "status");
    let fd = vfs_open(&path, O_RDONLY)?;
    let mut buf = [0u8; 256];
    let n = vfs_read(fd, &mut buf)?;
    vfs_close(fd)?;

    let text =
        crate::str::from_utf8(&buf[..n]).map_err(|_| io::Error::from_raw_os_error(EINVAL))?;

    let mut out = StatusInfo::default();
    for line in text.lines() {
        if let Some(state) = line.strip_prefix("state: ") {
            out.state = match state.trim() {
                "created" => TcpState::Created,
                "bound" | "syn-sent" | "syn-received" => TcpState::Bound,
                "connected" | "established" | "fin-wait-1" | "fin-wait-2" | "close-wait" => {
                    TcpState::Connected
                }
                "closed" | "time-wait" | "closing" | "last-ack" => TcpState::Closed,
                _ => TcpState::Other,
            };
        } else if let Some(local) = line.strip_prefix("local: ") {
            out.local = parse_socket_addr(local.trim());
        } else if let Some(remote) = line.strip_prefix("remote: ") {
            out.remote = parse_socket_addr(remote.trim());
        }
    }
    Ok(out)
}

fn parse_socket_addr(text: &str) -> Option<SocketAddr> {
    let (ip, port) = text.rsplit_once(':')?;
    let port = port.parse::<u16>().ok()?;
    let ip = ip.parse::<Ipv4Addr>().ok()?;
    Some(SocketAddr::V4(SocketAddrV4::new(ip, port)))
}

fn deadline_expired(deadline: Option<u64>) -> bool {
    match deadline {
        Some(ns) => monotonic_ns() >= ns,
        None => false,
    }
}

fn duration_to_ns(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

fn monotonic_ns() -> u64 {
    let ret = unsafe { raw_syscall6(SYS_TIME_MONOTONIC, 0, 0, 0, 0, 0, 0) };
    if ret < 0 { 0 } else { ret as u64 }
}

fn sleep_ns(ns: u64) {
    let _ = unsafe { raw_syscall6(SYS_SLEEP_NS, ns as usize, 0, 0, 0, 0, 0) };
}

fn vfs_open(path: &str, flags: u32) -> io::Result<i32> {
    let ret = unsafe {
        raw_syscall6(SYS_FS_OPEN, path.as_ptr() as usize, path.len(), flags as usize, 0, 0, 0)
    };
    decode_ret(ret).map(|fd| fd as i32)
}

fn vfs_close(fd: i32) -> io::Result<()> {
    let ret = unsafe { raw_syscall6(SYS_FS_CLOSE, fd as usize, 0, 0, 0, 0, 0) };
    decode_ret(ret).map(|_| ())
}

fn vfs_read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    let ret = unsafe {
        raw_syscall6(SYS_FS_READ, fd as usize, buf.as_mut_ptr() as usize, buf.len(), 0, 0, 0)
    };
    decode_ret(ret)
}

fn vfs_write(fd: i32, buf: &[u8]) -> io::Result<usize> {
    let ret = unsafe {
        raw_syscall6(SYS_FS_WRITE, fd as usize, buf.as_ptr() as usize, buf.len(), 0, 0, 0)
    };
    decode_ret(ret)
}

fn decode_ret(ret: isize) -> io::Result<usize> {
    if ret < 0 && ret >= -4096 {
        Err(io::Error::from_raw_os_error((-ret) as i32))
    } else {
        Ok(ret as usize)
    }
}

