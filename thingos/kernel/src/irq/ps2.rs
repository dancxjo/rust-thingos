use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const PS2_QUEUE_CAPACITY: usize = 64;
const PS2_STATUS_OUTPUT_FULL: u8 = 0x01;

struct Ps2Queue {
    head: AtomicUsize,
    tail: AtomicUsize,
    buf: UnsafeCell<[u8; PS2_QUEUE_CAPACITY]>,
}

impl Ps2Queue {
    const fn new() -> Self {
        Self {
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            buf: UnsafeCell::new([0; PS2_QUEUE_CAPACITY]),
        }
    }

    fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire) == self.tail.load(Ordering::Acquire)
    }

    fn push(&self, byte: u8) {
        let head = self.head.load(Ordering::Relaxed);
        let next = (head + 1) % PS2_QUEUE_CAPACITY;
        let tail = self.tail.load(Ordering::Acquire);

        if next == tail {
            self.tail.store((tail + 1) % PS2_QUEUE_CAPACITY, Ordering::Release);
        }

        unsafe {
            (*self.buf.get())[head] = byte;
        }
        self.head.store(next, Ordering::Release);
    }

    fn pop(&self) -> Option<u8> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail == head {
            return None;
        }

        let byte = unsafe { (*self.buf.get())[tail] };
        self.tail.store((tail + 1) % PS2_QUEUE_CAPACITY, Ordering::Release);
        Some(byte)
    }
}

unsafe impl Sync for Ps2Queue {}

static PS2_QUEUE: Ps2Queue = Ps2Queue::new();

static EXTENDED_PREFIX: AtomicBool = AtomicBool::new(false);
static ALT_DOWN: AtomicBool = AtomicBool::new(false);
static F12_DOWN: AtomicBool = AtomicBool::new(false);
static TERMINAL_HOTKEY_PENDING: AtomicBool = AtomicBool::new(false);
static FB_INPUT_ENABLED: AtomicBool = AtomicBool::new(false);

static SHIFT_DOWN: AtomicBool = AtomicBool::new(false);
static CTRL_DOWN: AtomicBool = AtomicBool::new(false);
static CAPS_LOCK: AtomicBool = AtomicBool::new(false);

static INPUT_QUEUE: Ps2Queue = Ps2Queue::new();

pub fn set_fb_input_enabled(enabled: bool) {
    FB_INPUT_ENABLED.store(enabled, Ordering::Release);
}

pub fn take_input_char() -> Option<u8> {
    INPUT_QUEUE.pop()
}

const SCANCODE_MAP_NORMAL: &[u8] = b"\0\x1b1234567890-=\x08\tqwertyuiop[]\n\0asdfghjkl;'` \0zxcvbnm,./\0*\0 ";
const SCANCODE_MAP_SHIFT: &[u8] = b"\0\x1b!@#$%^&*()_+\x08\tQWERTYUIOP{}\n\0ASDFGHJKL:\"~ \0ZXCVBNM<>?\0*\0 ";

pub fn buffer_scancode(byte: u8) -> bool {
    PS2_QUEUE.push(byte);
    update_pause_hotkey_state(byte)
}

pub fn take_scancode() -> Option<u8> {
    PS2_QUEUE.pop()
}

pub fn take_terminal_hotkey() -> bool {
    TERMINAL_HOTKEY_PENDING.swap(false, Ordering::AcqRel)
}

pub fn overlay_status(status: u8) -> u8 {
    if PS2_QUEUE.is_empty() { status } else { status | PS2_STATUS_OUTPUT_FULL }
}

