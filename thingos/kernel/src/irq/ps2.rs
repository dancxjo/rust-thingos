use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const PS2_QUEUE_CAPACITY: usize = 64;
const PS2_STATUS_OUTPUT_FULL: u8 = 0x01;

struct Ps2Queue {
    head: AtomicUsize,
    tail: AtomicUsize,
    buf: UnsafeCell<[u16; PS2_QUEUE_CAPACITY]>,
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

    fn push(&self, val: u16) {
        let head = self.head.load(Ordering::Relaxed);
        let next = (head + 1) % PS2_QUEUE_CAPACITY;
        let tail = self.tail.load(Ordering::Acquire);

        if next == tail {
            self.tail.store((tail + 1) % PS2_QUEUE_CAPACITY, Ordering::Release);
        }

        unsafe {
            (*self.buf.get())[head] = val;
        }
        self.head.store(next, Ordering::Release);
    }

    fn pop(&self) -> Option<u16> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail == head {
            return None;
        }

        let val = unsafe { (*self.buf.get())[tail] };
        self.tail.store((tail + 1) % PS2_QUEUE_CAPACITY, Ordering::Release);
        Some(val)
    }

    fn peek(&self) -> Option<u16> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail == head {
            return None;
        }

        unsafe { Some((*self.buf.get())[tail]) }
    }
}

unsafe impl Sync for Ps2Queue {}

static PS2_QUEUE: Ps2Queue = Ps2Queue::new();

static EXTENDED_PREFIX: AtomicBool = AtomicBool::new(false);
static ALT_DOWN: AtomicBool = AtomicBool::new(false);
static F1_DOWN: AtomicBool = AtomicBool::new(false);
static F12_DOWN: AtomicBool = AtomicBool::new(false);
static LOG_LEVEL_HOTKEY_PENDING: AtomicUsize = AtomicUsize::new(0);
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
    INPUT_QUEUE.pop().map(|v| v as u8)
}

const SCANCODE_MAP_NORMAL: &[u8] =
    b"\0\x1b1234567890-=\x08\tqwertyuiop[]\n\0asdfghjkl;'` \0zxcvbnm,./\0*\0 ";
const SCANCODE_MAP_SHIFT: &[u8] =
    b"\0\x1b!@#$%^&*()_+\x08\tQWERTYUIOP{}\n\0ASDFGHJKL:\"~ \0ZXCVBNM<>?\0*\0 ";

pub fn buffer_scancode(byte: u8, is_aux: bool) -> bool {
    let val = byte as u16 | ((is_aux as u16) << 8);
    PS2_QUEUE.push(val);
    update_pause_hotkey_state(byte)
}

pub fn take_scancode() -> Option<u8> {
    let res = PS2_QUEUE.pop();
    if let Some(val) = res {
        let byte = val as u8;
        crate::kinfo!("PS/2 take_scancode: popped 0x{:02x} (is_aux={})", byte, (val >> 8) != 0);
        Some(byte)
    } else {
        None
    }
}

pub fn overlay_status(status: u8) -> u8 {
    if let Some(val) = PS2_QUEUE.peek() {
        let is_aux = (val >> 8) != 0;
        let mut s = (status & !0x20) | PS2_STATUS_OUTPUT_FULL;
        if is_aux {
            s |= 0x20;
        }
        s
    } else {
        status
    }
}

pub fn take_terminal_hotkey() -> bool {
    TERMINAL_HOTKEY_PENDING.swap(false, Ordering::AcqRel)
}

