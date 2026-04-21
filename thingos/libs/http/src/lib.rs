#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Write};

use abi::syscall::{PollHandle, poll_flags};
use embedded_io::ErrorKind;
use embedded_tls::blocking::{
    Aes128GcmSha256, Aes256GcmSha384, TlsConfig, TlsConnection, TlsContext, UnsecureProvider,
};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use stem::syscall::port::{PortHandle, port_close, port_create, port_recv, port_send_all};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_poll, vfs_read, vfs_write};
use stem::thread::spawn_task_detached;
use stem::{debug, info, trace, warn};

const IO_POLL_TIMEOUT_MS: u64 = 60_000;
const CONNECT_TIMEOUT_MS: u64 = 5_000;
const HEADER_READ_TIMEOUT_MS: u64 = 5_000;
const PROVIDER_POLL_SLICE_MS: u64 = 100;
const MAX_HEADER_READ_ITERATIONS: usize = 128;
const HEADER_STAGING_CHUNK_SIZE: usize = 4_096;
const MAX_HEADER_BYTES: usize = 64 * 1024;
const HTTPS_STREAM_CHUNK_SIZE: usize = 16_384;
const HTTPS_PORT_CAPACITY_BYTES: usize = HTTPS_STREAM_CHUNK_SIZE * 2;
// Max TLS record payload + TLS overhead as recommended by embedded-tls docs.
const TLS_RECORD_READ_BUF_SIZE: usize = 16_640;
// Write-side TLS record staging buffer.
const TLS_RECORD_WRITE_BUF_SIZE: usize = 4_096;
const DEFAULT_USER_AGENT: &str = "ThingOS-httpsd/0.1 (+https://github.com/dancxjo/thingos)";

fn deadline_after_ms(timeout_ms: u64) -> u64 {
    let now = stem::syscall::monotonic_ns();
    now.saturating_add(timeout_ms.saturating_mul(1_000_000))
}

fn timeout_ms_until_deadline(deadline_ns: u64) -> Result<u64, String> {
    let now = stem::syscall::monotonic_ns();
    if now >= deadline_ns {
        return Err("timed out waiting for readiness".to_string());
    }
    let remaining_ns = deadline_ns.saturating_sub(now);
    Ok(ns_to_timeout_ms(remaining_ns))
}

fn ns_to_timeout_ms(ns: u64) -> u64 {
    ((ns.saturating_add(999_999)) / 1_000_000).max(1)
}

fn wait_fd_ready(fd: u32, events: u16, deadline_ns: u64, context: &str) -> Result<u16, String> {
    let thing = i32::try_from(fd).map_err(|_| format!("{context}: fd out of range"))?;
    loop {
        let timeout_ms = timeout_ms_until_deadline(deadline_ns)
            .map_err(|_| format!("{context}: timed out waiting for readiness"))?;
        // Userland VFS providers do not always have a precise readiness wakeup path,
        // so bound each wait and re-issue poll requests until the deadline.
        let timeout_ms = timeout_ms.min(PROVIDER_POLL_SLICE_MS);
        let mut pollfd = [PollHandle { handle: thing, events, revents: 0 }];
        match vfs_poll(&mut pollfd, timeout_ms) {
            Ok(0) => return Err(format!("{context}: timed out waiting for readiness")),
            Ok(_) => {
                if pollfd[0].revents == 0 {
                    continue;
                }
                return Ok(pollfd[0].revents);
            }
            Err(abi::errors::Errno::EINTR) => continue,
            Err(e) => return Err(format!("{context}: poll failed: {:?}", e)),
        }
    }
}

pub struct TcpStream {
    data_fd: u32,
    ctl_fd: u32,
}

impl TcpStream {
    pub fn connect(host: &str, port: u16) -> Result<Self, String> {
        use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_RDWR};

        info!("http: connect host={} port={}", host, port);

