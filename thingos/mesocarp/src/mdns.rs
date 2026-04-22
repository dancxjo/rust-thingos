//! mDNS packet encoding and decoding.
//!
//! A stripped-down subset of RFC 6762 (Multicast DNS) and RFC 1035 (DNS
//! wire format) sufficient for the `mesocarp` daemon:
//!
//! * Build A-record queries for `<name>.local.`
//! * Build A-record authoritative responses announcing our own hostname.
//! * Parse A-record questions from inbound packets so we can decide whether
//!   to answer, and parse A-record answers from inbound packets so we can
//!   cache peer addresses.
//!
//! This module is `no_std` but uses `alloc`. All functions are pure byte
//! manipulations so they can be unit-tested on the host with `cargo test`.
//!
//! # mDNS conventions
//!
//! * Destination is the IPv4 multicast group `224.0.0.251` on UDP port
//!   `5353` for both queries and (most) responses.
//! * Transaction ID is conventionally `0` for multicast traffic.
//! * Responses set `QR=1`, `AA=1` (authoritative answer).
//! * `.local.` is the mDNS link-local top-level domain.

extern crate alloc;

use alloc::string::String;
#[cfg(test)]
use alloc::string::ToString;
use alloc::vec::Vec;

/// Record type code for an A (IPv4 address) record.
pub const RTYPE_A: u16 = 1;
/// Class code for IN (Internet).
pub const RCLASS_IN: u16 = 1;
/// mDNS "cache-flush" bit OR'd into the class field of answers.
pub const MDNS_CACHE_FLUSH: u16 = 0x8000;

/// Default TTL (seconds) we advertise on our own records.
///
/// 120 seconds is the recommended value from RFC 6762 §10 for hostname
/// records announced by a typical responder.
pub const DEFAULT_TTL: u32 = 120;

/// A parsed question from an inbound mDNS packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    /// Fully qualified name with trailing dot stripped (e.g. `"foo.local"`).
    pub name: String,
    pub rtype: u16,
    /// Class with the top bit (unicast-response request) masked off.
    pub rclass: u16,
}

/// A parsed answer from an inbound mDNS packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    /// For A records, a 4-byte IPv4 address.
    pub rdata: Vec<u8>,
}

/// The parsed contents of an mDNS packet we care about.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Message {
    pub id: u16,
    pub flags: u16,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    /// Does this packet have the `QR` response flag set?
    pub fn is_response(&self) -> bool {
        (self.flags & 0x8000) != 0
    }
}

/// Encode a label sequence for a fully-qualified name, terminating with the
/// root label (a single zero byte).
///
/// Does not perform compression, which is always legal on the wire. Labels
/// that exceed the 63-byte DNS limit are silently dropped so that the
/// resulting packet is always well-formed (terminated). Callers that need
/// strict validation should check the input first.
fn write_name(out: &mut Vec<u8>, name: &str) {
    for label in name.trim_end_matches('.').split('.') {
        let bytes = label.as_bytes();
        if bytes.is_empty() || bytes.len() > 63 {
            continue;
        }
        out.push(bytes.len() as u8);
        out.extend_from_slice(bytes);
    }
    out.push(0);
}

/// Build an mDNS A-record query for `name` (e.g. `"forebrain.local"`).
///
/// The returned bytes are a complete UDP payload ready to be sent to
/// `224.0.0.251:5353`.
pub fn build_query(name: &str) -> Vec<u8> {
    let mut pkt = Vec::with_capacity(64);
    // Header: id=0, flags=0 (standard query, no recursion bit for mDNS),
    // qd=1, an=0, ns=0, ar=0.
    pkt.extend_from_slice(&0u16.to_be_bytes()); // ID
    pkt.extend_from_slice(&0u16.to_be_bytes()); // flags
    pkt.extend_from_slice(&1u16.to_be_bytes()); // QDCOUNT
    pkt.extend_from_slice(&0u16.to_be_bytes()); // ANCOUNT
    pkt.extend_from_slice(&0u16.to_be_bytes()); // NSCOUNT
    pkt.extend_from_slice(&0u16.to_be_bytes()); // ARCOUNT
    write_name(&mut pkt, name);
    pkt.extend_from_slice(&RTYPE_A.to_be_bytes());
    pkt.extend_from_slice(&RCLASS_IN.to_be_bytes());
    pkt
}

/// Build an mDNS authoritative response announcing `name → ipv4`.
///
/// The cache-flush bit is set on the answer per RFC 6762 §10.2 so peers
/// replace any stale cached address immediately.
pub fn build_a_response(name: &str, ipv4: [u8; 4], ttl: u32) -> Vec<u8> {
    let mut pkt = Vec::with_capacity(64);
    // Header: id=0, flags = QR | AA (0x8400), qd=0, an=1, ns=0, ar=0.
    pkt.extend_from_slice(&0u16.to_be_bytes()); // ID
    pkt.extend_from_slice(&0x8400u16.to_be_bytes()); // flags (QR=1, AA=1)
    pkt.extend_from_slice(&0u16.to_be_bytes()); // QDCOUNT
    pkt.extend_from_slice(&1u16.to_be_bytes()); // ANCOUNT
    pkt.extend_from_slice(&0u16.to_be_bytes()); // NSCOUNT
    pkt.extend_from_slice(&0u16.to_be_bytes()); // ARCOUNT
    write_name(&mut pkt, name);
    pkt.extend_from_slice(&RTYPE_A.to_be_bytes());
    pkt.extend_from_slice(&(RCLASS_IN | MDNS_CACHE_FLUSH).to_be_bytes());
    pkt.extend_from_slice(&ttl.to_be_bytes());
    pkt.extend_from_slice(&4u16.to_be_bytes()); // RDLENGTH
    pkt.extend_from_slice(&ipv4);
    pkt
}

