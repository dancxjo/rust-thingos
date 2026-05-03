use abi::hid::{Key, Mods};

pub enum WmAction {
    None,
    CycleFocus { forward: bool },
    ToggleFullscreen,
    CloseSurface,
    ToggleRunBox,
    ToggleLauncher,
}

pub fn handle_hotkey(key: Key, mods: Mods, is_repeat: bool) -> WmAction {
    if is_repeat {
        return WmAction::None;
    }

    if key == Key::Tab && mods.has_alt() {
        return WmAction::CycleFocus { forward: !mods.has_shift() };
    }

    if (key == Key::R && mods.has_meta()) || (key == Key::F2 && mods.has_alt()) {
        return WmAction::ToggleRunBox;
    }

    if key == Key::Space && mods.has_meta() {
        return WmAction::ToggleLauncher;
    }

    if key == Key::F11 {
        return WmAction::ToggleFullscreen;
    }

    if (mods.has_alt() && key == Key::F4) || (mods.has_ctrl() && key == Key::W) {
        return WmAction::CloseSurface;
    }

    WmAction::None
}

#[cfg(test)]
mod tests {
    use abi::hid::Mods;

    use super::*;

    #[test]
    fn meta_r_toggles_runbox() {
        assert!(matches!(handle_hotkey(Key::R, Mods(Mods::META), false), WmAction::ToggleRunBox));
    }

    #[test]
    fn alt_f2_toggles_runbox() {
        assert!(matches!(handle_hotkey(Key::F2, Mods(Mods::ALT), false), WmAction::ToggleRunBox));
    }

    #[test]
    fn meta_space_toggles_launcher() {
        assert!(matches!(
            handle_hotkey(Key::Space, Mods(Mods::META), false),
            WmAction::ToggleLauncher
        ));
    }
}