        // 1. Allocate a new TCP socket via /net/tcp/new
        info!("http: opening /net/tcp/new");
        let new_fd = vfs_open("/net/tcp/new", O_RDONLY | O_NONBLOCK)
            .map_err(|e| format!("failed to open /net/tcp/new: {:?}", e))?;
        let mut buf = [0u8; 16];
        let deadline_ns = deadline_after_ms(CONNECT_TIMEOUT_MS);
        let n = loop {
            match vfs_read(new_fd, &mut buf) {
                Ok(n) if n > 0 => break n,
                Ok(_) | Err(abi::errors::Errno::EAGAIN) => {
                    if let Err(e) = wait_fd_ready(
                        new_fd,
                        poll_flags::POLLIN,
                        deadline_ns,
                        "http connect new socket id",
                    ) {
                        let _ = vfs_close(new_fd);
                        return Err(e);
                    }
                }
                Err(e) => {
                    let _ = vfs_close(new_fd);
                    return Err(format!("failed to read socket id: {:?}", e));
                }
            }
        };
        let _ = vfs_close(new_fd);

        let socket_id =
            core::str::from_utf8(&buf[..n]).map_err(|_| "invalid socket id encoding")?.trim();
        info!("http: allocated tcp socket id={}", socket_id);

        // 2. Open ctl and data files
        let ctl_path = format!("/net/tcp/{}/ctl", socket_id);
        let data_path = format!("/net/tcp/{}/data", socket_id);

        info!("http: opening ctl path {}", ctl_path);
        let ctl_fd =
            vfs_open(&ctl_path, O_RDWR).map_err(|e| format!("failed to open ctl: {:?}", e))?;
        info!("http: opening data path {}", data_path);
        let data_fd = match vfs_open(&data_path, O_RDWR | O_NONBLOCK) {
            Ok(fd) => fd,
            Err(e) => {
                let _ = vfs_close(ctl_fd);
                return Err(format!("failed to open data: {:?}", e));
            }
        };

        // 3. Connect via ctl file
        let conn_cmd = format!("connect {} {}", host, port);
        info!("http: issuing connect command: {}", conn_cmd.trim());
        if let Err(e) = vfs_write(ctl_fd, conn_cmd.as_bytes()) {
            let _ = vfs_close(data_fd);
            let _ = vfs_close(ctl_fd);
            return Err(format!("connect command failed: {:?}", e));
        }

        info!("http: waiting for socket readiness...");
        let revents = wait_fd_ready(
            data_fd,
            poll_flags::POLLOUT | poll_flags::POLLIN,
            deadline_after_ms(CONNECT_TIMEOUT_MS),
            "http connect socket",
        )?;
        if (revents & (poll_flags::POLLERR | poll_flags::POLLNVAL) != 0)
            || ((revents & poll_flags::POLLHUP != 0) && (revents & poll_flags::POLLOUT == 0))
        {
            let _ = vfs_close(data_fd);
            let _ = vfs_close(ctl_fd);
            return Err(format!("connect failed: revents=0x{:x}", revents));
        }

        debug!("http: connect command issued for socket id={}", socket_id);