pub fn take_log_level_hotkey() -> Option<u8> {
    match LOG_LEVEL_HOTKEY_PENDING.swap(0, Ordering::AcqRel) {
        0 => None,
        level => Some((level - 1) as u8),
    }
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
        (false, 0x2A) | (false, 0x36) => {
            SHIFT_DOWN.store(!released, Ordering::Release);
        }
        (false, 0x1D) | (true, 0x1D) => {
            CTRL_DOWN.store(!released, Ordering::Release);
        }
        (false, 0x3A) => {
            if !released {
                CAPS_LOCK.fetch_xor(true, Ordering::Release);
            }
        }
        _ => {}
    }

    match (extended, scancode) {
        (false, 0x38) | (true, 0x38) => {
            ALT_DOWN.store(!released, Ordering::Release);
            false
        }
        (false, 0x3B) => {
            if released {
                F1_DOWN.store(false, Ordering::Release);
            } else if !F1_DOWN.swap(true, Ordering::AcqRel) {
                crate::kinfo!("PS/2 hotkey F1 detected; cycling log level");
                let level = crate::logging::cycle_log_level();
                LOG_LEVEL_HOTKEY_PENDING.store(level as usize + 1, Ordering::Release);
            }
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

            let map = if SHIFT_DOWN.load(Ordering::Acquire) {
                SCANCODE_MAP_SHIFT
            } else {
                SCANCODE_MAP_NORMAL
            };
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
                    INPUT_QUEUE.push(c as u16);
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
        F1_DOWN.store(false, Ordering::Release);
        F12_DOWN.store(false, Ordering::Release);
        LOG_LEVEL_HOTKEY_PENDING.store(0, Ordering::Release);
        TERMINAL_HOTKEY_PENDING.store(false, Ordering::Release);
        crate::logging::set_log_level(3);
    }

    #[test]
    fn alt_f12_triggers_once() {
        reset_state();

        assert!(!buffer_scancode(0x38, false));
        assert!(buffer_scancode(0x58, false));
        assert!(!buffer_scancode(0x58, false));
        assert!(!buffer_scancode(0xD8, false));
        assert!(!buffer_scancode(0xB8, false));
    }

    #[test]
    fn right_alt_triggers_combo() {
        reset_state();

        assert!(!buffer_scancode(0xE0, false));
        assert!(!buffer_scancode(0x38, false));
        assert!(buffer_scancode(0x58, false));
    }

    #[test]
    fn buffered_status_sets_output_full() {
        reset_state();

        assert_eq!(overlay_status(0), 0);
        assert!(!buffer_scancode(0x1E, false));
        assert_ne!(overlay_status(0) & PS2_STATUS_OUTPUT_FULL, 0);
        assert_eq!(take_scancode(), Some(0x1E));
        assert_eq!(overlay_status(0) & PS2_STATUS_OUTPUT_FULL, 0);
    }

    #[test]
    fn plain_f12_sets_terminal_hotkey_once() {
        reset_state();

        assert!(!buffer_scancode(0x58, false));
        assert!(take_terminal_hotkey());
        assert!(!take_terminal_hotkey());

        // Typematic repeat while held should not retrigger.
        assert!(!buffer_scancode(0x58, false));
        assert!(!take_terminal_hotkey());

        // Release + press should retrigger.
        assert!(!buffer_scancode(0xD8, false));
        assert!(!take_terminal_hotkey());
        assert!(!buffer_scancode(0x58, false));
        assert!(take_terminal_hotkey());
    }

    #[test]
    fn plain_f1_cycles_log_level_once_per_press() {
        reset_state();
 
        assert_eq!(crate::logging::get_log_level(), 3);
        assert!(!buffer_scancode(0x3B, false));
        assert_eq!(take_log_level_hotkey(), Some(4));
        assert_eq!(crate::logging::get_log_level(), 4);
 
        // Typematic repeat while held should not cycle.
        assert!(!buffer_scancode(0x3B, false));
        assert_eq!(take_log_level_hotkey(), None);
        assert_eq!(crate::logging::get_log_level(), 4);
 
        assert!(!buffer_scancode(0xBB, false));
        assert!(!buffer_scancode(0x3B, false));
        assert_eq!(take_log_level_hotkey(), Some(5));
        assert_eq!(crate::logging::get_log_level(), 5);
    }

    #[test]
    fn f1_cycle_wraps_through_off() {
        reset_state();
        crate::logging::set_log_level(5);
 
        assert!(!buffer_scancode(0x3B, false));
        assert_eq!(take_log_level_hotkey(), Some(0));
        assert_eq!(crate::logging::get_log_level(), 0);
 
        assert!(!buffer_scancode(0xBB, false));
        assert!(!buffer_scancode(0x3B, false));
        assert_eq!(take_log_level_hotkey(), Some(1));
        assert_eq!(crate::logging::get_log_level(), 1);
    }
}