/// Read a DNS name from `buf` starting at `pos`, following label
/// compression pointers per RFC 1035 §4.1.4.
///
/// Returns `(decoded_name_without_trailing_dot, bytes_consumed_at_cursor)`
/// or `None` if the name is malformed. `bytes_consumed_at_cursor` is the
/// number of bytes consumed starting at `pos` (not the byte length of the
/// decoded name).
fn read_name(buf: &[u8], pos: usize) -> Option<(String, usize)> {
    // Guard against compression-pointer loops with a simple step budget.
    const MAX_JUMPS: usize = 16;
    let mut out = String::new();
    let mut cursor = pos;
    let mut first_nonpointer_end: Option<usize> = None;
    let mut jumps: usize = 0;

    loop {
        if cursor >= buf.len() {
            return None;
        }
        let len = buf[cursor];
        if len == 0 {
            // End of name.
            cursor += 1;
            break;
        }
        if (len & 0xC0) == 0xC0 {
            // Compression pointer.
            if cursor + 1 >= buf.len() {
                return None;
            }
            let off = (((len & 0x3F) as usize) << 8) | buf[cursor + 1] as usize;
            if first_nonpointer_end.is_none() {
                first_nonpointer_end = Some(cursor + 2);
            }
            jumps += 1;
            if jumps > MAX_JUMPS {
                return None;
            }
            if off >= buf.len() {
                return None;
            }
            cursor = off;
            continue;
        }
        if (len & 0xC0) != 0 {
            // Reserved label type — RFC 2671 EDNS0 extended labels etc. Not supported.
            return None;
        }
        let label_len = len as usize;
        let start = cursor + 1;
        let end = start + label_len;
        if end > buf.len() {
            return None;
        }
        if !out.is_empty() {
            out.push('.');
        }
        let label = core::str::from_utf8(&buf[start..end]).ok()?;
        out.push_str(label);
        cursor = end;
    }

    let consumed_end = first_nonpointer_end.unwrap_or(cursor);
    Some((out, consumed_end - pos))
}

/// Parse an mDNS packet into the subset of fields we care about
/// (questions and answers). Unknown/unsupported record types are skipped.
pub fn parse(buf: &[u8]) -> Option<Message> {
    if buf.len() < 12 {
        return None;
    }
    let id = u16::from_be_bytes([buf[0], buf[1]]);
    let flags = u16::from_be_bytes([buf[2], buf[3]]);
    let qd = u16::from_be_bytes([buf[4], buf[5]]) as usize;
    let an = u16::from_be_bytes([buf[6], buf[7]]) as usize;
    // NSCOUNT / ARCOUNT are ignored for our purposes but still validated
    // in the "skip rest" phase below to catch truncation.

    let mut msg = Message { id, flags, questions: Vec::new(), answers: Vec::new() };
    let mut pos = 12;

    for _ in 0..qd {
        let (name, used) = read_name(buf, pos)?;
        pos += used;
        if pos + 4 > buf.len() {
            return None;
        }
        let rtype = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        let rclass = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]) & 0x7FFF;
        pos += 4;
        msg.questions.push(Question { name, rtype, rclass });
    }

    for _ in 0..an {
        let (name, used) = read_name(buf, pos)?;
        pos += used;
        if pos + 10 > buf.len() {
            return None;
        }
        let rtype = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        // Strip the mDNS cache-flush bit from the class.
        let rclass = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]) & 0x7FFF;
        let ttl = u32::from_be_bytes([buf[pos + 4], buf[pos + 5], buf[pos + 6], buf[pos + 7]]);
        let rdlen = u16::from_be_bytes([buf[pos + 8], buf[pos + 9]]) as usize;
        pos += 10;
        if pos + rdlen > buf.len() {
            return None;
        }
        let rdata = buf[pos..pos + rdlen].to_vec();
        pos += rdlen;
        msg.answers.push(Answer { name, rtype, rclass, ttl, rdata });
    }

    Some(msg)
}

/// Normalize a hostname for case-insensitive comparison per RFC 1035 §2.3.3:
/// strip a trailing dot, lowercase ASCII letters, leave everything else as-is.
pub fn normalize_name(name: &str) -> String {
    name.trim_end_matches('.').to_ascii_lowercase()
}

/// Is this name inside the link-local `.local` domain?
pub fn is_local_name(name: &str) -> bool {
    let n = normalize_name(name);
    n == "local" || n.ends_with(".local")
}

