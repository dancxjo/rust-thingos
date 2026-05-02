use abi::hid::{Key, Mods};

pub enum WmAction {
    None,
    CycleFocus { forward: bool },
    ToggleFullscreen,
    CloseSurface,
}

pub fn handle_hotkey(key: Key, mods: Mods, is_repeat: bool) -> WmAction {
    if is_repeat {
        return WmAction::None;
    }

    if key == Key::Tab && mods.has_alt() {
        return WmAction::CycleFocus { forward: !mods.has_shift() };
    }

    if key == Key::F11 {
        return WmAction::ToggleFullscreen;
    }

    if (mods.has_alt() && key == Key::F4) || (mods.has_ctrl() && key == Key::W) {
        return WmAction::CloseSurface;
    }

    WmAction::None
}
