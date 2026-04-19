#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{self, Write};
use stem::syscall::port::{port_close, port_create, port_recv, port_send_all, PortHandle};
use stem::thread::spawn_task_detached;

use embedded_io::ErrorKind;
use embedded_tls::blocking::{
    Aes128GcmSha256, Aes256GcmSha384, TlsConfig, TlsConnection, TlsContext, UnsecureProvider,
};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_write};
use stem::{debug, info, warn};

const IO_POLL_TIMEOUT_MS: u64 = 60_000;
const IO_POLL_SLICE_EARLY_MS: u64 = 1;
const IO_POLL_SLICE_MEDIUM_MS: u64 = 5;
const IO_POLL_SLICE_LATE_MS: u64 = 25;
const IO_POLL_EARLY_THRESHOLD_MS: u64 = 20;
const IO_POLL_MEDIUM_THRESHOLD_MS: u64 = 200;
const CONNECT_TIMEOUT_MS: u64 = 5_000;
const MAX_HEADER_READ_ITERATIONS: usize = 20;
// Max TLS record payload + TLS overhead as recommended by embedded-tls docs.
const TLS_RECORD_READ_BUF_SIZE: usize = 16_640;
// Write-side TLS record staging buffer.
const TLS_RECORD_WRITE_BUF_SIZE: usize = 4_096;
const DEFAULT_USER_AGENT: &str = "ThingOS-httpsd/0.1 (+https://github.com/dancxjo/thingos)";

/// Use shorter sleeps early to reduce first-byte/first-write latency, then
/// back off to avoid tight spinning during longer waits.
fn poll_sleep_slice_ms(waited_ms: u64, timeout_ms: u64) -> u64 {
    let remaining = timeout_ms.saturating_sub(waited_ms);
    let preferred = if waited_ms < IO_POLL_EARLY_THRESHOLD_MS {
        IO_POLL_SLICE_EARLY_MS
    } else if waited_ms < IO_POLL_MEDIUM_THRESHOLD_MS {
        IO_POLL_SLICE_MEDIUM_MS
    } else {
        IO_POLL_SLICE_LATE_MS
    };
    remaining.min(preferred)
}

