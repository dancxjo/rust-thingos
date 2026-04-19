#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::str::FromStr;

use abi::syscall::{poll_flags, PollThing};
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_poll, vfs_read, vfs_write};
use stem::{info, warn};

const IO_POLL_TIMEOUT_MS: u64 = 5_000;
const IO_POLL_SLICE_MS: u64 = 100;

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

        // Wait for connection to establish (poor man's poll/check for now)
        // In a real implementation we would poll status or events.
        stem::time::sleep_ms(100);
        info!("http: connect command issued for socket id={}", socket_id);

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
        let (host, port, path, final_url) = if url.starts_with("http://") {
            let rest = &url[7..];
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
                        .map_err(|_| "Invalid port")?,
                )
            } else {
                (host_port, 80)
            };
            (host.to_string(), port, path.to_string(), url.to_string())
        } else {
            // Use proxy for non-http (likely https)
            // Proxy format: http://10.0.2.2:8081/?url=<encoded_url>
            // Note: 10.0.2.2 is QEMU host loopback
            let encoded_url = url_encode(url);
            let proxy_path = format!("/?url={}", encoded_url);
            ("10.0.2.2".to_string(), 8081, proxy_path, url.to_string())
        };

        info!("http: request resolved host={} port={} path={}", host, port, path);

        let mut stream = TcpStream::connect(&host, port)?;

        let mut req = String::new();
        write!(req, "{} {} HTTP/1.1\r\n", method, path).ok();
        if (url.starts_with("http://") && port != 80) || (!url.starts_with("http://") && port != 80) {
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

        let request_line_end = req.find("\r\n").unwrap_or(req.len());
        info!(
            "http: sending request bytes={} first_line={}",
            req.len(),
            &req[..request_line_end]
        );
        stream.write(req.as_bytes())?;

        let mut buffer = Vec::new();
        let mut temp_buf = [0u8; 1024];
        let mut body_start = 0;
        let mut headers_done = false;

        // Initial read loop to find headers
        for iter in 0..20 {
            // Limit tries
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

        Ok(Response {
            stream,
            buffer,
            cursor: body_start,
        })
    }
}

pub struct Response {
    stream: TcpStream,
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
        let n = self.stream.read(&mut buf)?;
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

fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.as_bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(b) {
            out.push(*b as char);
        } else {
            write!(out, "%{:02X}", b).ok();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn test_url_encoding() {
        // Alphanumeric - should not be encoded
        assert_eq!(url_encode("abc123XYZ"), "abc123XYZ");

        // Allowed characters - should not be encoded
        assert_eq!(url_encode("a-b_c.d~e"), "a-b_c.d~e");

        // Space - should be encoded as %20
        assert_eq!(url_encode("hello world"), "hello%20world");

        // Special characters - should be encoded
        // / -> %2F, : -> %3A
        assert_eq!(url_encode("http://example.com"), "http%3A%2F%2Fexample.com");

        // Empty string
        assert_eq!(url_encode(""), "");
    }

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
}
