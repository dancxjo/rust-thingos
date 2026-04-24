#![no_std]
#![no_main]
extern crate alloc;

use core::ops::ControlFlow;

use abi::display_driver_protocol as drvproto;
use abi::driver_interface::{
    DRIVER_DESCRIPTOR_ABI_VERSION, DeviceInfo, DriverClass, DriverDescriptor, DriverStartContext,
    ProbeResult, Status,
};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::info;
use stem::service_loop::{ServiceEvent, ServiceLoop};
use stem::syscall::message::{KindId, msg_send};
const THINGOS_DRIVER_NAME: &[u8] = b"display_fake";

#[unsafe(no_mangle)]
#[used]
pub static THINGOS_DRIVER: DriverDescriptor = DriverDescriptor {
    abi_version: DRIVER_DESCRIPTOR_ABI_VERSION,
    driver_name_ptr: THINGOS_DRIVER_NAME.as_ptr(),
    driver_name_len: THINGOS_DRIVER_NAME.len(),
    driver_class: DriverClass::Display,
    flags: 0,
    probe: thingos_driver_probe,
    start: thingos_driver_start,
};

unsafe extern "C" fn thingos_driver_probe(
    _dev: *const DeviceInfo,
    out: *mut ProbeResult,
) -> Status {
    if out.is_null() {
        return Status::InvalidArgument;
    }
    let out = &mut *out;
    out.matched = 0;
    out.score = 0;
    out.claimed_class = DriverClass::Display;
    out.flags = 0;
    Status::NoMatch
}

unsafe extern "C" fn thingos_driver_start(_ctx: *const DriverStartContext) -> Status {
    main(0)
}

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.display.Fake\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

#[derive(Clone, Copy, Debug)]
struct FakeConfig {
    caps: u32,
    max_rects: u16,
    burst: bool,
}

fn parse_config(arg: usize) -> FakeConfig {
    // Config word layout in arg>>32 (for tests):
    // bits 0-1: caps mask (bit0=DIRTY_RECTS, bit1=FULLFRAME; 0 => default both)
    // bits 8-15: max_rects (0 => default 8)
    // bit 17: burst (send ACKs twice)
    let cfg = (arg >> 32) as u32;
    let mut caps = 0u32;
    if cfg & 0x1 != 0 {
        caps |= drvproto::CAP_DIRTY_RECTS;
    }
    if cfg & 0x2 != 0 {
        caps |= drvproto::CAP_FULLFRAME;
    }
    if caps == 0 {
        caps = drvproto::CAP_DIRTY_RECTS | drvproto::CAP_FULLFRAME;
    }

    let max_rects = ((cfg >> 8) & 0xFF) as u16;
    let max_rects = if max_rects == 0 { 8 } else { max_rects };

    let burst = cfg & (1 << 17) != 0;

    FakeConfig { caps, max_rects, burst }
}

fn send_welcome(reply_pid: u32, welcome: &drvproto::WelcomePayload) {
    let mut buf = [0u8; drvproto::WELCOME_PAYLOAD_WIRE_SIZE];
    if let Some(len) = drvproto::encode_welcome_payload_le(welcome, &mut buf) {
        let _ = msg_send(reply_pid, drvproto::KIND_DRV_DISPLAY_WELCOME, &buf[..len]);
    }
}

fn send_ack(reply_pid: u32, burst: bool) {
    let _ = msg_send(reply_pid, drvproto::KIND_DRV_DISPLAY_ACK, &[]);
    if burst {
        let _ = msg_send(reply_pid, drvproto::KIND_DRV_DISPLAY_ACK, &[]);
    }
}

fn send_err(reply_pid: u32, code: u32) {
    let err = drvproto::ErrResp { code };
    let mut buf = [0u8; drvproto::ERR_RESP_WIRE_SIZE];
    if let Some(len) = drvproto::encode_err_resp_le(&err, &mut buf) {
        let _ = msg_send(reply_pid, drvproto::KIND_DRV_DISPLAY_ERR, &buf[..len]);
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    let config = parse_config(arg);

    info!(
        "display_fake: starting (inbox-backed) caps=0x{:x} max_rects={} burst={}",
        config.caps,
        config.max_rects,
        config.burst
    );

    let mut svc = ServiceLoop::new(512).expect("display_fake: ServiceLoop::new");

    let mut bound = false;
    let mut bound_fd: Option<u32> = None;

    svc.run(
        |event| match event {
            ServiceEvent::Message { kind, payload } => {
                handle_message(kind, payload, &config, &mut bound, &mut bound_fd);
                ControlFlow::Continue(())
            }
            ServiceEvent::InboxClosed => ControlFlow::Break(()),
            _ => ControlFlow::Continue(()),
        },
        None,
    )
    .ok();

    loop {
        stem::yield_now();
    }
}

fn handle_message(
    kind: KindId,
    payload: &[u8],
    config: &FakeConfig,
    bound: &mut bool,
    bound_fd: &mut Option<u32>,
) {
    if kind == drvproto::KIND_DRV_DISPLAY_HELLO {
        if let Some(req) = drvproto::decode_hello_req_inbox_le(payload) {
            let welcome = drvproto::WelcomePayload {
                proto_major: drvproto::PROTO_MAJOR,
                proto_minor: drvproto::PROTO_MINOR,
                have_caps: config.caps & req.want_caps,
                max_rects: config.max_rects,
                reserved: 0,
            };
            send_welcome(req.reply_pid, &welcome);
        }
    } else if kind == drvproto::KIND_DRV_DISPLAY_BIND {
        if let Some(req) = drvproto::decode_bind_req_inbox_le(payload) {
            *bound = true;
            *bound_fd = Some(req.fb_fd);
            send_ack(req.reply_pid, config.burst);
        } else if let Some(reply_pid) = extract_reply_pid(payload) {
            send_err(reply_pid, 2);
        }
    } else if kind == drvproto::KIND_DRV_DISPLAY_PRESENT {
        if let Some(req) = drvproto::decode_present_req_inbox_le(payload) {
            if !*bound || bound_fd.is_none() {
                send_err(req.reply_pid, 1);
                return;
            }
            if (req.flags & drvproto::PRESENT_FLAG_FULLFRAME == 0)
                && req.rect_count > config.max_rects as u32
            {
                send_err(req.reply_pid, 3);
                return;
            }
            send_ack(req.reply_pid, config.burst);
        }
    }
}

/// Extract the `reply_pid` (first 4 bytes LE) from a payload when full
/// decode fails, so we can still send an error reply.
fn extract_reply_pid(payload: &[u8]) -> Option<u32> {
    if payload.len() < 4 {
        return None;
    }
    Some(u32::from_le_bytes(payload[0..4].try_into().ok()?))
}