/// Refresh state when we are in setup/terminal/unknown phases, or on the
/// first wait iteration before we have a stable writable-state signal.
fn should_refresh_tcp_state(last_state: TcpConnectState, waited_ms: u64) -> bool {
    matches!(
        last_state,
        TcpConnectState::Created
            | TcpConnectState::Connecting
            | TcpConnectState::Closed
            | TcpConnectState::Other
    ) || waited_ms == 0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TcpConnectState {
    Created,
    Connecting,
    Connected,
    CloseWait,
    Closed,
    Other,
}

fn read_tcp_state(socket_id: &str) -> TcpConnectState {
    use abi::syscall::vfs_flags::O_RDONLY;

    let status_path = format!("/net/tcp/{}/status", socket_id);
    let Ok(fd) = vfs_open(&status_path, O_RDONLY) else {
        return TcpConnectState::Other;
    };
    let mut buf = [0u8; 256];
    let n = vfs_read(fd, &mut buf).unwrap_or(0);
    let _ = vfs_close(fd);

    let text = core::str::from_utf8(&buf[..n]).unwrap_or("");
    for line in text.lines() {
        if let Some(state) = line.strip_prefix("state: ") {
            return match state.trim() {
                "created" => TcpConnectState::Created,
                "bound" | "syn-sent" | "syn-received" => TcpConnectState::Connecting,
                "connected" | "established" | "fin-wait-1" | "fin-wait-2" => {
                    TcpConnectState::Connected
                }
                "close-wait" => TcpConnectState::CloseWait,
                "closed" | "time-wait" | "closing" | "last-ack" => TcpConnectState::Closed,
                _ => TcpConnectState::Other,
            };
        }
    }
    TcpConnectState::Other
}

pub struct TcpStream {
    data_fd: u32,
    ctl_fd: u32,
    socket_id: String,
}

impl TcpStream {
    pub fn connect(host: &str, port: u16) -> Result<Self, String> {
        use abi::syscall::vfs_flags::{O_NONBLOCK, O_RDONLY, O_RDWR};

        debug!("http: connect host={} port={}", host, port);

        // 1. Allocate a new TCP socket via /net/tcp/new
        debug!("http: opening /net/tcp/new");
        let new_fd = vfs_open("/net/tcp/new", O_RDONLY | O_NONBLOCK)
            .map_err(|e| format!("failed to open /net/tcp/new: {:?}", e))?;
        let mut buf = [0u8; 16];
        let mut waited_ms = 0;
        let n = loop {
            match vfs_read(new_fd, &mut buf) {
                Ok(n) if n > 0 => break n,
                Ok(_) | Err(abi::errors::Errno::EAGAIN) => {
                    if waited_ms >= CONNECT_TIMEOUT_MS {
                        let _ = vfs_close(new_fd);
                        return Err("timed out waiting for /net/tcp/new socket id".to_string());
                    }
                    let slice_ms = poll_sleep_slice_ms(waited_ms, CONNECT_TIMEOUT_MS);
                    debug!("http: waiting for /net/tcp/new socket id ({} ms elapsed)", waited_ms);
                    stem::time::sleep_ms(slice_ms);
                    waited_ms += slice_ms;
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
        debug!("http: allocated tcp socket id={}", socket_id);

        // 2. Open ctl and data files
        let ctl_path = format!("/net/tcp/{}/ctl", socket_id);
        let data_path = format!("/net/tcp/{}/data", socket_id);

        debug!("http: opening ctl path {}", ctl_path);
        let ctl_fd =
            vfs_open(&ctl_path, O_RDWR).map_err(|e| format!("failed to open ctl: {:?}", e))?;
        debug!("http: opening data path {}", data_path);
        let data_fd = vfs_open(&data_path, O_RDWR | O_NONBLOCK)
            .map_err(|e| format!("failed to open data: {:?}", e))?;

        // 3. Connect via ctl file
        let conn_cmd = format!("connect {} {}", host, port);
        vfs_write(ctl_fd, conn_cmd.as_bytes())
            .map_err(|e| format!("connect command failed: {:?}", e))?;

        debug!("http: connect command issued for socket id={}", socket_id);

        Ok(Self { data_fd, ctl_fd, socket_id: socket_id.to_string() })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        debug!("http: write {} bytes", data.len());
        let mut waited_ms = 0;
        // Start at `Other` so the first loop iteration refreshes state.
        let mut last_state = TcpConnectState::Other;
        loop {
            match vfs_write(self.data_fd, data) {
                Ok(n) => {
                    if n == 0 {
                        let state = read_tcp_state(&self.socket_id);
                        last_state = state;
                        if waited_ms >= IO_POLL_TIMEOUT_MS {
                            return Err(format!(
                                "write returned 0 for {} ms (state={:?})",
                                waited_ms, state
                            ));
                        }
                        let slice_ms = poll_sleep_slice_ms(waited_ms, IO_POLL_TIMEOUT_MS);
                        debug!(
                            "http: write returned 0 while {:?}; sleeping for {} ms",
                            state, slice_ms
                        );
                        stem::time::sleep_ms(slice_ms);
                        waited_ms += slice_ms;
                        continue;
                    }
                    if waited_ms != 0 {
                        debug!("http: write completed after waiting {} ms", waited_ms);
                    }
                    return Ok(n);
                }
                Err(abi::errors::Errno::EAGAIN) => {
                    let state = if should_refresh_tcp_state(last_state, waited_ms) {
                        read_tcp_state(&self.socket_id)
                    } else {
                        last_state
                    };
                    last_state = state;
                    if matches!(state, TcpConnectState::Closed) {
                        return Err("write failed: socket closed before writable".to_string());
                    }

                    let timeout_ms = if matches!(
                        state,
                        TcpConnectState::Created | TcpConnectState::Connecting
                    ) {
                        CONNECT_TIMEOUT_MS
                    } else {
                        IO_POLL_TIMEOUT_MS
                    };

                    if waited_ms >= timeout_ms {
                        return Err(format!(
                            "write timed out waiting for socket writable (state={:?}, waited={} ms)",
                            state, waited_ms
                        ));
                    }

                    let slice_ms = poll_sleep_slice_ms(waited_ms, timeout_ms);
                    debug!(
                        "http: write would block while {:?}; sleeping for {} ms",
                        state, slice_ms
                    );
                    stem::time::sleep_ms(slice_ms);
                    waited_ms += slice_ms;
                }
                Err(e) => {
                    let state = read_tcp_state(&self.socket_id);
                    if matches!(state, TcpConnectState::Created | TcpConnectState::Connecting) {
                        if waited_ms >= CONNECT_TIMEOUT_MS {
                            return Err(format!(
                                "write failed while connecting: {:?} (state={:?})",
                                e, state
                            ));
                        }
                        let slice_ms = poll_sleep_slice_ms(waited_ms, CONNECT_TIMEOUT_MS);
                        debug!("http: write while {:?}; sleeping for {} ms", state, slice_ms);
                        stem::time::sleep_ms(slice_ms);
                        waited_ms += slice_ms;
                        continue;
                    }
                    return Err(format!("write failed: {:?} (state={:?})", e, state));
                }
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        let mut waited_ms = 0;
        loop {
            match vfs_read(self.data_fd, buf) {
                Ok(n) => {
                    if n == 0 {
                        // Some provider-backed TCP paths can transiently return 0
                        // before payload bytes arrive. Treat that as "wait" while
                        // the socket is still connected.
                        let state = read_tcp_state(&self.socket_id);
                        if matches!(state, TcpConnectState::Connected | TcpConnectState::Connecting)
                        {
                            if waited_ms >= IO_POLL_TIMEOUT_MS {
                                warn!(
                                    "http: read got 0 bytes while {:?}; timed out after {} ms",
                                    state, waited_ms
                                );
                                return Err(
                                    "read timed out waiting for socket data (zero-byte reads)"
                                        .to_string(),
                                );
                            }

                            let slice_ms = poll_sleep_slice_ms(waited_ms, IO_POLL_TIMEOUT_MS);
                            debug!(
                                "http: read returned 0 while {:?}; sleeping for {} ms",
                                state, slice_ms
                            );
                            stem::time::sleep_ms(slice_ms);
                            waited_ms += slice_ms;
                            continue;
                        }
                    }
                    debug!("http: read returned {} bytes after waiting {} ms", n, waited_ms);
                    return Ok(n);
                }
                Err(abi::errors::Errno::EAGAIN) => {
                    let state = read_tcp_state(&self.socket_id);
                    if matches!(state, TcpConnectState::Closed | TcpConnectState::CloseWait) {
                        debug!("http: read got EAGAIN but socket is closed/close-wait; treating as EOF");
                        return Ok(0);
                    }

                    if waited_ms >= IO_POLL_TIMEOUT_MS {
                        warn!("http: read timed out after {} ms", waited_ms);
                        return Err("read timed out waiting for socket data".to_string());
                    }

                    let slice_ms = poll_sleep_slice_ms(waited_ms, IO_POLL_TIMEOUT_MS);
                    debug!("http: read would block while {:?}; sleeping for {} ms", state, slice_ms);
                    stem::time::sleep_ms(slice_ms);
                    waited_ms += slice_ms;
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
    let mut temp_buf = [0u8; 1024];
    let mut body_start = 0;
    let mut headers_done = false;

    for attempt in 0..MAX_HEADER_READ_ITERATIONS {
        let n = stream.read(&mut temp_buf)?;
        debug!("http: header read iter={} bytes={}", attempt, n);
        if n == 0 {
            break;
        }
        buffer.extend_from_slice(&temp_buf[..n]);

        if let Some(idx) = find_subsequence(&buffer, b"\r\n\r\n") {
            body_start = idx + 4;
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
    let (write_handle, read_handle) = port_create(64 * 1024).map_err(|e| format!("port create failed: {:?}", e))?;
    let host = url.host.clone();
    let req_bytes = request.as_bytes().to_vec();
    
    // We do the TLS connection setup in the main thread so we can return errors immediately.
    let mut tcp = TcpStream::connect(&host, url.port)?;
    
    // To handle the TLS context borrowing locally in the background thread,
    // we spawn a native stem task that will perform the TLS read loop.
    let _ = spawn_task_detached(move || {
        let transport = TcpTransport { inner: tcp };
        let mut record_read_buf = [0u8; TLS_RECORD_READ_BUF_SIZE];
        let mut record_write_buf = [0u8; TLS_RECORD_WRITE_BUF_SIZE];
        let mut tls = TlsConnection::new(transport, &mut record_read_buf, &mut record_write_buf);

        let config = TlsConfig::new().with_server_name(&host).enable_rsa_signatures();
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
            let _ = port_send_all(write_handle, msg.as_bytes());
            let _ = port_close(write_handle);
            return;
        }

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

        let mut buf = [0u8; 16384];
        loop {
            match tls.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let mut offset = 0;
                    while offset < n {
                        match port_send_all(write_handle, &buf[offset..n]) {
                            Ok(written) => offset += written,
                            Err(_) => break,
                        }
                    }
                    if offset < n {
                        break; // Receiver likely closed or error occurred
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
                            warn!("http: https read terminated with transport/crypto error {:?}", err_text);
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
    let mut buf = [0u8; 4096];
    let mut body_start = 0;
    let mut headers_done = false;

    // Read chunks from the port until we have all the headers
    for attempt in 0..MAX_HEADER_READ_ITERATIONS {
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
                
                response.extend_from_slice(chunk);
                if let Some(idx) = find_subsequence(&response, b"\r\n\r\n") {
                    body_start = idx + 4;
                    headers_done = true;
                    debug!("http: headers complete body_start={}", body_start);
                    break;
                }
            }
            Err(e) => return Err(format!("port_recv failed: {:?}", e)),
        }
    }

    if !headers_done {
        warn!("http: https headers not completed, initial buffer={}", response.len());
    }

    Ok((read_handle, response, body_start))
}

fn read_https_response(url: &ParsedUrl, request: &str) -> Result<(PortHandle, Vec<u8>, usize), String> {
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
                let mut buf = [0u8; 4096];
                match port_recv(*handle, &mut buf) {
                    Ok(0) | Err(abi::errors::Errno::EPIPE) => {
                        debug!("http: https response stream EOF");
                        Ok(Vec::new())
                    }
                    Ok(n) => {
                        let chunk = &buf[..n];
                        if chunk.starts_with(b"ERR: ") {
                            if let Ok(msg) = core::str::from_utf8(&chunk[5..]) {
                                return Err(msg.to_string());
                            }
                            return Err("Background thread reported unknown error".to_string());
                        }
                        debug!("http: returning streamed https chunk bytes={}", n);
                        Ok(chunk.to_vec())
                    }
                    Err(e) => Err(format!("port_recv failed: {:?}", e)),
                }
            }
        }
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
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
}