        Ok(Self { data_fd, ctl_fd })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        debug!("http: write {} bytes", data.len());
        let deadline_ns = deadline_after_ms(IO_POLL_TIMEOUT_MS);
        loop {
            match vfs_write(self.data_fd, data) {
                Ok(n) => {
                    if n > 0 {
                        return Ok(n);
                    }
                    let revents = wait_fd_ready(
                        self.data_fd,
                        poll_flags::POLLOUT | poll_flags::POLLIN,
                        deadline_ns,
                        "http write",
                    )?;
                    if (revents & (poll_flags::POLLERR | poll_flags::POLLNVAL) != 0)
                        || ((revents & poll_flags::POLLHUP != 0)
                            && (revents & poll_flags::POLLOUT == 0))
                    {
                        return Err(format!("write readiness failed: revents=0x{:x}", revents));
                    }
                }
                Err(abi::errors::Errno::EAGAIN) => {
                    let revents = wait_fd_ready(
                        self.data_fd,
                        poll_flags::POLLOUT | poll_flags::POLLIN,
                        deadline_ns,
                        "http write",
                    )?;
                    if (revents & (poll_flags::POLLERR | poll_flags::POLLNVAL) != 0)
                        || ((revents & poll_flags::POLLHUP != 0)
                            && (revents & poll_flags::POLLOUT == 0))
                    {
                        return Err(format!("write readiness failed: revents=0x{:x}", revents));
                    }
                }
                Err(e) => return Err(format!("write failed: {:?}", e)),
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        let deadline_ns = deadline_after_ms(IO_POLL_TIMEOUT_MS);
        loop {
            match vfs_read(self.data_fd, buf) {
                Ok(n) => {
                    if n > 0 {
                        return Ok(n);
                    }
                    let revents = wait_fd_ready(
                        self.data_fd,
                        poll_flags::POLLIN | poll_flags::POLLHUP,
                        deadline_ns,
                        "http read",
                    )?;
                    if revents & (poll_flags::POLLERR | poll_flags::POLLNVAL) != 0 {
                        return Err(format!("read readiness failed: revents=0x{:x}", revents));
                    }
                    if (revents & poll_flags::POLLHUP != 0) && (revents & poll_flags::POLLIN == 0) {
                        return Ok(0);
                    }
                }
                Err(abi::errors::Errno::EAGAIN) => {
                    let revents = wait_fd_ready(
                        self.data_fd,
                        poll_flags::POLLIN | poll_flags::POLLHUP,
                        deadline_ns,
                        "http read",
                    )?;
                    if revents & (poll_flags::POLLERR | poll_flags::POLLNVAL) != 0 {
                        return Err(format!("read readiness failed: revents=0x{:x}", revents));
                    }
                    if (revents & poll_flags::POLLHUP != 0) && (revents & poll_flags::POLLIN == 0) {
                        return Ok(0);
                    }
                }
                Err(e) => return Err(format!("read failed: {:?}", e)),
            }
        }
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        let _ = vfs_close(self.data_fd);
        let _ = vfs_close(self.ctl_fd);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UrlScheme {
    Http,
    Https,
}

struct ParsedUrl {
    scheme: UrlScheme,
    host: String,
    port: u16,
    path: String,
}

fn parse_url(url: &str) -> Result<ParsedUrl, String> {
    let (scheme, rest, default_port) = if let Some(rest) = url.strip_prefix("http://") {
        (UrlScheme::Http, rest, 80)
    } else if let Some(rest) = url.strip_prefix("https://") {
        (UrlScheme::Https, rest, 443)
    } else {
        return Err("unsupported URL scheme".to_string());
    };

    let (host_port, path) =
        if let Some(idx) = rest.find('/') { (&rest[..idx], &rest[idx..]) } else { (rest, "/") };

    let (host, port) = if let Some(idx) = host_port.find(':') {
        (
            &host_port[..idx],
            host_port[idx + 1..].parse::<u16>().map_err(|_| "invalid port".to_string())?,
        )
    } else {
        (host_port, default_port)
    };

    Ok(ParsedUrl { scheme, host: host.to_string(), port, path: path.to_string() })
}

fn build_request(
    method: &str,
    host: &str,
    port: u16,
    path: &str,
    body: Option<&str>,
    scheme_default_port: u16,
) -> String {
    let mut req = String::new();
    write!(req, "{} {} HTTP/1.1\r\n", method, path).ok();
    if port != scheme_default_port {
        write!(req, "Host: {}:{}\r\n", host, port).ok();
    } else {
        write!(req, "Host: {}\r\n", host).ok();
    }
    write!(req, "User-Agent: {}\r\n", DEFAULT_USER_AGENT).ok();
    write!(req, "Connection: close\r\n").ok();
    if let Some(b) = body {
        write!(req, "Content-Length: {}\r\n", b.len()).ok();
        write!(req, "Content-Type: application/json\r\n").ok();
    }
    write!(req, "\r\n").ok();
    if let Some(b) = body {
        req.push_str(b);
    }
    req
}

fn build_tls_seed() -> Result<[u8; 32], String> {
    let mut seed = [0u8; 32];
    stem::syscall::getrandom(&mut seed).map_err(|e| format!("getrandom failed: {:?}", e))?;
    Ok(seed)
}

#[derive(Debug, Clone, Copy)]
struct TcpTransportError(ErrorKind);

impl fmt::Display for TcpTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl core::error::Error for TcpTransportError {}

impl embedded_io::Error for TcpTransportError {
    fn kind(&self) -> ErrorKind {
        self.0
    }
}

struct TcpTransport {
    inner: TcpStream,
}

impl embedded_io::ErrorType for TcpTransport {
    type Error = TcpTransportError;
}

impl embedded_io::Read for TcpTransport {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        // The VFS socket API currently surfaces string errors here, so we map
        // into a conservative transport `Other` kind for embedded-io.
        self.inner.read(buf).map_err(|_| TcpTransportError(ErrorKind::Other))
    }
}

impl embedded_io::Write for TcpTransport {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // The VFS socket API currently surfaces string errors here, so we map
        // into a conservative transport `Other` kind for embedded-io.
        self.inner.write(buf).map_err(|_| TcpTransportError(ErrorKind::Other))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn read_header_and_initial_body_from_tcp(
    stream: &mut TcpStream,
) -> Result<(Vec<u8>, usize), String> {
    let mut buffer = Vec::new();
    let mut temp_buf = [0u8; HEADER_STAGING_CHUNK_SIZE];
    let mut body_start = 0;
    let mut headers_done = false;
    let deadline_ns = deadline_after_ms(HEADER_READ_TIMEOUT_MS);

    for attempt in 0..MAX_HEADER_READ_ITERATIONS {
        if stem::syscall::monotonic_ns() >= deadline_ns {
            warn!("http: header read timed out after {} iterations", attempt);
            break;
        }
        let n = stream.read(&mut temp_buf)?;
        debug!("http: header read iter={} bytes={}", attempt, n);
        if n == 0 {
            break;
        }
        if let Some(start) = append_header_chunk_and_find_body_start(&mut buffer, &temp_buf[..n])? {
            body_start = start;
            headers_done = true;
            debug!(
                "http: headers complete iter={} total_buffer={} body_start={}",
                attempt,
                buffer.len(),
                body_start
            );
            break;
        }
    }

    if !headers_done {
        warn!("http: headers not completed initial_buffer={}", buffer.len());
    }

    Ok((buffer, body_start))
}

fn read_https_response_with_suite<S>(
    url: &ParsedUrl,
    request: &str,
) -> Result<(PortHandle, Vec<u8>, usize), String>
where
    S: embedded_tls::TlsCipherSuite + Send + 'static,
{
    // Keep the bridge bounded so producer-side `port_send_all` naturally applies
    // backpressure when the response reader lags behind.
    let (write_handle, read_handle) = port_create(HTTPS_PORT_CAPACITY_BYTES)
        .map_err(|e| format!("port create failed: {:?}", e))?;
    let host = url.host.clone();
    let req_bytes = request.as_bytes().to_vec();
    let port = url.port;
    // To handle the TLS context borrowing locally in the background thread,
    // we spawn a native stem task that will perform the TLS read loop.
    let _ = spawn_task_detached(move || {
        let tcp = match TcpStream::connect(&host, port) {
            Ok(t) => t,
            Err(e) => {
                let msg = format!("ERR: http connect failed: {}", e);
                let _ = port_send_all(write_handle, msg.as_bytes());
                let _ = port_close(write_handle);
                return;
            }
        };
        info!("http: background task: TCP connected to {}", host);

        let transport = TcpTransport { inner: tcp };
        let mut record_read_buf = [0u8; TLS_RECORD_READ_BUF_SIZE];
        let mut record_write_buf = [0u8; TLS_RECORD_WRITE_BUF_SIZE];
        let mut tls = TlsConnection::new(transport, &mut record_read_buf, &mut record_write_buf);

        let config = TlsConfig::new().with_server_name(&host).enable_rsa_signatures();
        info!("http: background task: starting TLS handshake with {}", host);
        let seed = match build_tls_seed() {
            Ok(s) => s,
            Err(e) => {
                let msg = format!("ERR: https seed failed: {:?}", e);
                let _ = port_send_all(write_handle, msg.as_bytes());
                let _ = port_close(write_handle);
                return;
            }
        };
        let rng = ChaCha20Rng::from_seed(seed);

        if let Err(e) = tls.open(TlsContext::new(&config, UnsecureProvider::new::<S>(rng))) {
            let msg = format!("ERR: https handshake failed: {:?}", e);
            warn!("http: background task: {}", msg);
            let _ = port_send_all(write_handle, msg.as_bytes());
            let _ = port_close(write_handle);
            return;
        }
        info!("http: background task: TLS handshake complete for {}", host);

        let mut offset = 0usize;
        while offset < req_bytes.len() {
            match tls.write(&req_bytes[offset..]) {
                Ok(0) => {
                    let _ = port_send_all(write_handle, b"ERR: https write returned 0 bytes");
                    let _ = port_close(write_handle);
                    return;
                }
                Ok(written) => offset += written,
                Err(e) => {
                    let msg = format!("ERR: https write failed: {:?}", e);
                    let _ = port_send_all(write_handle, msg.as_bytes());
                    let _ = port_close(write_handle);
                    return;
                }
            }
        }

        if let Err(e) = tls.flush() {
            let msg = format!("ERR: https flush failed: {:?}", e);
            let _ = port_send_all(write_handle, msg.as_bytes());
            let _ = port_close(write_handle);
            return;
        }

        let mut buf = [0u8; HTTPS_STREAM_CHUNK_SIZE];
        loop {
            match tls.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let mut offset = 0;
                    while offset < n {
                        match port_send_all(write_handle, &buf[offset..n]) {
                            Ok(written) => offset += written,
                            Err(abi::errors::Errno::EAGAIN) => {
                                stem::syscall::yield_now();
                                continue;
                            }
                            Err(_) => break,
                        }
                    }
                    if offset < n {
                        break; // Receiver likely closed or error occurred.
                    }
                }
                Err(e) => {
                    let kind = embedded_io::Error::kind(&e);
                    let err_text = format!("{:?}", e);
                    if matches!(
                        kind,
                        ErrorKind::ConnectionAborted
                            | ErrorKind::ConnectionReset
                            | ErrorKind::BrokenPipe
                    ) || err_text.contains("ConnectionClosed")
                        || err_text.contains("CryptoError")
                    {
                        if err_text.contains("ConnectionClosed") {
                            debug!("http: https read gracefully terminated ({:?})", err_text);
                        } else {
                            warn!(
                                "http: https read terminated with transport/crypto error {:?}",
                                err_text
                            );
                        }
                        break;
                    }
                    let msg = format!("ERR: https read failed: {:?}", e);
                    let _ = port_send_all(write_handle, msg.as_bytes());
                    break;
                }
            }
        }

        let _ = port_close(write_handle);
    });

    let mut response = Vec::new();
    let mut buf = [0u8; HEADER_STAGING_CHUNK_SIZE];
    let mut body_start = 0;
    let mut headers_done = false;
    let deadline_ns = deadline_after_ms(HEADER_READ_TIMEOUT_MS);

    let read_fd = stem::syscall::vfs::vfs_handle_from_port(read_handle as u32)
        .map_err(|e| format!("failed to bridge read port: {:?}", e))?;

    for attempt in 0..MAX_HEADER_READ_ITERATIONS {
        if stem::syscall::monotonic_ns() >= deadline_ns {
            warn!("http: https header read timed out after {} iterations", attempt);
            break;
        }

        trace!("http: waiting for header data from port (attempt={})", attempt);
        if let Err(e) = wait_fd_ready(
            read_fd,
            poll_flags::POLLIN,
            deadline_after_ms(HEADER_READ_TIMEOUT_MS / 10), // Small slice per iteration
            "http header read",
        ) {
            trace!("http: header read slice wait: {}", e);
            continue;
        }

        match port_recv(read_handle, &mut buf) {
            Ok(0) | Err(abi::errors::Errno::EPIPE) => {
                // Port closed early
                break;
            }
            Ok(n) => {
                let chunk = &buf[..n];
                // Check if it's an error message from the thread
                if chunk.starts_with(b"ERR: ") {
                    if let Ok(msg) = core::str::from_utf8(&chunk[5..]) {
                        return Err(msg.to_string());
                    }
                    return Err("Background thread reported unknown error".to_string());
                }

                if let Some(start) = append_header_chunk_and_find_body_start(&mut response, chunk)?
                {
                    body_start = start;
                    headers_done = true;
                    info!("http: headers complete body_start={}", body_start);
                    break;
                }
        }
    }

    let _ = vfs_close(read_fd);

    if !headers_done {
        warn!("http: https headers not completed, initial buffer={}", response.len());
    }

    Ok((read_handle, response, body_start))
}

fn read_https_response(
    url: &ParsedUrl,
    request: &str,
) -> Result<(PortHandle, Vec<u8>, usize), String> {
    match read_https_response_with_suite::<Aes128GcmSha256>(url, request) {
        Ok(v) => Ok(v),
        Err(e) if e.contains("InvalidHandshake") => {
            warn!("http: tls handshake with AES-128 failed ({}) - retrying with AES-256", e);
            read_https_response_with_suite::<Aes256GcmSha384>(url, request)
                .map_err(|e2| format!("{}; retry_with_aes256_failed: {}", e, e2))
        }
        Err(e) => Err(e),
    }
}

fn parse_ipv4(s: &str) -> Result<[u8; 4], ()> {
    let mut parts = s.split('.');
    let a = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    let b = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    let c = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    let d = parts.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
    if parts.next().is_some() {
        return Err(());
    }
    Ok([a, b, c, d])
}

// Minimal HTTP client
pub struct HttpClient;

impl HttpClient {
    pub fn post(url: &str, body: &str) -> Result<Response, String> {
        Self::request("POST", url, Some(body))
    }

    pub fn get(url: &str) -> Result<Response, String> {
        Self::request("GET", url, None)
    }

    fn request(method: &str, url: &str, body: Option<&str>) -> Result<Response, String> {
        // SECURITY: HTTPS currently uses embedded-tls UnsecureProvider
        // (no certificate-chain verification). This is intended as a minimal
        // in-OS TLS transport bootstrap and must not be treated as
        // production-grade authenticated HTTPS.
        debug!("http: request method={} url={}", method, url);
        let parsed = parse_url(url)?;
        debug!(
            "http: request resolved host={} port={} path={}",
            parsed.host, parsed.port, parsed.path
        );

        let scheme_default_port = match parsed.scheme {
            UrlScheme::Http => 80,
            UrlScheme::Https => 443,
        };
        let req = build_request(
            method,
            &parsed.host,
            parsed.port,
            &parsed.path,
            body,
            scheme_default_port,
        );

        let request_line_end = req.find("\r\n").unwrap_or(req.len());
        debug!("http: sending request bytes={} first_line={}", req.len(), &req[..request_line_end]);
        let (stream, buffer, body_start) = match parsed.scheme {
            UrlScheme::Http => {
                let mut stream = TcpStream::connect(&parsed.host, parsed.port)?;
                stream.write(req.as_bytes())?;
                let (buffer, body_start) = read_header_and_initial_body_from_tcp(&mut stream)?;
                (Some(ResponseStream::Http(stream)), buffer, body_start)
            }
            UrlScheme::Https => {
                let (rx, buffer, body_start) = read_https_response(&parsed, &req)?;
                (Some(ResponseStream::Https(rx)), buffer, body_start)
            }
        };

        Ok(Response { stream, buffer, cursor: body_start })
    }
}

pub enum ResponseStream {
    Http(TcpStream),
    Https(PortHandle),
}

impl Drop for ResponseStream {
    fn drop(&mut self) {
        if let ResponseStream::Https(handle) = self {
            let _ = port_close(*handle);
        }
    }
}

/// HTTP response body reader.
///
/// Body bytes are streamed from the underlying socket after the
/// initially buffered bytes are consumed.
pub struct Response {
    stream: Option<ResponseStream>,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Response {
    pub fn read_chunk(&mut self) -> Result<Vec<u8>, String> {
        if self.cursor < self.buffer.len() {
            let chunk = self.buffer[self.cursor..].to_vec();
            self.cursor = self.buffer.len();
            debug!("http: returning buffered chunk bytes={}", chunk.len());
            return Ok(chunk);
        }

        let Some(stream) = self.stream.as_mut() else {
            debug!("http: response fully consumed");
            return Ok(Vec::new());
        };

        match stream {
            ResponseStream::Http(tcp) => {
                let mut buf = [0u8; 4096];
                let n = tcp.read(&mut buf)?;
                if n == 0 {
                    debug!("http: response stream EOF");
                    return Ok(Vec::new());
                }
                debug!("http: returning streamed chunk bytes={}", n);
                Ok(buf[..n].to_vec())
            }
            ResponseStream::Https(handle) => {
                let mut buf = [0u8; HTTPS_STREAM_CHUNK_SIZE];

                let fd = stem::syscall::vfs::vfs_handle_from_port(*handle as u32)
                    .map_err(|e| format!("failed to bridge port to vfs: {:?}", e))?;

                let _ = wait_fd_ready(
                    fd,
                    poll_flags::POLLIN,
                    deadline_after_ms(IO_POLL_TIMEOUT_MS),
                    "http read_chunk",
                )?;

                match port_recv(*handle, &mut buf) {
                    Ok(n) => {
                        let _ = vfs_close(fd);
                        let chunk = &buf[..n];
                        if chunk.starts_with(b"ERR: ") {
                            if let Ok(msg) = core::str::from_utf8(&chunk[5..]) {
                                return Err(msg.to_string());
                            }
                            return Err("Background thread reported unknown error".to_string());
                        }
                        if n == 0 {
                            debug!("http: https response stream EOF");
                        } else {
                            debug!("http: returning streamed https chunk bytes={}", n);
                        }
                        Ok(chunk.to_vec())
                    }
                    Err(e) => {
                        let _ = vfs_close(fd);
                        Err(format!("port_recv failed: {:?}", e))
                    }
                }
            }
        }
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn append_header_chunk_and_find_body_start(
    buffer: &mut Vec<u8>,
    chunk: &[u8],
) -> Result<Option<usize>, String> {
    trace!(
        "http: append_header_chunk: chunk_len={} total_len_before={} total_len_after={}",
        chunk.len(),
        buffer.len(),
        buffer.len() + chunk.len()
    );
    buffer.extend_from_slice(chunk);

    if let Some(idx) = find_subsequence(buffer, b"\r\n\r\n") {
        let body_start = idx + 4;
        debug!("http: found body start at index {}", body_start);
        if body_start > MAX_HEADER_BYTES {
            return Err(format!(
                "response headers exceed max size: {} > {}",
                body_start, MAX_HEADER_BYTES
            ));
        }
        return Ok(Some(body_start));
    }

    if buffer.len() > MAX_HEADER_BYTES {
        return Err(format!(
            "response headers exceed max size: {} > {}",
            buffer.len(),
            MAX_HEADER_BYTES
        ));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn test_parse_ipv4() {
        // Valid IPs
        assert_eq!(parse_ipv4("127.0.0.1"), Ok([127, 0, 0, 1]));
        assert_eq!(parse_ipv4("192.168.1.100"), Ok([192, 168, 1, 100]));
        assert_eq!(parse_ipv4("0.0.0.0"), Ok([0, 0, 0, 0]));
        assert_eq!(parse_ipv4("255.255.255.255"), Ok([255, 255, 255, 255]));

        // Invalid format
        assert_eq!(parse_ipv4(""), Err(()));
        assert_eq!(parse_ipv4("1.2.3"), Err(()));
        assert_eq!(parse_ipv4("1.2.3.4.5"), Err(()));

        // Invalid numbers
        assert_eq!(parse_ipv4("256.0.0.1"), Err(()));
        assert_eq!(parse_ipv4("-1.0.0.0"), Err(()));

        // Non-numeric
        assert_eq!(parse_ipv4("a.b.c.d"), Err(()));
    }

    #[test]
    fn test_find_subsequence() {
        let sep = b"\r\n\r\n";

        // Found at end (typical header case)
        let data = b"Hello world\r\n\r\n";
        assert_eq!(find_subsequence(data, sep), Some(11));

        // Found in middle
        let data2 = b"Hello\r\n\r\nBody";
        assert_eq!(find_subsequence(data2, sep), Some(5));

        // Not found
        let data3 = b"Hello world";
        assert_eq!(find_subsequence(data3, sep), None);

        // Found at start
        let data4 = b"\r\n\r\nStart";
        assert_eq!(find_subsequence(data4, sep), Some(0));

        // Partial match
        let data5 = b"Partial\r\n\rEnd";
        assert_eq!(find_subsequence(data5, sep), None);

        // Overlapping needle
        assert_eq!(find_subsequence(b"aaaaa", b"aa"), Some(0));
    }

    #[test]
    fn test_parse_url_http_and_https() {
        let http = parse_url("http://example.com/path").unwrap();
        assert!(matches!(http.scheme, UrlScheme::Http));
        assert_eq!(http.host, "example.com");
        assert_eq!(http.port, 80);
        assert_eq!(http.path, "/path");

        let https = parse_url("https://example.com:8443/secure").unwrap();
        assert!(matches!(https.scheme, UrlScheme::Https));
        assert_eq!(https.host, "example.com");
        assert_eq!(https.port, 8443);
        assert_eq!(https.path, "/secure");
    }

    #[test]
    fn test_ns_to_timeout_ms_rounding() {
        assert_eq!(ns_to_timeout_ms(0), 1);
        assert_eq!(ns_to_timeout_ms(1), 1);
        assert_eq!(ns_to_timeout_ms(999_999), 1);
        assert_eq!(ns_to_timeout_ms(1_000_000), 1);
        assert_eq!(ns_to_timeout_ms(1_000_001), 2);
    }

    #[test]
    fn test_timeout_ms_until_deadline_errors_when_expired() {
        let now = stem::syscall::monotonic_ns();
        assert!(timeout_ms_until_deadline(now).is_err());
    }

    #[test]
    fn test_append_header_chunk_detects_fragmented_header_terminator() {
        let mut buffer = Vec::new();
        assert_eq!(
            append_header_chunk_and_find_body_start(
                &mut buffer,
                b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n"
            )
            .unwrap(),
            None
        );
        let body_start =
            append_header_chunk_and_find_body_start(&mut buffer, b"\r\nBody").unwrap().unwrap();
        assert_eq!(body_start, "HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\n".len());
        assert_eq!(&buffer[body_start..], b"Body");
    }

    #[test]
    fn test_append_header_chunk_allows_header_at_max_boundary() {
        let prefix = b"HTTP/1.1 200 OK\r\nX-Large: ";
        let suffix = b"\r\n\r\n";
        let filler_len = MAX_HEADER_BYTES - prefix.len() - suffix.len();
        let mut chunk = Vec::new();
        chunk.extend_from_slice(prefix);
        chunk.extend_from_slice(&vec![b'a'; filler_len]);
        chunk.extend_from_slice(suffix);

        let mut buffer = Vec::new();
        let body_start =
            append_header_chunk_and_find_body_start(&mut buffer, &chunk).unwrap().unwrap();
        assert_eq!(body_start, MAX_HEADER_BYTES);
    }

    #[test]
    fn test_append_header_chunk_rejects_header_over_max_boundary() {
        let mut buffer = Vec::new();
        let chunk = vec![b'a'; MAX_HEADER_BYTES];
        assert_eq!(append_header_chunk_and_find_body_start(&mut buffer, &chunk).unwrap(), None);

        let err = append_header_chunk_and_find_body_start(&mut buffer, b"b").unwrap_err();
        assert!(err.contains("response headers exceed max size"));
    }
}