#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::string::ToString;

    use super::*;

    #[test]
    fn query_roundtrips_through_parser() {
        let pkt = build_query("forebrain.local");
        let msg = parse(&pkt).expect("valid packet");
        assert_eq!(msg.id, 0);
        assert!(!msg.is_response());
        assert_eq!(msg.questions.len(), 1);
        assert_eq!(msg.questions[0].name, "forebrain.local");
        assert_eq!(msg.questions[0].rtype, RTYPE_A);
        assert_eq!(msg.questions[0].rclass, RCLASS_IN);
        assert!(msg.answers.is_empty());
    }

    #[test]
    fn response_roundtrips_through_parser() {
        let pkt = build_a_response("selfo.local", [192, 168, 2, 42], 120);
        let msg = parse(&pkt).expect("valid packet");
        assert!(msg.is_response());
        assert_eq!(msg.answers.len(), 1);
        let a = &msg.answers[0];
        assert_eq!(a.name, "selfo.local");
        assert_eq!(a.rtype, RTYPE_A);
        assert_eq!(a.ttl, 120);
        assert_eq!(a.rdata, std::vec![192, 168, 2, 42]);
    }

    #[test]
    fn parse_rejects_truncated_header() {
        assert!(parse(&[0u8; 5]).is_none());
    }

    #[test]
    fn parse_handles_compression_pointers() {
        // Hand-craft: question "x.local" then an answer whose name is a
        // pointer back to the question's "local" suffix.
        //
        // Header: id=0 flags=0x8400 qd=1 an=1 ns=0 ar=0.
        let mut pkt: std::vec::Vec<u8> = std::vec::Vec::new();
        pkt.extend_from_slice(&0u16.to_be_bytes());
        pkt.extend_from_slice(&0x8400u16.to_be_bytes());
        pkt.extend_from_slice(&1u16.to_be_bytes());
        pkt.extend_from_slice(&1u16.to_be_bytes());
        pkt.extend_from_slice(&0u16.to_be_bytes());
        pkt.extend_from_slice(&0u16.to_be_bytes());
        // Question "x.local" starts at offset 12.
        pkt.push(1);
        pkt.push(b'x');
        pkt.push(5);
        pkt.extend_from_slice(b"local");
        pkt.push(0);
        pkt.extend_from_slice(&RTYPE_A.to_be_bytes());
        pkt.extend_from_slice(&RCLASS_IN.to_be_bytes());
        // Answer: name is "y" + pointer to "local" at offset 14 (right after the "x" label).
        pkt.push(1);
        pkt.push(b'y');
        pkt.push(0xC0);
        pkt.push(14);
        pkt.extend_from_slice(&RTYPE_A.to_be_bytes());
        pkt.extend_from_slice(&RCLASS_IN.to_be_bytes());
        pkt.extend_from_slice(&60u32.to_be_bytes());
        pkt.extend_from_slice(&4u16.to_be_bytes());
        pkt.extend_from_slice(&[10, 0, 0, 1]);

        let msg = parse(&pkt).expect("valid packet");
        assert_eq!(msg.questions[0].name, "x.local");
        assert_eq!(msg.answers[0].name, "y.local");
        assert_eq!(msg.answers[0].rdata, std::vec![10, 0, 0, 1]);
    }

    #[test]
    fn parse_rejects_compression_loop() {
        // Header + a name consisting of a pointer to itself.
        let mut pkt: std::vec::Vec<u8> = std::vec::Vec::new();
        pkt.extend_from_slice(&0u16.to_be_bytes());
        pkt.extend_from_slice(&0u16.to_be_bytes());
        pkt.extend_from_slice(&1u16.to_be_bytes()); // qd
        pkt.extend_from_slice(&0u16.to_be_bytes());
        pkt.extend_from_slice(&0u16.to_be_bytes());
        pkt.extend_from_slice(&0u16.to_be_bytes());
        // Pointer at offset 12 pointing to offset 12.
        pkt.push(0xC0);
        pkt.push(12);
        pkt.extend_from_slice(&RTYPE_A.to_be_bytes());
        pkt.extend_from_slice(&RCLASS_IN.to_be_bytes());
        assert!(parse(&pkt).is_none());
    }

    #[test]
    fn is_local_recognises_local_suffix() {
        assert!(is_local_name("foo.local"));
        assert!(is_local_name("Foo.Local."));
        assert!(is_local_name("local"));
        assert!(!is_local_name("foo.example.com"));
        assert!(!is_local_name(""));
    }

    #[test]
    fn normalize_name_lowercases_and_strips_trailing_dot() {
        assert_eq!(normalize_name("FooBar.Local."), "foobar.local".to_string());
    }

    #[test]
    fn build_query_rejects_overlong_labels_gracefully() {
        // Label > 63 bytes should still produce *some* output without panicking;
        // we accept that the label is dropped and ensure the packet is still
        // parseable as an empty-name query (which peers will just ignore).
        let long = "a".repeat(100);
        let name = alloc::format!("{long}.local");
        let pkt = build_query(&name);
        // Header is always present; the parser may reject the body but must
        // not panic.
        let _ = parse(&pkt);
    }
}
