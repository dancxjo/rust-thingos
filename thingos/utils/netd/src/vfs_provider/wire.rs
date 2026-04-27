use alloc::vec::Vec;

use stem::syscall::port::{PortHandle, port_send_all};

use super::E_OK;

// ── Wire helpers ─────────────────────────────────────────────────────────────

pub fn send_resp(port: PortHandle, req_id: u16, data: &[u8]) {
    let mut resp = Vec::with_capacity(2 + data.len());
    resp.extend_from_slice(&req_id.to_le_bytes());
    resp.extend_from_slice(data);
    let _ = port_send_all(port, &resp);
}

pub(super) fn send_err(port: PortHandle, req_id: u16, errno: u8) {
    let mut resp = [0u8; 3];
    resp[0..2].copy_from_slice(&req_id.to_le_bytes());
    resp[2] = errno;
    let _ = port_send_all(port, &resp);
}

pub(super) fn send_handle(port: PortHandle, req_id: u16, handle: u64) {
    let mut resp = [0u8; 11];
    resp[0..2].copy_from_slice(&req_id.to_le_bytes());
    resp[2] = E_OK;
    resp[3..11].copy_from_slice(&handle.to_le_bytes());
    let _ = port_send_all(port, &resp);
}

pub(super) fn send_data(port: PortHandle, req_id: u16, data: &[u8]) {
    let mut resp = Vec::with_capacity(7 + data.len());
    resp.extend_from_slice(&req_id.to_le_bytes());
    resp.push(E_OK);
    resp.extend_from_slice(&(data.len() as u32).to_le_bytes());
    resp.extend_from_slice(data);
    let _ = port_send_all(port, &resp);
}

pub(super) fn send_write_ok(port: PortHandle, req_id: u16, bytes_written: u32) {
    let mut resp = [0u8; 7];
    resp[0..2].copy_from_slice(&req_id.to_le_bytes());
    resp[2] = E_OK;
    resp[3..7].copy_from_slice(&bytes_written.to_le_bytes());
    let _ = port_send_all(port, &resp);
}
