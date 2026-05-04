use abi::hid::{Key, Mods};

use crate::normalizer::set1_to_key;

pub struct KeyboardState {
    mods: u8,
    e0_prefix: bool,
    pressed: [u64; 4],
}

#[derive(Clone, Copy, Debug)]
pub enum KeyEdge {
    Down { key: Key, mods: Mods, repeat: bool },
    Up { key: Key, mods: Mods },
}

impl KeyboardState {
    pub const fn new() -> Self {
        Self { mods: 0, e0_prefix: false, pressed: [0; 4] }
    }

    pub fn process_set1(&mut self, byte: u8) -> Option<KeyEdge> {
        if byte == 0xE0 {
            self.e0_prefix = true;
            return None;
        }

        let is_break = byte & 0x80 != 0;
        let scancode = byte & 0x7F;
        let extended = self.e0_prefix;
        self.e0_prefix = false;

        let key = set1_to_key(scancode, extended);
        if key == Key::Unknown {
            return None;
        }

        let key_idx = key as u16 as usize;
        let (word_idx, bit_mask) = (key_idx / 64, 1u64 << (key_idx % 64));
        let mut was_pressed = false;

        if word_idx < self.pressed.len() {
            was_pressed = (self.pressed[word_idx] & bit_mask) != 0;
            if is_break {
                self.pressed[word_idx] &= !bit_mask;
            } else {
                self.pressed[word_idx] |= bit_mask;
            }
        }

        self.update_mods(key, !is_break);

        if is_break {
            Some(KeyEdge::Up { key, mods: Mods(self.mods) })
        } else {
            Some(KeyEdge::Down { key, mods: Mods(self.mods), repeat: was_pressed })
        }
    }

    fn update_mods(&mut self, key: Key, pressed: bool) {
        match key {
            Key::LeftShift | Key::RightShift => {
                if pressed {
                    self.mods |= Mods::SHIFT;
                } else if !self.is_key_pressed(Key::LeftShift)
                    && !self.is_key_pressed(Key::RightShift)
                {
                    self.mods &= !Mods::SHIFT;
                }
            }
            Key::LeftCtrl | Key::RightCtrl => {
                if pressed {
                    self.mods |= Mods::CTRL;
                } else if !self.is_key_pressed(Key::LeftCtrl)
                    && !self.is_key_pressed(Key::RightCtrl)
                {
                    self.mods &= !Mods::CTRL;
                }
            }
            Key::LeftAlt => {
                if pressed {
                    self.mods |= Mods::ALT;
                } else {
                    self.mods &= !Mods::ALT;
                }
            }
            Key::RightAlt => {
                if pressed {
                    self.mods |= Mods::ALTGR;
                } else {
                    self.mods &= !Mods::ALTGR;
                }
            }
            Key::LeftMeta | Key::RightMeta => {
                if pressed {
                    self.mods |= Mods::META;
                } else if !self.is_key_pressed(Key::LeftMeta)
                    && !self.is_key_pressed(Key::RightMeta)
                {
                    self.mods &= !Mods::META;
                }
            }
            _ => {}
        }
    }

    fn is_key_pressed(&self, key: Key) -> bool {
        let key_idx = key as u16 as usize;
        let word_idx = key_idx / 64;
        let bit_idx = key_idx % 64;
        word_idx < self.pressed.len() && (self.pressed[word_idx] & (1u64 << bit_idx)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set1_make_break_tracks_modifiers() {
        let mut state = KeyboardState::new();

        let edge = state.process_set1(0x2a).expect("left shift down");
        match edge {
            KeyEdge::Down { key, mods, repeat } => {
                assert_eq!(key, Key::LeftShift);
                assert!(mods.has_shift());
                assert!(!repeat);
            }
            _ => panic!("expected key down"),
        }

        let edge = state.process_set1(0x1e).expect("a down");
        match edge {
            KeyEdge::Down { key, mods, repeat } => {
                assert_eq!(key, Key::A);
                assert!(mods.has_shift());
                assert!(!repeat);
            }
            _ => panic!("expected key down"),
        }

        let edge = state.process_set1(0xaa).expect("left shift up");
        match edge {
            KeyEdge::Up { key, mods } => {
                assert_eq!(key, Key::LeftShift);
                assert!(!mods.has_shift());
            }
            _ => panic!("expected key up"),
        }
    }

    #[test]
    fn set1_extended_right_alt_sets_altgr() {
        let mut state = KeyboardState::new();

        assert!(state.process_set1(0xe0).is_none());
        let edge = state.process_set1(0x38).expect("right alt down");
        match edge {
            KeyEdge::Down { key, mods, .. } => {
                assert_eq!(key, Key::RightAlt);
                assert_eq!(mods.0 & Mods::ALTGR, Mods::ALTGR);
            }
            _ => panic!("expected key down"),
        }
    }
}
