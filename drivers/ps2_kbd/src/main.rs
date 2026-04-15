#![no_std]
#![no_main]
use alloc::string::ToString;
use core::default::Default;
extern crate alloc;

use stem::syscall::{ioport_read, irq_subscribe};
use stem::syscall::vfs::{vfs_thing_from_channel, vfs_write};
use stem::{error, info, warn};

/// PS/2 controller status register
const PS2_STATUS: usize = 0x64;
/// PS/2 controller data register  
const PS2_DATA: usize = 0x60;

/// Status: output buffer full
const STATUS_OUTPUT_FULL: usize = 0x01;
/// Status: data from aux port (mouse) - skip
const STATUS_AUX_DATA: usize = 0x20;

/// IRQ1 vector (keyboard) - legacy IRQ1 maps to vector 0x21 after IOAPIC remap
const KBD_VECTOR: u8 = 0x21;

/// Polling interval in milliseconds for the cooperative service loop.
const POLLING_INTERVAL_MS: u64 = 2;

/// Driver state node kind
const KIND_DRV_PS2_KBD: &str = "drv.Ps2Keyboard";

#[stem::main]
fn main(raw_write_handle: usize) -> ! {
    let handle = raw_write_handle as u32;

    stem::debug!("ps2_kbd: online (handle={})", handle);

    // Bridge the write channel handle to a VFS file descriptor so all I/O
    // flows through the VFS-first message path rather than the legacy port API.
    let fd = match vfs_thing_from_channel(handle) {
        Ok(f) => f,
        Err(e) => {
            stem::error!("ps2_kbd: fd bridge failed ({:?}), aborting", e);
            loop {
                stem::sleep_ms(1000);
            }
        }
    };

    // Subscribe to keyboard interrupt
    match irq_subscribe(KBD_VECTOR) {
        Ok(()) => {
            stem::debug!("ps2_kbd: subscribed to IRQ1 (vector 0x{:02x})", KBD_VECTOR);
            polling_loop(fd);
        }
        Err(e) => {
            info!(
                "ps2_kbd: IRQ subscribe failed ({:?}), falling back to polling",
                e
            );
            polling_loop(fd);
        }
    }
}

mod normalizer;
mod thigmonasty;

use abi::hid::{
    BristleEventHeader, EventType, KeyEventPayload, BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use thigmonasty::{KeyEdge, KeyboardState};

/// Drain all pending keyboard data from the controller
fn drain_keyboard_data(fd: u32, state: &mut KeyboardState, drop_counter: &mut u32) {
    // Read while data is available (handle burst of scancodes)
    for _ in 0..16 {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL == 0 {
            break; // No more data
        }

        if status & STATUS_AUX_DATA == 0 {
            // Keyboard data - read and send
            let scancode = ioport_read(PS2_DATA, 1) as u8;
            if let Some(edge) = state.process_ps2(scancode) {
                send_key_event(fd, edge, drop_counter);
            }
        } else {
            // If aux data (mouse), stop draining - let ps2_mouse handle it
            stem::debug!("ps2_kbd: yield on AUX data (mouse packet)");
            break;
        }
    }
}

fn send_key_event(fd: u32, edge: KeyEdge, drop_counter: &mut u32) {
    let timestamp_ns = stem::monotonic_ns();
    let mut buf = [0u8; 24]; // Max size is header + 4 byte payload

    let (event_type, key, mods, repeat) = match edge {
        KeyEdge::Down { key, mods, repeat } => (EventType::KeyDown, key, mods, repeat),
        KeyEdge::Up { key, mods } => (EventType::KeyUp, key, mods, false),
    };

    let header = BristleEventHeader {
        magic: BRISTLE_EVENT_MAGIC,
        version: BRISTLE_EVENT_VERSION,
        event_type: event_type as u16,
        timestamp_ns,
        payload_len: KeyEventPayload::SIZE as u32,
    };

    let payload = KeyEventPayload {
        key: key as u16,
        mods: mods.0,
        flags: if repeat { 1 } else { 0 },
    };

    buf[0..20].copy_from_slice(&header.to_bytes());
    buf[20..24].copy_from_slice(&payload.to_bytes());

    // Publish the event through the VFS-first message path.
    let send_ok = vfs_write(fd, &buf[..24])
        .map(|n| n == 24)
        .unwrap_or(false);
    if !send_ok {
        *drop_counter = drop_counter.wrapping_add(1);
        if *drop_counter <= 4 || *drop_counter % 100 == 0 {
            warn!(
                "ps2_kbd: dropped {} key events (write fd={} failed)",
                *drop_counter, fd
            );
        }
    }
}

/// Fallback polling loop (if IRQ subscribe fails)
fn polling_loop(fd: u32) -> ! {
    stem::debug!(
        "ps2_kbd: using cooperative polling loop ({}ms interval)",
        POLLING_INTERVAL_MS
    );
    let mut state = KeyboardState::new();
    let mut drop_counter = 0u32;
    loop {
        let status = ioport_read(PS2_STATUS, 1);

        if status & STATUS_OUTPUT_FULL != 0 {
            if status & STATUS_AUX_DATA == 0 {
                drain_keyboard_data(fd, &mut state, &mut drop_counter);
            } else {
                // Leave mouse bytes queued for ps2_mouse.
                stem::sleep_ms(1);
            }
        } else {
            // Rate limit the polling to avoid burning CPU
            stem::sleep_ms(POLLING_INTERVAL_MS);
        }
    }
}
