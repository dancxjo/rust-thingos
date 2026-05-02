import re

with open("thingos/bloom/src/input.rs", "r") as f:
    content = f.read()

old_block = """                if key.key() == Key::Tab && key.mods().has_alt() {
                    if !key.is_repeat() {
                        let forward = !key.mods().has_shift();
                        let (old_focus, new_focus) = scene.cycle_focus(forward);
                        stem::info!(
                            "bloom: focus cycled from {:?} to {:?} (forward={})",
                            old_focus,
                            new_focus,
                            forward
                        );
                        mark_focus_damage(scene, damage, old_focus, new_focus);
                        self.send_keyboard_focus_events(
                            scene,
                            old_focus,
                            new_focus,
                            wayland_evt_write,
                        );
                    }
                    return true;
                }

                if key.key() == Key::F11 {
                    if !key.is_repeat() {
                        stem::info!("BLOOM_FULLSCREEN_TOGGLE_TRIGGERED");
                        if let Some(surface_id) = scene.keyboard_focus {
                            self.toggle_fullscreen(scene, damage, wayland_evt_write, surface_id);
                        }
                    }
                    return true;
                }

                // Alt+F4 or Meta+W: Close
                if (key.mods().has_alt() && key.key() == Key::F4)
                    || (key.mods().has_meta() && key.key() == Key::W)
                {
                    if !key.is_repeat() {
                        if let Some(surface_id) = scene.keyboard_focus {
                            self.close_surface(scene, damage, wayland_evt_write, surface_id);
                        }
                    }
                    return true;
                }"""

new_block = """                match blossom::input::handle_hotkey(key.key(), key.mods(), key.is_repeat()) {
                    blossom::input::WmAction::CycleFocus { forward } => {
                        let (old_focus, new_focus) = scene.cycle_focus(forward);
                        stem::info!(
                            "bloom: focus cycled from {:?} to {:?} (forward={})",
                            old_focus,
                            new_focus,
                            forward
                        );
                        mark_focus_damage(scene, damage, old_focus, new_focus);
                        self.send_keyboard_focus_events(
                            scene,
                            old_focus,
                            new_focus,
                            wayland_evt_write,
                        );
                        return true;
                    }
                    blossom::input::WmAction::ToggleFullscreen => {
                        stem::info!("BLOOM_FULLSCREEN_TOGGLE_TRIGGERED");
                        if let Some(surface_id) = scene.keyboard_focus {
                            self.toggle_fullscreen(scene, damage, wayland_evt_write, surface_id);
                        }
                        return true;
                    }
                    blossom::input::WmAction::CloseSurface => {
                        if let Some(surface_id) = scene.keyboard_focus {
                            self.close_surface(scene, damage, wayland_evt_write, surface_id);
                        }
                        return true;
                    }
                    blossom::input::WmAction::None => {}
                }"""

if old_block in content:
    content = content.replace(old_block, new_block)
    with open("thingos/bloom/src/input.rs", "w") as f:
        f.write(content)
    print("Patched hotkeys successfully!")
else:
    print("Could not find old block to patch hotkeys!")

