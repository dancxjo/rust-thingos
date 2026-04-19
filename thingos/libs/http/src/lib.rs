#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;

use abi::syscall::{poll_flags, PollThing};
use embedded_io::{Error as _, ErrorKind};
use embedded_tls::blocking::{Aes128GcmSha256, TlsConfig, TlsConnection, TlsContext, UnsecureProvider};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_poll, vfs_read, vfs_write};
use stem::{info, warn};

const IO_POLL_TIMEOUT_MS: u64 = 5_000;
const IO_POLL_SLICE_MS: u64 = 100;
const CONNECT_TIMEOUT_MS: u64 = 5_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TcpConnectState {
    Created,
    Connecting,
    Connected,
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
                "connected" | "established" | "fin-wait-1" | "fin-wait-2" | "close-wait" => {
                    TcpConnectState::Connected
                }
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
}

impl TcpStream {
    pub fn connect(host: &str, port: u16) -> Result<Self, String> {
        use abi::syscall::vfs_flags::{O_RDONLY, O_RDWR};

        info!("http: connect host={} port={}", host, port);

        // 1. Allocate a new TCP socket via /net/tcp/new
        let new_fd = vfs_open("/net/tcp/new", O_RDONLY).map_err(|e| format!("failed to open /net/tcp/new: {:?}", e))?;
        let mut buf = [0u8; 16];
        let n = vfs_read(new_fd, &mut buf).map_err(|e| format!("failed to read socket id: {:?}", e))?;
        let _ = vfs_close(new_fd);

        let socket_id = core::str::from_utf8(&buf[..n])
            .map_err(|_| "invalid socket id encoding")?
            .trim();
        info!("http: allocated tcp socket id={}", socket_id);

        // 2. Open ctl and data files
        let ctl_path = format!("/net/tcp/{}/ctl", socket_id);
        let data_path = format!("/net/tcp/{}/data", socket_id);

        let ctl_fd = vfs_open(&ctl_path, O_RDWR).map_err(|e| format!("failed to open ctl: {:?}", e))?;
        let data_fd = vfs_open(&data_path, O_RDWR).map_err(|e| format!("failed to open data: {:?}", e))?;

        // 3. Connect via ctl file
        let conn_cmd = format!("connect {} {}", host, port);
        vfs_write(ctl_fd, conn_cmd.as_bytes()).map_err(|e| format!("connect command failed: {:?}", e))?;

        info!("http: connect command issued for socket id={}", socket_id);

        let mut waited_ms = 0;
        loop {
            let state = read_tcp_state(socket_id);
            info!(
                "http: connect wait socket id={} state={:?} waited={} ms",
                socket_id, state, waited_ms
            );
            match state {
                TcpConnectState::Connected => break,
                TcpConnectState::Closed => {
                    let _ = vfs_close(data_fd);
                    let _ = vfs_close(ctl_fd);
                    return Err(format!("connect failed: socket {} closed before establishment", socket_id));
                }
                _ => {}
            }

            if waited_ms >= CONNECT_TIMEOUT_MS {
                let _ = vfs_close(data_fd);
                let _ = vfs_close(ctl_fd);
                return Err(format!(
                    "connect timed out waiting for socket {} to establish",
                    socket_id
                ));
            }

            let slice_ms = (CONNECT_TIMEOUT_MS - waited_ms).min(IO_POLL_SLICE_MS);
            let mut pollfd = [PollThing {
                thing: data_fd as i32,
                events: poll_flags::POLLOUT,
                revents: 0,
            }];
            let ready = vfs_poll(&mut pollfd, slice_ms)
                .map_err(|e| format!("poll failed while waiting for connect: {:?}", e))?;
            info!(
                "http: connect poll ready_count={} revents=0x{:x}",
                ready,
                pollfd[0].revents
            );
            waited_ms += slice_ms;
        }

        Ok(Self { data_fd, ctl_fd })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        info!("http: write {} bytes", data.len());
        vfs_write(self.data_fd, data).map_err(|e| format!("write failed: {:?}", e))
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        let mut waited_ms = 0;
        loop {
            match vfs_read(self.data_fd, buf) {
                Ok(n) => {
                    info!("http: read returned {} bytes after waiting {} ms", n, waited_ms);
                    return Ok(n);
                }
                Err(abi::errors::Errno::EAGAIN) => {
                    if waited_ms >= IO_POLL_TIMEOUT_MS {
                        warn!("http: read timed out after {} ms", waited_ms);
                        return Err("read timed out waiting for socket data".to_string());
                    }

                    let slice_ms = (IO_POLL_TIMEOUT_MS - waited_ms).min(IO_POLL_SLICE_MS);
                    info!("http: read would block; polling for {} ms", slice_ms);
                    let mut pollfd = [PollThing {
                        thing: self.data_fd as i32,
                        events: poll_flags::POLLIN,
                        revents: 0,
                    }];
                    let ready = vfs_poll(&mut pollfd, slice_ms)
                        .map_err(|e| format!("poll failed while waiting for read: {:?}", e))?;
                    info!(
                        "http: poll result ready_count={} revents=0x{:x}",
                        ready,
                        pollfd[0].revents
                    );
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

#[derive(Clone, Copy)]
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

    let (host_port, path) = if let Some(idx) = rest.find('/') {
        (&rest[..idx], &rest[idx..])
    } else {
        (rest, "/")
    };

    let (host, port) = if let Some(idx) = host_port.find(':') {
        (
            &host_port[..idx],
            host_port[idx + 1..]
                .parse::<u16>()
                .map_err(|_| "Invalid port".to_string())?,
        )
    } else {
        (host_port, default_port)
    };

    Ok(ParsedUrl {
        scheme,
        host: host.to_string(),
        port,
        path: path.to_string(),
    })
}

fn build_request(method: &str, host: &str, port: u16, path: &str, body: Option<&str>, default_port: u16) -> String {
    let mut req = String::new();
    write!(req, "{} {} HTTP/1.1\r\n", method, path).ok();
    if port != default_port {
        write!(req, "Host: {}:{}\r\n", host, port).ok();
    } else {
        write!(req, "Host: {}\r\n", host).ok();
    }
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

fn build_tls_seed(host: &str, port: u16, path: &str) -> [u8; 32] {
    let mut seed = [0u8; 32];
    let mut state = stem::syscall::monotonic_ns()
        ^ ((port as u64) << 32)
        ^ 0x9e37_79b9_7f4a_7c15;
    for (idx, byte) in host.as_bytes().iter().chain(path.as_bytes().iter()).enumerate() {
        let mix = (*byte as u64) ^ ((idx as u64).wrapping_mul(0x94d0_49bb_1331_11eb));
        state ^= mix.rotate_left((idx % 64) as u32);
        state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9).rotate_left(17);
    }
    for chunk in seed.chunks_mut(8) {
        state ^= state >> 33;
        state = state.wrapping_mul(0xff51_afd7_ed55_8ccd);
        state ^= state >> 33;
        state = state.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        state ^= state >> 33;
        chunk.copy_from_slice(&state.to_le_bytes());
    }
    seed
}

#[derive(Debug, Clone, Copy)]
struct TcpTransportError(ErrorKind);

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
        self.inner
            .read(buf)
            .map_err(|_| TcpTransportError(ErrorKind::Other))
    }
}

impl embedded_io::Write for TcpTransport {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.inner
            .write(buf)
            .map_err(|_| TcpTransportError(ErrorKind::Other))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn read_header_and_initial_body_from_tcp(stream: &mut TcpStream) -> Result<(Vec<u8>, usize), String> {
    let mut buffer = Vec::new();
    let mut temp_buf = [0u8; 1024];
    let mut body_start = 0;
    let mut headers_done = false;

    for iter in 0..20 {
        let n = stream.read(&mut temp_buf)?;
        info!("http: header read iter={} bytes={}", iter, n);
        if n == 0 {
            break;
        }
        buffer.extend_from_slice(&temp_buf[..n]);

        if let Some(idx) = find_subsequence(&buffer, b"\r\n\r\n") {
            body_start = idx + 4;
            headers_done = true;
            info!(
                "http: headers complete iter={} total_buffer={} body_start={}",
                iter,
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

fn read_https_response(url: &ParsedUrl, request: &str) -> Result<(Vec<u8>, usize), String> {
    let tcp = TcpStream::connect(&url.host, url.port)?;
    let mut transport = TcpTransport { inner: tcp };
    let mut record_read_buf = [0u8; 16_640];
    let mut record_write_buf = [0u8; 4_096];
    let mut tls =
        TlsConnection::new(transport, &mut record_read_buf, &mut record_write_buf);

    let config = TlsConfig::new()
        .with_server_name(&url.host)
        .enable_rsa_signatures();
    let seed = build_tls_seed(&url.host, url.port, &url.path);
    let rng = ChaCha20Rng::from_seed(seed);
    tls.open(TlsContext::new(
        &config,
        UnsecureProvider::new::<Aes128GcmSha256>(rng),
    ))
    .map_err(|e| format!("https handshake failed: {:?}", e))?;

    let mut offset = 0usize;
    while offset < request.len() {
        let written = tls
            .write(&request.as_bytes()[offset..])
            .map_err(|e| format!("https write failed: {:?}", e))?;
        if written == 0 {
            return Err("https write returned 0 bytes".to_string());
        }
        offset += written;
    }
    tls.flush()
        .map_err(|e| format!("https flush failed: {:?}", e))?;

    let mut response = Vec::new();
    let mut buf = [0u8; 1024];
    loop {
        match tls.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => response.extend_from_slice(&buf[..n]),
            Err(e) => {
                if matches!(
                    e.kind(),
                    ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset | ErrorKind::UnexpectedEof
                ) {
                    break;
                }
                return Err(format!("https read failed: {:?}", e));
            }
        }
    }

    let body_start = find_subsequence(&response, b"\r\n\r\n")
        .map(|idx| idx + 4)
        .unwrap_or(0);
    Ok((response, body_start))
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
        info!("http: request method={} url={}", method, url);
        let parsed = parse_url(url)?;
        info!(
            "http: request resolved host={} port={} path={}",
            parsed.host, parsed.port, parsed.path
        );

        let default_port = match parsed.scheme {
            UrlScheme::Http => 80,
            UrlScheme::Https => 443,
        };
        let req = build_request(method, &parsed.host, parsed.port, &parsed.path, body, default_port);

        let request_line_end = req.find("\r\n").unwrap_or(req.len());
        info!(
            "http: sending request bytes={} first_line={}",
            req.len(),
            &req[..request_line_end]
        );
        let (stream, buffer, body_start) = match parsed.scheme {
            UrlScheme::Http => {
                let mut stream = TcpStream::connect(&parsed.host, parsed.port)?;
                stream.write(req.as_bytes())?;
                let (buffer, body_start) = read_header_and_initial_body_from_tcp(&mut stream)?;
                (Some(stream), buffer, body_start)
            }
            UrlScheme::Https => {
                let (buffer, body_start) = read_https_response(&parsed, &req)?;
                (None, buffer, body_start)
            }
        };

        Ok(Response {
            stream,
            buffer,
            cursor: body_start,
        })
    }
}

pub struct Response {
    stream: Option<TcpStream>,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Response {
    pub fn read_chunk(&mut self) -> Result<Vec<u8>, String> {
        if self.cursor < self.buffer.len() {
            let chunk = self.buffer[self.cursor..].to_vec();
            self.cursor = self.buffer.len();
            info!("http: returning buffered chunk bytes={}", chunk.len());
            return Ok(chunk);
        }

        let mut buf = [0u8; 1024];
        let Some(stream) = self.stream.as_mut() else {
            return Ok(Vec::new());
        };
        let n = stream.read(&mut buf)?;
        if n == 0 {
            info!("http: response stream EOF");
            return Ok(Vec::new());
        }
        info!("http: returning streamed chunk bytes={}", n);
        Ok(buf[..n].to_vec())
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
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