fn update_pause_hotkey_state(byte: u8) -> bool {
    match byte {
        0xE0 => {
            EXTENDED_PREFIX.store(true, Ordering::Release);
            return false;
        }
        0xE1 => {
            EXTENDED_PREFIX.store(false, Ordering::Release);
            return false;
        }
        _ => {}
    }

    let extended = EXTENDED_PREFIX.swap(false, Ordering::AcqRel);
    let released = (byte & 0x80) != 0;
    let scancode = byte & 0x7F;

    // Track modifiers
    match (extended, scancode) {
        (false, 0x2A) | (false, 0x36) => { SHIFT_DOWN.store(!released, Ordering::Release); }
        (false, 0x1D) | (true, 0x1D) => { CTRL_DOWN.store(!released, Ordering::Release); }
        (false, 0x3A) => { if !released { CAPS_LOCK.fetch_xor(true, Ordering::Release); } }
        _ => {}
    }

    match (extended, scancode) {
        (false, 0x38) | (true, 0x38) => {
            ALT_DOWN.store(!released, Ordering::Release);
            false
        }
        (false, 0x58) => {
            if released {
                F12_DOWN.store(false, Ordering::Release);
                false
            } else {
                let first_press = !F12_DOWN.swap(true, Ordering::AcqRel);
                if first_press {
                    if ALT_DOWN.load(Ordering::Acquire) {
                        true
                    } else {
                        TERMINAL_HOTKEY_PENDING.store(true, Ordering::Release);
                        false
                    }
                } else {
                    false
                }
            }
        }
        (false, s) if !released && FB_INPUT_ENABLED.load(Ordering::Acquire) => {
            let shift = SHIFT_DOWN.load(Ordering::Acquire) ^ CAPS_LOCK.load(Ordering::Acquire);
            let ctrl = CTRL_DOWN.load(Ordering::Acquire);
            
            let map = if SHIFT_DOWN.load(Ordering::Acquire) { SCANCODE_MAP_SHIFT } else { SCANCODE_MAP_NORMAL };
            if (s as usize) < map.len() {
                let mut c = map[s as usize];
                if c != 0 {
                    if ctrl {
                        if (b'a'..=b'z').contains(&c) {
                            c -= b'a' - 1;
                        } else if (b'A'..=b'Z').contains(&c) {
                            c -= b'A' - 1;
                        }
                    }
                    INPUT_QUEUE.push(c);
                }
            }
            false
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reset_state() {
        while take_scancode().is_some() {}
        EXTENDED_PREFIX.store(false, Ordering::Release);
        ALT_DOWN.store(false, Ordering::Release);
        F12_DOWN.store(false, Ordering::Release);
        TERMINAL_HOTKEY_PENDING.store(false, Ordering::Release);
    }

    #[test]
    fn alt_f12_triggers_once() {
        reset_state();

        assert!(!buffer_scancode(0x38));
        assert!(buffer_scancode(0x58));
        assert!(!buffer_scancode(0x58));
        assert!(!buffer_scancode(0xD8));
        assert!(!buffer_scancode(0xB8));
    }

    #[test]
    fn right_alt_triggers_combo() {
        reset_state();

        assert!(!buffer_scancode(0xE0));
        assert!(!buffer_scancode(0x38));
        assert!(buffer_scancode(0x58));
    }

    #[test]
    fn buffered_status_sets_output_full() {
        reset_state();

        assert_eq!(overlay_status(0), 0);
        assert!(!buffer_scancode(0x1E));
        assert_ne!(overlay_status(0) & PS2_STATUS_OUTPUT_FULL, 0);
        assert_eq!(take_scancode(), Some(0x1E));
        assert_eq!(overlay_status(0) & PS2_STATUS_OUTPUT_FULL, 0);
    }

    #[test]
    fn plain_f12_sets_terminal_hotkey_once() {
        reset_state();

        assert!(!buffer_scancode(0x58));
        assert!(take_terminal_hotkey());
        assert!(!take_terminal_hotkey());

        // Typematic repeat while held should not retrigger.
        assert!(!buffer_scancode(0x58));
        assert!(!take_terminal_hotkey());

        // Release + press should retrigger.
        assert!(!buffer_scancode(0xD8));
        assert!(!take_terminal_hotkey());
        assert!(!buffer_scancode(0x58));
        assert!(take_terminal_hotkey());
    }
}
