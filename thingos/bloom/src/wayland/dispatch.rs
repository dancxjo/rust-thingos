//! Wayland message dispatcher.
//!
//! Each `dispatch_*` function handles messages arriving on a particular
//! interface.  The dispatcher receives a decoded [`WireMsg`] and mutates
//! the client state, emitting port commands to the main bloom thread as
//! needed.
//!
//! # Global table (advertised in `wl_registry`)
//!
//! | name | interface        | version |
//! |------|------------------|---------|
//! | 1    | wl_compositor    | 4       |
//! | 2    | wl_shm           | 1       |
//! | 3    | xdg_wm_base      | 1       |
//! | 4    | wl_seat          | 5       |
//! | 5    | wl_output        | 2       |
//! | 6    | wl_subcompositor | 1       |
//! | 7    | wl_data_device_manager   | 3       |
//! | 8    | zwp_linux_dmabuf_v1      | 3       |
//! | 9    | wp_presentation          | 1       |
//! | 10   | zwlr_layer_shell_v1      | 4       |

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use abi::pixel::PixelFormat;
use blossom::{BlossomCommand, BlossomError};
use stem::{debug as blossom_debug, warn as blossom_warn};

use crate::wayland::client::{DmabufPlane, ObjectEntry, WaylandClient};
use crate::wayland::ipc;
use crate::wayland::wire::{WireMsg, read_i32, read_string, read_u32};

// ── Global registry constants ────────────────────────────────────────────────

pub const GLOBAL_WL_COMPOSITOR: u32 = 1;
pub const GLOBAL_WL_SHM: u32 = 2;
pub const GLOBAL_XDG_WM_BASE: u32 = 3;
pub const GLOBAL_WL_SEAT: u32 = 4;
pub const GLOBAL_WL_OUTPUT: u32 = 5;
pub const GLOBAL_WL_SUBCOMPOSITOR: u32 = 6;
pub const GLOBAL_WL_DATA_DEVICE_MANAGER: u32 = 7;
pub const GLOBAL_ZWP_LINUX_DMABUF: u32 = 8;
pub const GLOBAL_WP_PRESENTATION: u32 = 9;
pub const GLOBAL_ZWLR_LAYER_SHELL: u32 = 10;

const DRM_FORMAT_ARGB8888: u32 = 0x3432_5241; // "AR24"
const DRM_FORMAT_XRGB8888: u32 = 0x3432_5258; // "XR24"
const DRM_FORMAT_MOD_LINEAR: u64 = 0;

// ── Top-level dispatcher ─────────────────────────────────────────────────────

/// Dispatch a single decoded Wayland message.
///
/// Returns a (possibly empty) list of raw IPC command bytes that must be sent
/// to the main bloom thread via the command port.
pub fn dispatch(
    msg: &WireMsg,
    client: &mut WaylandClient,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
    next_surface_key: &mut u32,
    output: &crate::display::OutputInfo,
) -> Vec<Vec<u8>> {
    let obj_id = msg.object_id;

    // Look up the object type.  Clone/copy just the variant discriminant info
    // we need without holding the borrow.
    let obj_kind = match client.objects.get(&obj_id) {
        Some(e) => classify(e),
        None => {
            // Unknown object — send protocol error.
            client.send_protocol_error(obj_id, 0, "unknown object");
            return vec![];
        }
    };

    match obj_kind {
        ObjKind::Display => dispatch_display(msg, client, next_surface_key, cmd_write, output),
        ObjKind::Registry => dispatch_registry(msg, client, output),
        ObjKind::Compositor => dispatch_compositor(msg, client, next_surface_key, cmd_write),
        ObjKind::Shm => dispatch_shm(msg, client),
        ObjKind::Dmabuf => dispatch_dmabuf(msg, client),
        ObjKind::DmabufParams => dispatch_dmabuf_params(msg, client, obj_id, output),
        ObjKind::ShmPool => dispatch_shm_pool(msg, client, obj_id),
        ObjKind::Buffer => dispatch_buffer(msg, client, obj_id),
        ObjKind::Surface => dispatch_surface(msg, client, obj_id, blossom, cmd_write),
        ObjKind::Callback => vec![],
        ObjKind::XdgWmBase => dispatch_xdg_wm_base(msg, client, obj_id, blossom),
        ObjKind::XdgPositioner => vec![],
        ObjKind::XdgSurface => dispatch_xdg_surface(msg, client, obj_id, blossom, cmd_write),
        ObjKind::XdgToplevel => dispatch_xdg_toplevel(msg, client, obj_id, blossom, cmd_write),
        ObjKind::XdgPopup => dispatch_xdg_popup(msg, client, obj_id, blossom),
        ObjKind::Seat => dispatch_seat(msg, client, obj_id),
        ObjKind::Pointer => dispatch_pointer(msg, client, obj_id),
        ObjKind::Keyboard => dispatch_keyboard(msg, client, obj_id),
        ObjKind::Output => dispatch_output(msg, client, obj_id),
        ObjKind::Subcompositor => dispatch_subcompositor(msg, client, obj_id),
        ObjKind::Subsurface => dispatch_subsurface(msg, client, obj_id, cmd_write),
        ObjKind::DataDeviceManager => dispatch_data_device_manager(msg, client),
        ObjKind::DataSource => dispatch_data_source(msg, client, obj_id),
        ObjKind::DataDevice => dispatch_data_device(msg, client, obj_id),
        ObjKind::DataOffer => dispatch_data_offer(msg, client, obj_id),
        ObjKind::Presentation => dispatch_presentation(msg, client, obj_id),
        ObjKind::PresentationFeedback => dispatch_presentation_feedback(msg, client, obj_id),
        ObjKind::LayerShell => dispatch_layer_shell(msg, client, obj_id, output),
        ObjKind::LayerSurface => dispatch_layer_surface(msg, client, obj_id, cmd_write, output),
        ObjKind::Region => dispatch_region(msg, client, obj_id),
        ObjKind::Destroyed | ObjKind::Unknown => vec![],
    }
}

// ── Object kind classifier ────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
enum ObjKind {
    Display,
    Registry,
    Compositor,
    Shm,
    Dmabuf,
    DmabufParams,
    ShmPool,
    Buffer,
    Surface,
    Callback,
    XdgWmBase,
    XdgPositioner,
    XdgSurface,
    XdgToplevel,
    XdgPopup,
    Seat,
    Pointer,
    Keyboard,
    Output,
    Subcompositor,
    Subsurface,
    DataDeviceManager,
    DataSource,
    DataDevice,
    DataOffer,
    Presentation,
    PresentationFeedback,
    LayerShell,
    LayerSurface,
    Region,
    Destroyed,
    Unknown,
}

fn classify(e: &ObjectEntry) -> ObjKind {
    match e {
        ObjectEntry::Display => ObjKind::Display,
        ObjectEntry::Registry => ObjKind::Registry,
        ObjectEntry::Compositor => ObjKind::Compositor,
        ObjectEntry::Shm => ObjKind::Shm,
        ObjectEntry::Dmabuf => ObjKind::Dmabuf,
        ObjectEntry::DmabufParams { .. } => ObjKind::DmabufParams,
        ObjectEntry::ShmPool { .. } => ObjKind::ShmPool,
        ObjectEntry::Buffer { .. } => ObjKind::Buffer,
        ObjectEntry::Surface { .. } => ObjKind::Surface,
        ObjectEntry::Callback => ObjKind::Callback,
        ObjectEntry::XdgWmBase => ObjKind::XdgWmBase,
        ObjectEntry::XdgPositioner => ObjKind::XdgPositioner,
        ObjectEntry::XdgSurface { .. } => ObjKind::XdgSurface,
        ObjectEntry::XdgToplevel { .. } => ObjKind::XdgToplevel,
        ObjectEntry::XdgPopup { .. } => ObjKind::XdgPopup,
        ObjectEntry::Seat => ObjKind::Seat,
        ObjectEntry::Pointer => ObjKind::Pointer,
        ObjectEntry::Keyboard => ObjKind::Keyboard,
        ObjectEntry::Output => ObjKind::Output,
        ObjectEntry::Subcompositor => ObjKind::Subcompositor,
        ObjectEntry::Subsurface { .. } => ObjKind::Subsurface,
        ObjectEntry::DataDeviceManager => ObjKind::DataDeviceManager,
        ObjectEntry::DataSource { .. } => ObjKind::DataSource,
        ObjectEntry::DataDevice { .. } => ObjKind::DataDevice,
        ObjectEntry::DataOffer => ObjKind::DataOffer,
        ObjectEntry::Presentation => ObjKind::Presentation,
        ObjectEntry::PresentationFeedback => ObjKind::PresentationFeedback,
        ObjectEntry::LayerShell => ObjKind::LayerShell,
        ObjectEntry::LayerSurface { .. } => ObjKind::LayerSurface,
        ObjectEntry::Region { .. } => ObjKind::Region,
        ObjectEntry::Destroyed => ObjKind::Destroyed,
    }
}

// ── wl_display ────────────────────────────────────────────────────────────────

/// wl_display opcodes
const WL_DISPLAY_SYNC: u16 = 0;
const WL_DISPLAY_GET_REGISTRY: u16 = 1;

fn dispatch_display(
    msg: &WireMsg,
    client: &mut WaylandClient,
    _next_surface_key: &mut u32,
    _cmd_write: u32,
    output: &crate::display::OutputInfo,
) -> Vec<Vec<u8>> {
    use crate::wayland::wire::encode_string;
    match msg.opcode {
        WL_DISPLAY_SYNC => {
            // sync(new_id) → create wl_callback, send done immediately.
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            client.insert(new_id, ObjectEntry::Callback);
            // wl_callback.done(serial=0)
            let payload = 0u32.to_ne_bytes();
            client.send(new_id, 0, &payload);
            client.destroy(new_id);
        }
        WL_DISPLAY_GET_REGISTRY => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            client.insert(new_id, ObjectEntry::Registry);
            // Send wl_registry.global for each advertised global.
            // wl_registry.global opcode = 0: (name: uint, interface: string, version: uint)
            for &(name, iface, version) in &[
                (GLOBAL_WL_COMPOSITOR, "wl_compositor", 4u32),
                (GLOBAL_WL_SHM, "wl_shm", 1u32),
                (GLOBAL_XDG_WM_BASE, "xdg_wm_base", 1u32),
                (GLOBAL_WL_SEAT, "wl_seat", 5u32),
                (GLOBAL_WL_OUTPUT, "wl_output", 2u32),
                (GLOBAL_WL_SUBCOMPOSITOR, "wl_subcompositor", 1u32),
                (GLOBAL_WL_DATA_DEVICE_MANAGER, "wl_data_device_manager", 3u32),
                (GLOBAL_WP_PRESENTATION, "wp_presentation", 1u32),
                (GLOBAL_ZWLR_LAYER_SHELL, "zwlr_layer_shell_v1", 4u32),
            ] {
                let mut p = Vec::new();
                p.extend_from_slice(&name.to_ne_bytes());
                p.extend_from_slice(&encode_string(iface));
                p.extend_from_slice(&version.to_ne_bytes());
                client.send(new_id, 0, &p);
            }
            if output.supports_dmabuf {
                let mut p = Vec::new();
                p.extend_from_slice(&GLOBAL_ZWP_LINUX_DMABUF.to_ne_bytes());
                p.extend_from_slice(&encode_string("zwp_linux_dmabuf_v1"));
                p.extend_from_slice(&3u32.to_ne_bytes());
                client.send(new_id, 0, &p);
            }
            // No wl_registry.global_done in the Wayland protocol;
            // clients learn all globals when the initial global list is done
            // (they stop waiting after bind).
        }
        _ => {}
    }
    vec![]
}

// ── wl_registry ───────────────────────────────────────────────────────────────

/// wl_registry.bind opcode = 0
const WL_REGISTRY_BIND: u16 = 0;

fn dispatch_registry(
    msg: &WireMsg,
    client: &mut WaylandClient,
    output: &crate::display::OutputInfo,
) -> Vec<Vec<u8>> {
    if msg.opcode != WL_REGISTRY_BIND {
        return vec![];
    }
    // bind(name: uint, interface: string, version: uint, new_id: uint)
    let name = match read_u32(&msg.data, 0) {
        Some(n) => n,
        None => return vec![],
    };
    let (_, consumed) = match read_string(&msg.data, 4) {
        Some(s) => s,
        None => return vec![],
    };
    let after_str = 4 + consumed;
    let _version = read_u32(&msg.data, after_str);
    let new_id = match read_u32(&msg.data, after_str + 4) {
        Some(id) => id,
        None => return vec![],
    };

    match name {
        GLOBAL_WL_COMPOSITOR => {
            client.insert(new_id, ObjectEntry::Compositor);
        }
        GLOBAL_WL_SHM => {
            client.insert(new_id, ObjectEntry::Shm);
            // Advertise ARGB8888 format (0) and XRGB8888 format (1).
            // wl_shm.format opcode = 0: (format: uint)
            client.send(new_id, 0, &0u32.to_ne_bytes()); // ARGB8888
            client.send(new_id, 0, &1u32.to_ne_bytes()); // XRGB8888
        }
        GLOBAL_XDG_WM_BASE => {
            client.insert(new_id, ObjectEntry::XdgWmBase);
        }
        GLOBAL_WL_SEAT => {
            client.insert(new_id, ObjectEntry::Seat);
            send_seat_capabilities(client, new_id);
        }
        GLOBAL_WL_OUTPUT => {
            client.insert(new_id, ObjectEntry::Output);
            send_output_events(client, new_id, output);
        }
        GLOBAL_WL_SUBCOMPOSITOR => {
            client.insert(new_id, ObjectEntry::Subcompositor);
        }
        GLOBAL_WL_DATA_DEVICE_MANAGER => {
            client.insert(new_id, ObjectEntry::DataDeviceManager);
        }
        GLOBAL_ZWP_LINUX_DMABUF if output.supports_dmabuf => {
            client.insert(new_id, ObjectEntry::Dmabuf);
            send_dmabuf_formats(client, new_id, output);
        }
        GLOBAL_WP_PRESENTATION => {
            client.insert(new_id, ObjectEntry::Presentation);
            // Per the Wayland Presentation Time protocol, the server must send
            // `clock_id` to the client right after binding so the client knows
            // which clock the timestamps in `presented` events come from.
            // Bloom uses the monotonic clock (CLOCK_MONOTONIC = 1 on Linux);
            // see `stem::time::monotonic_ns` which all other Bloom timestamps
            // are derived from.
            const CLOCK_MONOTONIC: u32 = 1;
            // wp_presentation.clock_id opcode = 0: (clk_id: uint)
            client.send(new_id, 0, &CLOCK_MONOTONIC.to_ne_bytes());
        }
        GLOBAL_ZWLR_LAYER_SHELL => {
            client.insert(new_id, ObjectEntry::LayerShell);
        }
        _ => {
            client.send_protocol_error(new_id, 0, "unknown global");
        }
    }
    vec![]
}

// ── wl_seat / wl_pointer / wl_keyboard ──────────────────────────────────────

const WL_SEAT_GET_POINTER: u16 = 0;
const WL_SEAT_GET_KEYBOARD: u16 = 1;
const WL_SEAT_RELEASE: u16 = 3;
const WL_POINTER_RELEASE: u16 = 1;
const WL_KEYBOARD_RELEASE: u16 = 1;

const WL_SEAT_CAPABILITY_POINTER: u32 = 1;
const WL_SEAT_CAPABILITY_KEYBOARD: u32 = 2;

fn send_seat_capabilities(client: &WaylandClient, seat_obj: u32) {
    let caps = WL_SEAT_CAPABILITY_POINTER | WL_SEAT_CAPABILITY_KEYBOARD;
    client.send(seat_obj, 0, &caps.to_ne_bytes());
    client.send(seat_obj, 1, &crate::wayland::wire::encode_string("seat0"));
}

fn dispatch_seat(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_SEAT_GET_POINTER => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::Pointer);
            }
        }
        WL_SEAT_GET_KEYBOARD => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::Keyboard);
                send_keyboard_keymap(client, new_id);
                send_keyboard_repeat_info(client, new_id);
            }
        }
        WL_SEAT_RELEASE => {
            client.destroy(obj_id);
        }
        _ => {}
    }
    vec![]
}

fn dispatch_pointer(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    if msg.opcode == WL_POINTER_RELEASE {
        client.destroy(obj_id);
    }
    vec![]
}

fn dispatch_keyboard(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    if msg.opcode == WL_KEYBOARD_RELEASE {
        client.destroy(obj_id);
    }
    vec![]
}

/// Self-contained XKB keymap for the US International (us-intl) layout.
///
/// This is a complete, self-contained XKB text keymap (no `include` statements)
/// that defines the us-intl layout with dead keys.  It is sent to Wayland
/// clients as the `wl_keyboard.keymap` event payload when they bind a keyboard.
///
/// The string is null-terminated; `US_INTL_KEYMAP.len()` includes the `\0`.
const US_INTL_KEYMAP: &[u8] = b"xkb_keymap {\
  xkb_keycodes \"thingos_evdev\" {\
    minimum = 8;\
    maximum = 255;\
    <ESC>  =  9; <AE01> = 10; <AE02> = 11; <AE03> = 12; <AE04> = 13;\
    <AE05> = 14; <AE06> = 15; <AE07> = 16; <AE08> = 17; <AE09> = 18;\
    <AE10> = 19; <AE11> = 20; <AE12> = 21; <BKSP> = 22; <TAB>  = 23;\
    <AD01> = 24; <AD02> = 25; <AD03> = 26; <AD04> = 27; <AD05> = 28;\
    <AD06> = 29; <AD07> = 30; <AD08> = 31; <AD09> = 32; <AD10> = 33;\
    <AD11> = 34; <AD12> = 35; <RTRN> = 36; <LCTL> = 37; <AC01> = 38;\
    <AC02> = 39; <AC03> = 40; <AC04> = 41; <AC05> = 42; <AC06> = 43;\
    <AC07> = 44; <AC08> = 45; <AC09> = 46; <AC10> = 47; <AC11> = 48;\
    <TLDE> = 49; <LFSH> = 50; <BKSL> = 51; <AB01> = 52; <AB02> = 53;\
    <AB03> = 54; <AB04> = 55; <AB05> = 56; <AB06> = 57; <AB07> = 58;\
    <AB08> = 59; <AB09> = 60; <AB10> = 61; <RTSH> = 62; <KPMU> = 63;\
    <LALT> = 64; <SPCE> = 65; <CAPS> = 66; <FK01> = 67; <FK02> = 68;\
    <FK03> = 69; <FK04> = 70; <FK05> = 71; <FK06> = 72; <FK07> = 73;\
    <FK08> = 74; <FK09> = 75; <FK10> = 76; <NMLK> = 77; <SCLK> = 78;\
    <KP7>  = 79; <KP8>  = 80; <KP9>  = 81; <KPSU> = 82; <KP4>  = 83;\
    <KP5>  = 84; <KP6>  = 85; <KPAD> = 86; <KP1>  = 87; <KP2>  = 88;\
    <KP3>  = 89; <KP0>  = 90; <KPDL> = 91; <LSGT> = 94; <FK11> = 95;\
    <FK12> = 96; <KPEN> = 104; <RCTL> = 105; <KPDV> = 106; <PRSC> = 107;\
    <RALT> = 108; <HOME> = 110; <UP>   = 111; <PGUP> = 112; <LEFT> = 113;\
    <RGHT> = 114; <END>  = 115; <DOWN> = 116; <PGDN> = 117; <INS>  = 118;\
    <DELE> = 119; <PAUS> = 127; <LWIN> = 133; <RWIN> = 134; <COMP> = 135;\
  };\
  xkb_types \"thingos\" {\
    virtual_modifiers NumLock,LevelThree;\
    type \"ONE_LEVEL\" {\
      modifiers = none;\
      map[none] = Level1;\
      level_name[Level1] = \"Any\";\
    };\
    type \"TWO_LEVEL\" {\
      modifiers = Shift;\
      map[none] = Level1;\
      map[Shift] = Level2;\
      level_name[Level1] = \"Base\";\
      level_name[Level2] = \"Shift\";\
    };\
    type \"ALPHABETIC\" {\
      modifiers = Shift+Lock;\
      map[none] = Level1;\
      map[Shift] = Level2;\
      map[Lock] = Level2;\
      map[Shift+Lock] = Level1;\
      level_name[Level1] = \"Base\";\
      level_name[Level2] = \"Caps\";\
    };\
    type \"KEYPAD\" {\
      modifiers = Shift+NumLock;\
      map[none] = Level1;\
      map[NumLock] = Level2;\
      map[Shift+NumLock] = Level1;\
      map[Shift] = Level2;\
      level_name[Level1] = \"Base\";\
      level_name[Level2] = \"Number\";\
    };\
    type \"FOUR_LEVEL\" {\
      modifiers = Shift+LevelThree;\
      map[none] = Level1;\
      map[Shift] = Level2;\
      map[LevelThree] = Level3;\
      map[Shift+LevelThree] = Level4;\
      level_name[Level1] = \"Base\";\
      level_name[Level2] = \"Shift\";\
      level_name[Level3] = \"Alt Base\";\
      level_name[Level4] = \"Shift Alt\";\
    };\
    type \"FOUR_LEVEL_ALPHABETIC\" {\
      modifiers = Shift+Lock+LevelThree;\
      map[none] = Level1;\
      map[Shift] = Level2;\
      map[Lock] = Level2;\
      map[Shift+Lock] = Level1;\
      map[LevelThree] = Level3;\
      map[Shift+LevelThree] = Level4;\
      map[Lock+LevelThree] = Level4;\
      map[Shift+Lock+LevelThree] = Level3;\
      level_name[Level1] = \"Base\";\
      level_name[Level2] = \"Caps\";\
      level_name[Level3] = \"Alt Base\";\
      level_name[Level4] = \"Shift Alt\";\
    };\
  };\
  xkb_compat \"thingos\" {\
    virtual_modifiers NumLock,LevelThree;\
    interpret.useModMapMods = AnyLevel;\
    interpret.repeat = False;\
    interpret.locking = False;\
    interpret ISO_Level3_Shift+AnyOf(all) {\
      action = SetMods(modifiers=LevelThree,clearLocks);\
    };\
    interpret Any+AnyOf(all) {\
      action = SetMods(modifiers=modMapMods,clearLocks);\
    };\
    indicator \"Caps Lock\" {\
      !allowExplicit;\
      whichModState = locked;\
      modifiers = Lock;\
    };\
    indicator \"Num Lock\" {\
      !allowExplicit;\
      whichModState = locked;\
      virtualMods = NumLock;\
    };\
  };\
  xkb_symbols \"thingos_us_intl\" {\
    name[Group1] = \"English (US, intl., with dead keys)\";\
    key <ESC>  { [ Escape ] };\
    key <AE01> { type[Group1] = \"FOUR_LEVEL\", [ 1, exclam, onesuperior, exclamdown ] };\
    key <AE02> { type[Group1] = \"FOUR_LEVEL\", [ 2, at, twosuperior, onehalf ] };\
    key <AE03> { type[Group1] = \"FOUR_LEVEL\", [ 3, numbersign, threesuperior, sterling ] };\
    key <AE04> { type[Group1] = \"FOUR_LEVEL\", [ 4, dollar, EuroSign, cent ] };\
    key <AE05> { type[Group1] = \"TWO_LEVEL\", [ 5, percent ] };\
    key <AE06> { type[Group1] = \"FOUR_LEVEL\", [ dead_circumflex, 6, onequarter, threequarters ] };\
    key <AE07> { type[Group1] = \"TWO_LEVEL\", [ 7, ampersand ] };\
    key <AE08> { type[Group1] = \"TWO_LEVEL\", [ 8, asterisk ] };\
    key <AE09> { type[Group1] = \"TWO_LEVEL\", [ 9, parenleft ] };\
    key <AE10> { type[Group1] = \"TWO_LEVEL\", [ 0, parenright ] };\
    key <AE11> { type[Group1] = \"TWO_LEVEL\", [ minus, underscore ] };\
    key <AE12> { type[Group1] = \"FOUR_LEVEL\", [ equal, plus, dead_cedilla, dead_ogonek ] };\
    key <BKSP> { [ BackSpace, BackSpace ] };\
    key <TAB>  { [ Tab, ISO_Left_Tab ] };\
    key <AD01> { type[Group1] = \"ALPHABETIC\", [ q, Q ] };\
    key <AD02> { type[Group1] = \"ALPHABETIC\", [ w, W ] };\
    key <AD03> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ dead_acute, E, eacute, Eacute ] };\
    key <AD04> { type[Group1] = \"ALPHABETIC\", [ r, R ] };\
    key <AD05> { type[Group1] = \"ALPHABETIC\", [ t, T ] };\
    key <AD06> { type[Group1] = \"ALPHABETIC\", [ y, Y ] };\
    key <AD07> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ dead_diaeresis, U, udiaeresis, Udiaeresis ] };\
    key <AD08> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ i, I, iacute, Iacute ] };\
    key <AD09> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ o, O, oacute, Oacute ] };\
    key <AD10> { type[Group1] = \"ALPHABETIC\", [ p, P ] };\
    key <AD11> { type[Group1] = \"TWO_LEVEL\", [ dead_diaeresis, dead_circumflex ] };\
    key <AD12> { type[Group1] = \"TWO_LEVEL\", [ dead_tilde, dead_grave ] };\
    key <RTRN> { [ Return ] };\
    key <LCTL> { [ Control_L ] };\
    key <AC01> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ a, A, aacute, Aacute ] };\
    key <AC02> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ s, S, ssharp, section ] };\
    key <AC03> { type[Group1] = \"ALPHABETIC\", [ d, D ] };\
    key <AC04> { type[Group1] = \"ALPHABETIC\", [ f, F ] };\
    key <AC05> { type[Group1] = \"ALPHABETIC\", [ g, G ] };\
    key <AC06> { type[Group1] = \"ALPHABETIC\", [ h, H ] };\
    key <AC07> { type[Group1] = \"ALPHABETIC\", [ j, J ] };\
    key <AC08> { type[Group1] = \"ALPHABETIC\", [ k, K ] };\
    key <AC09> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ l, L, oslash, Oslash ] };\
    key <AC10> { type[Group1] = \"FOUR_LEVEL\", [ semicolon, colon, dead_acute, dead_diaeresis ] };\
    key <AC11> { type[Group1] = \"TWO_LEVEL\", [ dead_acute, dead_diaeresis ] };\
    key <TLDE> { type[Group1] = \"TWO_LEVEL\", [ dead_grave, dead_tilde ] };\
    key <LFSH> { [ Shift_L ] };\
    key <BKSL> { type[Group1] = \"TWO_LEVEL\", [ backslash, bar ] };\
    key <AB01> { type[Group1] = \"ALPHABETIC\", [ z, Z ] };\
    key <AB02> { type[Group1] = \"ALPHABETIC\", [ x, X ] };\
    key <AB03> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ c, C, ccedilla, Ccedilla ] };\
    key <AB04> { type[Group1] = \"ALPHABETIC\", [ v, V ] };\
    key <AB05> { type[Group1] = \"ALPHABETIC\", [ b, B ] };\
    key <AB06> { type[Group1] = \"FOUR_LEVEL_ALPHABETIC\", [ n, N, ntilde, Ntilde ] };\
    key <AB07> { type[Group1] = \"ALPHABETIC\", [ m, M ] };\
    key <AB08> { type[Group1] = \"TWO_LEVEL\", [ comma, less ] };\
    key <AB09> { type[Group1] = \"TWO_LEVEL\", [ period, greater ] };\
    key <AB10> { type[Group1] = \"TWO_LEVEL\", [ slash, question ] };\
    key <RTSH> { [ Shift_R ] };\
    key <KPMU> { [ KP_Multiply, KP_Multiply ] };\
    key <LALT> { [ Alt_L, Meta_L ] };\
    key <SPCE> { [ space ] };\
    key <CAPS> { [ Caps_Lock ] };\
    key <FK01> { [ F1 ] }; key <FK02> { [ F2 ] }; key <FK03> { [ F3 ] };\
    key <FK04> { [ F4 ] }; key <FK05> { [ F5 ] }; key <FK06> { [ F6 ] };\
    key <FK07> { [ F7 ] }; key <FK08> { [ F8 ] }; key <FK09> { [ F9 ] };\
    key <FK10> { [ F10 ] }; key <FK11> { [ F11 ] }; key <FK12> { [ F12 ] };\
    key <NMLK> { [ Num_Lock ] };\
    key <SCLK> { [ Scroll_Lock ] };\
    key <KP7>  { type[Group1] = \"KEYPAD\", [ KP_Home,   KP_7 ] };\
    key <KP8>  { type[Group1] = \"KEYPAD\", [ KP_Up,     KP_8 ] };\
    key <KP9>  { type[Group1] = \"KEYPAD\", [ KP_Prior,  KP_9 ] };\
    key <KPSU> { [ KP_Subtract, KP_Subtract ] };\
    key <KP4>  { type[Group1] = \"KEYPAD\", [ KP_Left,   KP_4 ] };\
    key <KP5>  { type[Group1] = \"KEYPAD\", [ KP_Begin,  KP_5 ] };\
    key <KP6>  { type[Group1] = \"KEYPAD\", [ KP_Right,  KP_6 ] };\
    key <KPAD> { [ KP_Add, KP_Add ] };\
    key <KP1>  { type[Group1] = \"KEYPAD\", [ KP_End,    KP_1 ] };\
    key <KP2>  { type[Group1] = \"KEYPAD\", [ KP_Down,   KP_2 ] };\
    key <KP3>  { type[Group1] = \"KEYPAD\", [ KP_Next,   KP_3 ] };\
    key <KP0>  { type[Group1] = \"KEYPAD\", [ KP_Insert, KP_0 ] };\
    key <KPDL> { type[Group1] = \"KEYPAD\", [ KP_Delete, KP_Decimal ] };\
    key <LSGT> { type[Group1] = \"TWO_LEVEL\", [ less, greater ] };\
    key <KPEN> { [ KP_Enter, KP_Enter ] };\
    key <RCTL> { [ Control_R ] };\
    key <KPDV> { [ KP_Divide, KP_Divide ] };\
    key <PRSC> { [ Print ] };\
    key <RALT> { type[Group1] = \"TWO_LEVEL\", [ ISO_Level3_Shift, Multi_key ] };\
    key <HOME> { [ Home ] }; key <UP>   { [ Up ] }; key <PGUP> { [ Prior ] };\
    key <LEFT> { [ Left ] }; key <RGHT> { [ Right ] };\
    key <END>  { [ End ] };  key <DOWN> { [ Down ] }; key <PGDN> { [ Next ] };\
    key <INS>  { [ Insert ] }; key <DELE> { [ Delete ] };\
    key <PAUS> { [ Pause ] };\
    key <LWIN> { [ Super_L ] }; key <RWIN> { [ Super_R ] };\
    key <COMP> { [ Menu ] };\
    modifier_map Control { <LCTL>, <RCTL> };\
    modifier_map Shift   { <LFSH>, <RTSH> };\
    modifier_map Mod1    { <LALT> };\
    modifier_map Lock    { <CAPS> };\
    modifier_map Mod2    { <NMLK> };\
    modifier_map Mod4    { <LWIN>, <RWIN> };\
    modifier_map Mod5    { <RALT> };\
  };\
};\0";

fn send_keyboard_keymap(client: &WaylandClient, keyboard_obj: u32) {
    // Send the us-intl XKB keymap (WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1 = 1).
    // The keymap string is written into a memfd so it can be transferred to
    // the client via Wayland's ancillary fd mechanism.
    let size = US_INTL_KEYMAP.len() as u32;
    let fd = match stem::syscall::memfd_create("xkb-keymap-us-intl", size as usize) {
        Ok(fd) => fd,
        Err(_) => {
            // Fallback: send no_keymap if memfd creation fails.
            blossom_warn!("wayland-server: memfd_create for XKB keymap failed, sending no_keymap");
            let mut payload = Vec::new();
            payload.extend_from_slice(&0u32.to_ne_bytes());
            payload.extend_from_slice(&0u32.to_ne_bytes());
            client.send_with_fds(keyboard_obj, 0, &payload, &[]);
            return;
        }
    };
    let _ = stem::syscall::vfs::vfs_write(fd, US_INTL_KEYMAP);
    let mut payload = Vec::new();
    payload.extend_from_slice(&1u32.to_ne_bytes()); // WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1
    payload.extend_from_slice(&size.to_ne_bytes());
    blossom_debug!(
        "wayland-server: sending us-intl XKB keymap to keyboard obj={} size={}",
        keyboard_obj,
        size
    );
    client.send_with_fds(keyboard_obj, 0, &payload, &[fd]);
    // Close our copy of the fd; the client received its own copy via sendmsg.
    let _ = stem::syscall::vfs::vfs_close(fd);
}

fn send_keyboard_repeat_info(client: &WaylandClient, keyboard_obj: u32) {
    let mut payload = Vec::new();
    payload.extend_from_slice(&25i32.to_ne_bytes());
    payload.extend_from_slice(&600i32.to_ne_bytes());
    client.send(keyboard_obj, 5, &payload);
}

// ── wl_output ────────────────────────────────────────────────────────────────

/// `wl_output.release` request opcode (version 3+).
const WL_OUTPUT_RELEASE: u16 = 0;

fn dispatch_output(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    // wl_output.release is a version-3 destructor; accept it gracefully even
    // though we only advertise version 2 (some clients send it anyway).
    if msg.opcode == WL_OUTPUT_RELEASE {
        client.destroy(obj_id);
    }
    vec![]
}

/// Send the initial burst of `wl_output` events to a newly-bound output object.
///
/// Sends (in order): `geometry`, `mode`, `scale` (v2), `done` (v2).
pub(crate) fn send_output_events(
    client: &WaylandClient,
    output_obj: u32,
    info: &crate::display::OutputInfo,
) {
    use crate::wayland::wire::encode_string;

    // wl_output.geometry (opcode 0)
    // x, y, physical_width_mm, physical_height_mm, subpixel, make, model, transform
    let mut geom = Vec::new();
    geom.extend_from_slice(&0i32.to_ne_bytes()); // x = 0
    geom.extend_from_slice(&0i32.to_ne_bytes()); // y = 0
    geom.extend_from_slice(&0i32.to_ne_bytes()); // physical_width_mm = 0 (unknown)
    geom.extend_from_slice(&0i32.to_ne_bytes()); // physical_height_mm = 0 (unknown)
    geom.extend_from_slice(&0i32.to_ne_bytes()); // subpixel = WL_OUTPUT_SUBPIXEL_UNKNOWN
    geom.extend_from_slice(&encode_string("ThingOS"));
    geom.extend_from_slice(&encode_string("Virtual"));
    geom.extend_from_slice(&0i32.to_ne_bytes()); // transform = WL_OUTPUT_TRANSFORM_NORMAL
    client.send(output_obj, 0, &geom);

    // wl_output.mode (opcode 1)
    // flags, width, height, refresh_mHz
    let mut mode = Vec::new();
    mode.extend_from_slice(&3u32.to_ne_bytes()); // WL_OUTPUT_MODE_CURRENT(1) | WL_OUTPUT_MODE_PREFERRED(2)
    mode.extend_from_slice(&(info.width as i32).to_ne_bytes());
    mode.extend_from_slice(&(info.height as i32).to_ne_bytes());
    mode.extend_from_slice(&(info.refresh_mhz as i32).to_ne_bytes());
    client.send(output_obj, 1, &mode);

    // wl_output.scale (opcode 3, version 2+)
    // factor: int = 1 (no HiDPI scaling)
    client.send(output_obj, 3, &1i32.to_ne_bytes());

    // wl_output.done (opcode 2, version 2+)
    // Signals that all output properties have been sent.
    client.send(output_obj, 2, &[]);
}

// ── wl_compositor ─────────────────────────────────────────────────────────────

const WL_COMPOSITOR_CREATE_SURFACE: u16 = 0;
const WL_COMPOSITOR_CREATE_REGION: u16 = 1;

fn dispatch_compositor(
    msg: &WireMsg,
    client: &mut WaylandClient,
    next_surface_key: &mut u32,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_COMPOSITOR_CREATE_SURFACE => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            // Allocate a bloom_surface_id synchronously via port.
            let bloom_surface_id = alloc_bloom_surface(cmd_write);
            *next_surface_key += 1;
            client.insert(
                new_id,
                ObjectEntry::Surface {
                    bloom_surface_id,
                    xdg_surface_obj: None,
                    layer_surface_obj: None,
                    pending_buffer: None,
                    pending_damage: None,
                    pending_frame_cb: None,
                    pending_presentation_feedback: alloc::vec::Vec::new(),
                    subsurface_obj: None,
                    subsurface_children: alloc::vec::Vec::new(),
                    pending_opaque_region: None,
                    pending_input_region: None,
                },
            );
        }
        WL_COMPOSITOR_CREATE_REGION => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                blossom_debug!("wayland-server: wl_compositor create_region id={}", new_id);
                client.insert(new_id, ObjectEntry::Region { rects: alloc::vec::Vec::new() });
            }
        }
        _ => {}
    }
    vec![]
}

/// Synchronously allocate a bloom scene surface via the command port.
///
/// Sends [`ipc::WCMD_CREATE_SURFACE`] with a one-shot reply port, blocks until
/// the main thread replies with the `bloom_surface_id`.
fn alloc_bloom_surface(cmd_write: u32) -> u32 {
    use stem::syscall::{port_create, port_recv, port_send_all};
    let (reply_write, reply_read) = match port_create(16) {
        Ok(p) => p,
        Err(_) => return 0,
    };
    let msg = ipc::encode_create_surface(reply_write);
    let _ = port_send_all(cmd_write, &msg);
    let mut buf = [0u8; 4];
    let _ = port_recv(reply_read, &mut buf);
    // Close the reply port handles (best-effort).
    use stem::syscall::vfs::vfs_close;
    if let Ok(fd) = stem::syscall::vfs::vfs_handle_from_port(reply_write) {
        let _ = vfs_close(fd);
    }
    if let Ok(fd) = stem::syscall::vfs::vfs_handle_from_port(reply_read) {
        let _ = vfs_close(fd);
    }
    u32::from_ne_bytes(buf)
}

// ── wl_shm ────────────────────────────────────────────────────────────────────

const WL_SHM_CREATE_POOL: u16 = 0;

fn dispatch_shm(msg: &WireMsg, client: &mut WaylandClient) -> Vec<Vec<u8>> {
    if msg.opcode != WL_SHM_CREATE_POOL {
        return vec![];
    }
    // create_pool(new_id: new_id<wl_shm_pool>, fd: fd, size: int)
    // The current Thing-OS socket path transfers `fd` through sendmsg/recvmsg
    // ancillary handles.  Older demos sent the handle inline; keep accepting
    // that layout so stale clients fail gracefully instead of desynchronizing.
    let new_id = match read_u32(&msg.data, 0) {
        Some(id) => id,
        None => return vec![],
    };
    let (handle, size) = if let Some(fd) = client.pending_fds.pop_front() {
        (fd, read_u32(&msg.data, 4).unwrap_or(0))
    } else {
        (read_u32(&msg.data, 4).unwrap_or(0), read_u32(&msg.data, 8).unwrap_or(0))
    };
    client.insert(new_id, ObjectEntry::ShmPool { handle, size });
    vec![]
}

fn wl_shm_format_to_pixel(format: u32) -> Option<PixelFormat> {
    match format {
        0 => Some(PixelFormat::Bgra8888), // WL_SHM_FORMAT_ARGB8888
        1 => Some(PixelFormat::Bgrx8888), // WL_SHM_FORMAT_XRGB8888
        _ => None,
    }
}

fn drm_format_to_pixel(format: u32) -> Option<PixelFormat> {
    match format {
        DRM_FORMAT_ARGB8888 => Some(PixelFormat::Bgra8888),
        DRM_FORMAT_XRGB8888 => Some(PixelFormat::Bgrx8888),
        _ => None,
    }
}

fn pixel_to_drm_format(format: PixelFormat) -> Option<u32> {
    match format {
        PixelFormat::Bgra8888 => Some(DRM_FORMAT_ARGB8888),
        PixelFormat::Bgrx8888 => Some(DRM_FORMAT_XRGB8888),
        _ => None,
    }
}

fn output_supports_pixel(output: &crate::display::OutputInfo, format: PixelFormat) -> bool {
    output.supported_formats & (1u64 << (format as u8)) != 0
}

// ── zwp_linux_dmabuf_v1 ──────────────────────────────────────────────────────

const ZWP_LINUX_DMABUF_DESTROY: u16 = 0;
const ZWP_LINUX_DMABUF_CREATE_PARAMS: u16 = 1;

fn dispatch_dmabuf(msg: &WireMsg, client: &mut WaylandClient) -> Vec<Vec<u8>> {
    match msg.opcode {
        ZWP_LINUX_DMABUF_DESTROY => {
            client.destroy(msg.object_id);
        }
        ZWP_LINUX_DMABUF_CREATE_PARAMS => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client
                    .insert(new_id, ObjectEntry::DmabufParams { used: false, planes: Vec::new() });
            }
        }
        _ => {}
    }
    vec![]
}

fn send_dmabuf_formats(
    client: &WaylandClient,
    dmabuf_obj: u32,
    output: &crate::display::OutputInfo,
) {
    for format in [PixelFormat::Bgra8888, PixelFormat::Bgrx8888] {
        if !output_supports_pixel(output, format) {
            continue;
        }
        let Some(drm_format) = pixel_to_drm_format(format) else {
            continue;
        };
        client.send(dmabuf_obj, 0, &drm_format.to_ne_bytes());

        let mut payload = Vec::new();
        payload.extend_from_slice(&drm_format.to_ne_bytes());
        payload.extend_from_slice(&((DRM_FORMAT_MOD_LINEAR >> 32) as u32).to_ne_bytes());
        payload.extend_from_slice(&(DRM_FORMAT_MOD_LINEAR as u32).to_ne_bytes());
        client.send(dmabuf_obj, 1, &payload);
    }
}

const ZWP_LINUX_BUFFER_PARAMS_DESTROY: u16 = 0;
const ZWP_LINUX_BUFFER_PARAMS_ADD: u16 = 1;
const ZWP_LINUX_BUFFER_PARAMS_CREATE: u16 = 2;
const ZWP_LINUX_BUFFER_PARAMS_CREATE_IMMED: u16 = 3;

fn dispatch_dmabuf_params(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    output: &crate::display::OutputInfo,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        ZWP_LINUX_BUFFER_PARAMS_DESTROY => {
            client.destroy(obj_id);
        }
        ZWP_LINUX_BUFFER_PARAMS_ADD => {
            let fd = match client.pending_fds.pop_front() {
                Some(fd) => fd,
                None => {
                    client.send_protocol_error(obj_id, 6, "dmabuf add missing fd");
                    return vec![];
                }
            };
            let plane_idx = read_u32(&msg.data, 0).unwrap_or(0);
            let offset = read_u32(&msg.data, 4).unwrap_or(0);
            let stride = read_u32(&msg.data, 8).unwrap_or(0);
            let modifier_hi = read_u32(&msg.data, 12).unwrap_or(0);
            let modifier_lo = read_u32(&msg.data, 16).unwrap_or(0);
            let modifier = ((modifier_hi as u64) << 32) | modifier_lo as u64;

            let error = match client.objects.get_mut(&obj_id) {
                Some(ObjectEntry::DmabufParams { used, planes }) => {
                    if *used {
                        Some((0, "dmabuf params already used"))
                    } else if plane_idx >= 4 {
                        Some((1, "dmabuf plane index out of range"))
                    } else if planes.iter().any(|plane| plane.plane_idx == plane_idx) {
                        Some((2, "dmabuf plane already set"))
                    } else {
                        planes.push(DmabufPlane { fd, plane_idx, offset, stride, modifier });
                        None
                    }
                }
                _ => return vec![],
            };
            if let Some((code, message)) = error {
                client.send_protocol_error(obj_id, code, message);
                return vec![];
            }
        }
        ZWP_LINUX_BUFFER_PARAMS_CREATE | ZWP_LINUX_BUFFER_PARAMS_CREATE_IMMED => {
            let new_id = read_u32(&msg.data, 0).unwrap_or(0);
            let width = read_i32(&msg.data, 4).unwrap_or(0);
            let height = read_i32(&msg.data, 8).unwrap_or(0);
            let drm_format = read_u32(&msg.data, 12).unwrap_or(0);
            let _flags = read_u32(&msg.data, 16).unwrap_or(0);
            let immediate = msg.opcode == ZWP_LINUX_BUFFER_PARAMS_CREATE_IMMED;

            match create_dmabuf_buffer(client, obj_id, new_id, width, height, drm_format, output) {
                Ok(()) => {
                    if !immediate {
                        client.send(obj_id, 0, &new_id.to_ne_bytes());
                    }
                    stem::info!(
                        "wayland-server: imported dmabuf wl_buffer={} {}x{} format=0x{:08x}",
                        new_id,
                        width,
                        height,
                        drm_format
                    );
                }
                Err(msg_text) => {
                    if immediate {
                        client.send_protocol_error(obj_id, 4, msg_text);
                    } else {
                        client.send(obj_id, 1, &[]);
                    }
                }
            }
        }
        _ => {}
    }
    vec![]
}

fn create_dmabuf_buffer(
    client: &mut WaylandClient,
    params_obj: u32,
    new_id: u32,
    width: i32,
    height: i32,
    drm_format: u32,
    output: &crate::display::OutputInfo,
) -> Result<(), &'static str> {
    if new_id == 0 || width <= 0 || height <= 0 {
        return Err("invalid dmabuf dimensions");
    }
    let pixel_format = drm_format_to_pixel(drm_format).ok_or("unsupported dmabuf format")?;
    if !output_supports_pixel(output, pixel_format) {
        return Err("dmabuf format not supported by display");
    }

    let (plane, modifier) = {
        let Some(ObjectEntry::DmabufParams { used, planes }) = client.objects.get_mut(&params_obj)
        else {
            return Err("invalid dmabuf params object");
        };
        if *used {
            return Err("dmabuf params already used");
        }
        *used = true;
        if planes.len() != 1 || planes[0].plane_idx != 0 {
            return Err("only single-plane dmabuf buffers are supported");
        }
        let plane = planes[0];
        if plane.modifier != DRM_FORMAT_MOD_LINEAR {
            return Err("only linear dmabuf modifiers are supported");
        }
        (plane, plane.modifier)
    };

    let bpp = pixel_format.bytes_per_pixel() as u32;
    let min_stride = (width as u32).saturating_mul(bpp);
    if plane.stride < min_stride {
        return Err("dmabuf stride is too small");
    }

    client.insert(
        new_id,
        ObjectEntry::Buffer {
            handle: plane.fd,
            offset: plane.offset,
            width: width as u32,
            height: height as u32,
            stride: plane.stride,
            format: pixel_format as u32,
            modifier,
        },
    );
    Ok(())
}

// ── wl_shm_pool ───────────────────────────────────────────────────────────────

const WL_SHM_POOL_CREATE_BUFFER: u16 = 0;
const WL_SHM_POOL_DESTROY: u16 = 1;
const WL_SHM_POOL_RESIZE: u16 = 2;

fn dispatch_shm_pool(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    let handle = match client.objects.get(&obj_id) {
        Some(ObjectEntry::ShmPool { handle, .. }) => *handle,
        _ => return vec![],
    };
    match msg.opcode {
        WL_SHM_POOL_CREATE_BUFFER => {
            // create_buffer(new_id, offset, width, height, stride, format)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let offset = read_u32(&msg.data, 4).unwrap_or(0);
            let width = read_u32(&msg.data, 8).unwrap_or(0);
            let height = read_u32(&msg.data, 12).unwrap_or(0);
            let stride = read_u32(&msg.data, 16).unwrap_or(0);
            let shm_format = read_u32(&msg.data, 20).unwrap_or(0);
            let Some(format) = wl_shm_format_to_pixel(shm_format).map(|fmt| fmt as u32) else {
                client.send_protocol_error(obj_id, 1, "unsupported wl_shm format");
                return vec![];
            };
            client.insert(
                new_id,
                ObjectEntry::Buffer { handle, offset, width, height, stride, format, modifier: 0 },
            );
        }
        WL_SHM_POOL_DESTROY => {
            client.destroy(obj_id);
        }
        WL_SHM_POOL_RESIZE => {
            if let Some(ObjectEntry::ShmPool { size, .. }) = client.objects.get_mut(&obj_id) {
                *size = read_u32(&msg.data, 0).unwrap_or(*size);
            }
        }
        _ => {}
    }
    vec![]
}

// ── wl_region ─────────────────────────────────────────────────────────────────

const WL_REGION_DESTROY: u16 = 0;
const WL_REGION_ADD: u16 = 1;
const WL_REGION_SUBTRACT: u16 = 2;

fn dispatch_region(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_REGION_DESTROY => {
            // wl_region.destroy — destructor; replace the entry with a tombstone.
            blossom_debug!("wayland-server: wl_region obj={} destroyed", obj_id);
            client.destroy(obj_id);
        }
        WL_REGION_ADD => {
            // add(x: int, y: int, width: int, height: int)
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
            let w = read_i32(&msg.data, 8).unwrap_or(0);
            let h = read_i32(&msg.data, 12).unwrap_or(0);
            // Ignore degenerate rects (non-positive dimensions).
            if w > 0 && h > 0 {
                if let Some(ObjectEntry::Region { rects }) = client.objects.get_mut(&obj_id) {
                    rects.push((x, y, w, h, true));
                    blossom_debug!(
                        "wayland-server: wl_region obj={} add x={} y={} w={} h={} rects={}",
                        obj_id, x, y, w, h, rects.len()
                    );
                }
            }
        }
        WL_REGION_SUBTRACT => {
            // subtract(x: int, y: int, width: int, height: int)
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
            let w = read_i32(&msg.data, 8).unwrap_or(0);
            let h = read_i32(&msg.data, 12).unwrap_or(0);
            if w > 0 && h > 0 {
                if let Some(ObjectEntry::Region { rects }) = client.objects.get_mut(&obj_id) {
                    rects.push((x, y, w, h, false));
                    blossom_debug!(
                        "wayland-server: wl_region obj={} subtract x={} y={} w={} h={} rects={}",
                        obj_id, x, y, w, h, rects.len()
                    );
                }
            }
        }
        _ => {}
    }
    vec![]
}

/// Compute the bounding rectangle of all "add" operations in a `wl_region`
/// rect list.
///
/// The `rects` slice holds `(x, y, w, h, is_add)` entries as accumulated by
/// `wl_region.add` and `wl_region.subtract`.  This function returns the
/// axis-aligned bounding box of all add entries (subtract entries are ignored
/// for the V1 single-rect approximation used by the compositor culling pass).
///
/// Returns `None` when the region contains no valid add operations.
///
/// # Coordinate clamping
///
/// Wayland permits negative surface-local coordinates in `wl_region.add`
/// (e.g. to extend an opaque area into an off-screen margin).  Because the
/// scene and compositor work in non-negative pixel space, the origin of the
/// returned rect is clamped to `(0, 0)`.  This is conservative: the reported
/// opaque rectangle may be *smaller* than the actual opaque area declared by
/// the client, but it is never *larger*, which preserves correctness.
///
/// Example: an add rect `(-10, 0, 100, 50)` covers screen columns 0..90 once
/// the negative left margin is clipped, so the returned rect is
/// `(0, 0, 90, 50)`.
fn region_bounding_rect(rects: &[(i32, i32, i32, i32, bool)]) -> Option<(u32, u32, u32, u32)> {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut found = false;

    for &(x, y, w, h, is_add) in rects {
        if is_add && w > 0 && h > 0 {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x.saturating_add(w));
            max_y = max_y.max(y.saturating_add(h));
            found = true;
        }
    }

    if !found {
        return None;
    }

    let x = min_x.max(0) as u32;
    let y = min_y.max(0) as u32;
    let max_x = max_x.max(0) as u32;
    let max_y = max_y.max(0) as u32;
    Some((x, y, max_x.saturating_sub(x), max_y.saturating_sub(y)))
}

// ── wl_buffer ─────────────────────────────────────────────────────────────────

const WL_BUFFER_DESTROY: u16 = 0;

fn dispatch_buffer(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    if msg.opcode == WL_BUFFER_DESTROY {
        client.destroy(obj_id);
    }
    vec![]
}

// ── wl_surface ────────────────────────────────────────────────────────────────

const WL_SURFACE_DESTROY: u16 = 0;
const WL_SURFACE_ATTACH: u16 = 1;
const WL_SURFACE_DAMAGE: u16 = 2;
const WL_SURFACE_FRAME: u16 = 3;
const WL_SURFACE_SET_OPAQUE_REGION: u16 = 4;
const WL_SURFACE_SET_INPUT_REGION: u16 = 5;
const WL_SURFACE_COMMIT: u16 = 6;
const WL_SURFACE_SET_BUFFER_TRANSFORM: u16 = 7;
const WL_SURFACE_SET_BUFFER_SCALE: u16 = 8;
const WL_SURFACE_DAMAGE_BUFFER: u16 = 9;

fn dispatch_surface(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let mut ipc_cmds: Vec<Vec<u8>> = vec![];

    match msg.opcode {
        WL_SURFACE_DESTROY => {
            let bloom_id = client.bloom_surface_id(obj_id).unwrap_or(0);
            // Per `wp_presentation` protocol, presentation feedback objects
            // bound to content that will never be presented (e.g. because the
            // surface is being destroyed) must be settled with `discarded`.
            // Discard both pending (not-yet-committed) and in-flight feedbacks
            // for this surface so the client never sees a leaked feedback
            // object.
            let _ = client.discard_pending_presentation_feedback(obj_id);
            if bloom_id != 0 {
                let _ = client.fire_presentation_feedbacks_discarded(bloom_id);
            }
            client.destroy(obj_id);
            if bloom_id != 0 {
                ipc_cmds.push(ipc::encode_destroy_surface(bloom_id).to_vec());
            }
        }

        WL_SURFACE_ATTACH => {
            // attach(buffer: object<wl_buffer>, x: int, y: int)
            let buf_id = read_u32(&msg.data, 0).unwrap_or(0);
            if let Some(ObjectEntry::Surface { pending_buffer, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_buffer = if buf_id == 0 { None } else { Some(buf_id) };
            }
        }

        WL_SURFACE_DAMAGE | WL_SURFACE_DAMAGE_BUFFER => {
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
            let w = read_i32(&msg.data, 8).unwrap_or(0) as u32;
            let h = read_i32(&msg.data, 12).unwrap_or(0) as u32;
            if let Some(ObjectEntry::Surface { pending_damage, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_damage = Some((x, y, w, h));
            }
        }

        WL_SURFACE_FRAME => {
            let cb_id = read_u32(&msg.data, 0).unwrap_or(0);
            if cb_id != 0 {
                blossom_debug!(
                    "wayland-server: wl_surface obj={} registered frame callback cb={}",
                    obj_id,
                    cb_id
                );
                client.insert(cb_id, ObjectEntry::Callback);
                if let Some(ObjectEntry::Surface { pending_frame_cb, .. }) =
                    client.objects.get_mut(&obj_id)
                {
                    *pending_frame_cb = Some(cb_id);
                }
            }
        }

        WL_SURFACE_SET_OPAQUE_REGION => {
            // set_opaque_region(region: object<wl_region> or null)
            let region_id = read_u32(&msg.data, 0).unwrap_or(0);
            if let Some(ObjectEntry::Surface { pending_opaque_region, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_opaque_region = if region_id == 0 { None } else { Some(region_id) };
            }
        }

        WL_SURFACE_SET_INPUT_REGION => {
            // set_input_region(region: object<wl_region> or null)
            let region_id = read_u32(&msg.data, 0).unwrap_or(0);
            if let Some(ObjectEntry::Surface { pending_input_region, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_input_region = if region_id == 0 { None } else { Some(region_id) };
            }
        }

        WL_SURFACE_SET_BUFFER_TRANSFORM
        | WL_SURFACE_SET_BUFFER_SCALE => {
            // Accepted as no-ops.
        }

        WL_SURFACE_COMMIT => {
            ipc_cmds.extend(handle_surface_commit(obj_id, client, blossom, cmd_write));
        }

        _ => {}
    }

    ipc_cmds
}

/// Handle `wl_surface.commit`: validate xdg-shell state, then issue IPC commands.
fn handle_surface_commit(
    wl_surface_obj: u32,
    client: &mut WaylandClient,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let mut out: Vec<Vec<u8>> = vec![];

    // Extract surface state.
    let (
        bloom_surface_id,
        xdg_obj,
        layer_surface_obj,
        pending_buffer,
        pending_damage,
        pending_frame_cb,
        pending_opaque_region,
        pending_input_region,
    ) = {
        match client.objects.get(&wl_surface_obj) {
            Some(ObjectEntry::Surface {
                bloom_surface_id,
                xdg_surface_obj,
                layer_surface_obj,
                pending_buffer,
                pending_damage,
                pending_frame_cb,
                pending_opaque_region,
                pending_input_region,
                ..
            }) => (
                *bloom_surface_id,
                *xdg_surface_obj,
                *layer_surface_obj,
                *pending_buffer,
                *pending_damage,
                *pending_frame_cb,
                *pending_opaque_region,
                *pending_input_region,
            ),
            _ => return out,
        }
    };

    // xdg-shell lifecycle validation.
    if let Some(xdg_id) = xdg_obj {
        let has_buffer = pending_buffer.is_some();
        match blossom.on_surface_commit(xdg_id, has_buffer) {
            Err(BlossomError::CommitBeforeAckConfigure { .. }) => {
                blossom_warn!(
                    "wayland-server: xdg_surface error: buffer committed before ack_configure (obj={})",
                    xdg_id
                );
                client.send_protocol_error(
                    xdg_id,
                    1, // xdg_surface error: unconfigured_buffer
                    "buffer committed before ack_configure",
                );
                return out;
            }
            Err(_) => return out,
            Ok(blossom_cmds) => {
                // Handle any blossom commands (e.g. SendXdgSurfaceConfigure re-sent).
                send_blossom_commands(client, &blossom_cmds, cmd_write);
            }
        }
    }

    // wlr-layer-shell lifecycle validation.  This mirrors the xdg_surface
    // configure/ack handshake but is driven by the dispatcher rather than a
    // blossom helper because layer-shell state is small and per-surface.
    if let Some(layer_id) = layer_surface_obj {
        if !on_layer_surface_commit(layer_id, client, pending_buffer.is_some(), &mut out) {
            // Protocol error already sent.
            return out;
        }
    }

    // Import and attach pending buffer.
    if let Some(buf_obj) = pending_buffer {
        if let Some(ObjectEntry::Buffer {
            handle,
            offset,
            width,
            height,
            stride,
            format,
            modifier,
        }) = client.objects.get(&buf_obj).cloned()
        {
            let key = client.alloc_buf_key();
            client.buf_key_to_obj.insert(key, buf_obj);
            out.push(
                ipc::encode_import_attach(
                    bloom_surface_id,
                    key,
                    handle,
                    width,
                    height,
                    stride,
                    format,
                    offset as u64,
                    modifier,
                )
                .to_vec(),
            );
        }
    }

    // Damage.
    if let Some((x, y, w, h)) = pending_damage {
        out.push(ipc::encode_damage(bloom_surface_id, x, y, w, h).to_vec());
    }

    // Opaque region — resolve the region object (if any) to a bounding rect
    // and emit WCMD_SET_OPAQUE_REGION so the main thread applies it atomically
    // with the buffer commit.  A null region_id (0) clears the opaque region.
    {
        // The resolved bounding rect (in surface-local coords, clamped to
        // non-negative).  `None` means either "no pending change" (when
        // `pending_opaque_region` is `None`) or "explicit clear" (when the
        // client passed a null wl_region ID or the region has no add ops).
        let resolved_opaque_rect: Option<(u32, u32, u32, u32)> = match pending_opaque_region {
            None => {
                // No pending change from the client; nothing to send.
                // (Skip emitting the IPC message entirely so that a prior
                // committed opaque region is preserved across frames.)
                None
            }
            Some(0) => {
                // Explicit null region → clear.
                None // send with has_region=0 below
            }
            Some(region_id) => {
                // Resolve the wl_region object to a bounding rect.
                match client.objects.get(&region_id) {
                    Some(ObjectEntry::Region { rects }) => {
                        region_bounding_rect(rects)
                    }
                    _ => None,
                }
            }
        };

        // If the client set a pending opaque region (even if the resolved rect
        // is None because the region was empty / already destroyed), we must
        // inform the main thread so it can update the scene state before the
        // commit lands.
        if pending_opaque_region.is_some() {
            blossom_debug!(
                "wayland-server: wl_surface obj={} commit opaque_region={:?}",
                wl_surface_obj,
                resolved_opaque_rect
            );
            out.push(
                ipc::encode_set_opaque_region(bloom_surface_id, resolved_opaque_rect).to_vec(),
            );
        }
    }

    // Input region — resolve the region object (if any) to a bounding rect
    // and emit WCMD_SET_INPUT_REGION so the main thread applies it atomically
    // with the buffer commit.  A null region_id (0) clears the input region,
    // restoring the default where the entire surface receives pointer input.
    {
        // The resolved bounding rect (in surface-local coords, clamped to
        // non-negative).  `None` means either "no pending change" (when
        // `pending_input_region` is `None`) or "explicit clear" (when the
        // client passed a null wl_region ID or the region has no add ops).
        let resolved_input_rect: Option<(u32, u32, u32, u32)> = match pending_input_region {
            None => {
                // No pending change from the client; nothing to send.
                // (Skip emitting the IPC message entirely so that a prior
                // committed input region is preserved across frames.)
                None
            }
            Some(0) => {
                // Explicit null region → clear (entire surface receives input).
                None // send with has_region=0 below
            }
            Some(region_id) => {
                // Resolve the wl_region object to a bounding rect.
                match client.objects.get(&region_id) {
                    Some(ObjectEntry::Region { rects }) => region_bounding_rect(rects),
                    _ => None,
                }
            }
        };

        // If the client set a pending input region (even if the resolved rect
        // is None because the region was empty / already destroyed), we must
        // inform the main thread so it can update the scene state before the
        // commit lands.
        if pending_input_region.is_some() {
            blossom_debug!(
                "wayland-server: wl_surface obj={} commit input_region={:?}",
                wl_surface_obj,
                resolved_input_rect
            );
            out.push(
                ipc::encode_set_input_region(bloom_surface_id, resolved_input_rect).to_vec(),
            );
        }
    }

    // Register frame callback with IPC if present.
    let has_cb = pending_frame_cb.is_some();
    let cb_key = if let Some(cb_id) = pending_frame_cb {
        // Map bloom_surface_id → cb_id so we can fire it on WEVT_FRAME_DONE.
        client.add_frame_cb(bloom_surface_id, cb_id);
        cb_id
    } else {
        0
    };

    // Move any pending wp_presentation_feedback objects from per-surface
    // pending state into the per-bloom_surface_id in-flight map so they will
    // be settled when the next FRAME_DONE arrives for this surface.
    let _flushed = client.flush_pending_presentation_feedback(wl_surface_obj, bloom_surface_id);

    // Commit.
    out.push(ipc::encode_commit(bloom_surface_id, has_cb, cb_key).to_vec());

    // Clear pending state.
    if let Some(ObjectEntry::Surface {
        pending_buffer,
        pending_damage,
        pending_frame_cb,
        pending_opaque_region,
        pending_input_region,
        ..
    }) = client.objects.get_mut(&wl_surface_obj)
    {
        *pending_buffer = None;
        *pending_damage = None;
        *pending_frame_cb = None;
        *pending_opaque_region = None;
        *pending_input_region = None;
    }

    // Atomically apply pending state of any synchronized subsurface children.
    // (`wl_subsurface` spec: in synchronized mode the subsurface state is
    // cached and applied as part of the parent's commit.)
    flush_synchronized_subsurfaces(wl_surface_obj, client, &mut out);

    out
}

// ── xdg_wm_base ───────────────────────────────────────────────────────────────

const XDG_WM_BASE_DESTROY: u16 = 0;
const XDG_WM_BASE_CREATE_POSITIONER: u16 = 1;
const XDG_WM_BASE_GET_XDG_SURFACE: u16 = 2;
const XDG_WM_BASE_PONG: u16 = 3;

fn dispatch_xdg_wm_base(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        XDG_WM_BASE_DESTROY => {
            client.destroy(obj_id);
        }
        XDG_WM_BASE_CREATE_POSITIONER => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client.insert(new_id, ObjectEntry::XdgPositioner);
            }
            blossom.create_positioner();
        }
        XDG_WM_BASE_GET_XDG_SURFACE => {
            // get_xdg_surface(new_id, surface)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let wl_surface_obj = match read_u32(&msg.data, 4) {
                Some(id) => id,
                None => return vec![],
            };
            let bloom_surface_id = match client.bloom_surface_id(wl_surface_obj) {
                Some(id) => id,
                None => {
                    client.send_protocol_error(obj_id, 0, "invalid wl_surface");
                    return vec![];
                }
            };
            // wl_surface may carry at most one role.  Refuse if the surface
            // already has the layer-surface role.
            if let Some(ObjectEntry::Surface { layer_surface_obj: Some(_), .. }) =
                client.objects.get(&wl_surface_obj)
            {
                blossom_warn!(
                    "wayland-server: get_xdg_surface refused: wl_surface={} already has layer-surface role",
                    wl_surface_obj
                );
                client.send_protocol_error(obj_id, 0, "wl_surface already has a role");
                return vec![];
            }
            let client_id = client.fd;
            match blossom.get_xdg_surface(client_id, new_id, bloom_surface_id) {
                Ok(_) => {
                    blossom_debug!(
                        "wayland-server: xdg_surface obj={} created for surface={}",
                        new_id,
                        bloom_surface_id
                    );
                    client.insert(new_id, ObjectEntry::XdgSurface { bloom_surface_id });
                    // Link the wl_surface back to this xdg_surface.
                    if let Some(ObjectEntry::Surface { xdg_surface_obj, .. }) =
                        client.objects.get_mut(&wl_surface_obj)
                    {
                        *xdg_surface_obj = Some(new_id);
                    }
                }
                Err(BlossomError::XdgSurfaceAlreadyExists { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: role conflict for surface={}",
                        bloom_surface_id
                    );
                    client.send_protocol_error(obj_id, 0, "wl_surface already has xdg_surface");
                }
                Err(_) => {}
            }
        }
        XDG_WM_BASE_PONG => {
            let serial = read_u32(&msg.data, 0).unwrap_or(0);
            blossom_debug!("wayland-server: xdg_wm_base.pong serial={} received", serial);
            blossom.pong(serial);
        }
        _ => {}
    }
    vec![]
}

// ── xdg_surface ───────────────────────────────────────────────────────────────

const XDG_SURFACE_DESTROY: u16 = 0;
const XDG_SURFACE_GET_TOPLEVEL: u16 = 1;
const XDG_SURFACE_GET_POPUP: u16 = 2;
const XDG_SURFACE_SET_WINDOW_GEOMETRY: u16 = 3;
const XDG_SURFACE_ACK_CONFIGURE: u16 = 4;

fn dispatch_xdg_surface(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let _bloom_surface_id = match client.objects.get(&obj_id) {
        Some(ObjectEntry::XdgSurface { bloom_surface_id }) => *bloom_surface_id,
        _ => return vec![],
    };
    let client_id = client.fd;

    match msg.opcode {
        XDG_SURFACE_DESTROY => {
            blossom_debug!("wayland-server: xdg_surface obj={} destroyed", obj_id);
            if let Ok(cmds) = blossom.destroy_xdg_surface(obj_id) {
                send_blossom_commands(client, &cmds, cmd_write);
            }
            client.destroy(obj_id);
        }
        XDG_SURFACE_GET_TOPLEVEL => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            match blossom.get_toplevel(client_id, obj_id, new_id) {
                Ok(cmds) => {
                    stem::info!(
                        "wayland-server: xdg_toplevel obj={} assigned to xdg_surface={}",
                        new_id,
                        obj_id
                    );
                    client.insert(new_id, ObjectEntry::XdgToplevel { xdg_surface_obj: obj_id });
                    send_blossom_commands(client, &cmds, cmd_write);
                }
                Err(BlossomError::XdgSurfaceAlreadyHasRole { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: surface={} already has a role",
                        obj_id
                    );
                    client.send_protocol_error(obj_id, 4, "xdg_surface already has a role");
                }
                Err(_) => {}
            }
        }
        XDG_SURFACE_GET_POPUP => {
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let parent = read_u32(&msg.data, 4).filter(|id| *id != 0);
            let positioner = match read_u32(&msg.data, 8) {
                Some(id) => id,
                None => return vec![],
            };
            match blossom.get_popup(client_id, obj_id, new_id, parent, positioner) {
                Ok(cmds) => {
                    stem::info!(
                        "wayland-server: xdg_popup obj={} assigned to xdg_surface={}",
                        new_id,
                        obj_id
                    );
                    client.insert(new_id, ObjectEntry::XdgPopup { xdg_surface_obj: obj_id });
                    send_blossom_commands(client, &cmds, cmd_write);
                }
                Err(BlossomError::XdgSurfaceAlreadyHasRole { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: surface={} already has a role",
                        obj_id
                    );
                    client.send_protocol_error(obj_id, 4, "xdg_surface already has a role");
                }
                Err(_) => {}
            }
        }
        XDG_SURFACE_SET_WINDOW_GEOMETRY => {
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
            let w = read_i32(&msg.data, 8).unwrap_or(0);
            let h = read_i32(&msg.data, 12).unwrap_or(0);
            let _ = blossom.set_window_geometry(obj_id, blossom::Rect { x, y, w, h });
        }
        XDG_SURFACE_ACK_CONFIGURE => {
            let serial = read_u32(&msg.data, 0).unwrap_or(0);
            match blossom.ack_configure(obj_id, serial) {
                Ok(_) => {
                    blossom_debug!(
                        "wayland-server: xdg_surface.ack_configure serial={} accepted",
                        serial
                    );
                }
                Err(BlossomError::UnknownSerial { .. }) => {
                    blossom_warn!(
                        "wayland-server: xdg_surface error: invalid serial {} in ack_configure",
                        serial
                    );
                    client.send_protocol_error(obj_id, 3, "invalid serial in ack_configure");
                }
                Err(_) => {}
            }
        }
        _ => {}
    }
    vec![]
}

// ── xdg_popup ────────────────────────────────────────────────────────────────

const XDG_POPUP_DESTROY: u16 = 0;

fn dispatch_xdg_popup(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
) -> Vec<Vec<u8>> {
    if msg.opcode == XDG_POPUP_DESTROY {
        blossom_debug!("wayland-server: xdg_popup obj={} destroyed", obj_id);
        let _ = blossom.destroy_popup(obj_id);
        client.destroy(obj_id);
    }
    vec![]
}

// ── xdg_toplevel ─────────────────────────────────────────────────────────────

const XDG_TOPLEVEL_DESTROY: u16 = 0;
const XDG_TOPLEVEL_SET_PARENT: u16 = 1;
const XDG_TOPLEVEL_SET_TITLE: u16 = 2;
const XDG_TOPLEVEL_SET_APP_ID: u16 = 3;
const XDG_TOPLEVEL_SHOW_WINDOW_MENU: u16 = 4;
const XDG_TOPLEVEL_MOVE: u16 = 5;
const XDG_TOPLEVEL_RESIZE: u16 = 6;
const XDG_TOPLEVEL_SET_MAX_SIZE: u16 = 7;
const XDG_TOPLEVEL_SET_MIN_SIZE: u16 = 8;
const XDG_TOPLEVEL_SET_MAXIMIZED: u16 = 9;
const XDG_TOPLEVEL_UNSET_MAXIMIZED: u16 = 10;
const XDG_TOPLEVEL_SET_FULLSCREEN: u16 = 11;
const XDG_TOPLEVEL_UNSET_FULLSCREEN: u16 = 12;
const XDG_TOPLEVEL_SET_MINIMIZED: u16 = 13;

fn dispatch_xdg_toplevel(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    blossom: &mut blossom::Blossom,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        XDG_TOPLEVEL_DESTROY => {
            blossom_debug!("wayland-server: xdg_toplevel obj={} destroyed", obj_id);
            let _ = blossom.destroy_toplevel(obj_id);
            client.destroy(obj_id);
        }
        XDG_TOPLEVEL_SET_PARENT => {
            let parent = read_u32(&msg.data, 0);
            let _ = blossom.set_parent(obj_id, parent.filter(|&p| p != 0));
        }
        XDG_TOPLEVEL_SET_TITLE => {
            if let Some((s, _)) = read_string(&msg.data, 0) {
                let title = String::from_utf8_lossy(s).into_owned();
                blossom_debug!("wayland-server: xdg_toplevel obj={} title=\"{}\"", obj_id, title);
                let _ = blossom.set_title(obj_id, title);
                if let Some(bloom_surface_id) = toplevel_bloom_surface(client, obj_id) {
                    let msg = ipc::encode_set_title(bloom_surface_id, &String::from_utf8_lossy(s));
                    let _ = stem::syscall::port_send_all(cmd_write, &msg);
                }
            }
        }
        XDG_TOPLEVEL_SET_APP_ID => {
            if let Some((s, _)) = read_string(&msg.data, 0) {
                let app_id = String::from_utf8_lossy(s).into_owned();
                blossom_debug!("wayland-server: xdg_toplevel obj={} app_id=\"{}\"", obj_id, app_id);
                let _ = blossom.set_app_id(obj_id, app_id);
            }
        }
        XDG_TOPLEVEL_SHOW_WINDOW_MENU => {
            let _ = blossom.show_window_menu(obj_id);
        }
        XDG_TOPLEVEL_MOVE => {
            let _ = blossom.request_move(obj_id);
        }
        XDG_TOPLEVEL_RESIZE => {
            let _ = blossom.request_resize(obj_id);
        }
        XDG_TOPLEVEL_SET_MAX_SIZE => {
            let w = read_i32(&msg.data, 0).unwrap_or(0);
            let h = read_i32(&msg.data, 4).unwrap_or(0);
            let _ = blossom.set_max_size(obj_id, blossom::Size { w, h });
        }
        XDG_TOPLEVEL_SET_MIN_SIZE => {
            let w = read_i32(&msg.data, 0).unwrap_or(0);
            let h = read_i32(&msg.data, 4).unwrap_or(0);
            let _ = blossom.set_min_size(obj_id, blossom::Size { w, h });
        }
        XDG_TOPLEVEL_SET_MAXIMIZED => {
            let _ = blossom.set_maximized(obj_id);
        }
        XDG_TOPLEVEL_UNSET_MAXIMIZED => {
            let _ = blossom.unset_maximized(obj_id);
        }
        XDG_TOPLEVEL_SET_FULLSCREEN => {
            let _ = blossom.set_fullscreen(obj_id);
        }
        XDG_TOPLEVEL_UNSET_FULLSCREEN => {
            let _ = blossom.unset_fullscreen(obj_id);
        }
        XDG_TOPLEVEL_SET_MINIMIZED => {
            let _ = blossom.set_minimized(obj_id);
        }
        _ => {}
    }
    vec![]
}

fn toplevel_bloom_surface(client: &WaylandClient, toplevel_obj: u32) -> Option<u32> {
    let xdg_surface_obj = match client.objects.get(&toplevel_obj)? {
        ObjectEntry::XdgToplevel { xdg_surface_obj } => *xdg_surface_obj,
        _ => return None,
    };
    match client.objects.get(&xdg_surface_obj)? {
        ObjectEntry::XdgSurface { bloom_surface_id } => Some(*bloom_surface_id),
        _ => None,
    }
}

// ── wl_subcompositor ─────────────────────────────────────────────────────────

const WL_SUBCOMPOSITOR_DESTROY: u16 = 0;
const WL_SUBCOMPOSITOR_GET_SUBSURFACE: u16 = 1;

/// Error code values from `wl_subcompositor.error`.
const WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE: u32 = 0;

fn dispatch_subcompositor(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_SUBCOMPOSITOR_DESTROY => {
            client.destroy(obj_id);
        }
        WL_SUBCOMPOSITOR_GET_SUBSURFACE => {
            // get_subsurface(id: new_id<wl_subsurface>, surface: object<wl_surface>,
            //                parent:  object<wl_surface>)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let surface_obj = read_u32(&msg.data, 4).unwrap_or(0);
            let parent_obj = read_u32(&msg.data, 8).unwrap_or(0);

            // The surface must exist, must not already be a subsurface, and
            // must not be the parent of itself.
            if surface_obj == 0 || parent_obj == 0 || surface_obj == parent_obj {
                client.send_protocol_error(
                    obj_id,
                    WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE,
                    "invalid surface for get_subsurface",
                );
                return vec![];
            }

            // Validate child is a wl_surface and not yet a subsurface and not
            // already an xdg_surface or layer_surface (Wayland spec: a
            // wl_surface may carry at most one role).
            let (child_is_surface, child_already_sub, child_already_xdg, child_already_layer) =
                match client.objects.get(&surface_obj) {
                    Some(ObjectEntry::Surface {
                        subsurface_obj,
                        xdg_surface_obj,
                        layer_surface_obj,
                        ..
                    }) => (
                        true,
                        subsurface_obj.is_some(),
                        xdg_surface_obj.is_some(),
                        layer_surface_obj.is_some(),
                    ),
                    _ => (false, false, false, false),
                };
            let parent_is_surface =
                matches!(client.objects.get(&parent_obj), Some(ObjectEntry::Surface { .. }));
            if !child_is_surface
                || !parent_is_surface
                || child_already_sub
                || child_already_xdg
                || child_already_layer
            {
                client.send_protocol_error(
                    obj_id,
                    WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE,
                    "surface already has the subsurface role or is invalid",
                );
                return vec![];
            }

            // Register the subsurface object.  Spec defaults: position (0,0),
            // synchronized mode.
            client.insert(
                new_id,
                ObjectEntry::Subsurface {
                    child_wl_surface: surface_obj,
                    parent_wl_surface: parent_obj,
                    x: 0,
                    y: 0,
                    sync: true,
                    pending_position: None,
                    pending_place: None,
                },
            );
            // Link child surface back to its subsurface object.
            if let Some(ObjectEntry::Surface { subsurface_obj, .. }) =
                client.objects.get_mut(&surface_obj)
            {
                *subsurface_obj = Some(new_id);
            }
            // Append to the parent's children stack at the top.
            if let Some(ObjectEntry::Surface { subsurface_children, .. }) =
                client.objects.get_mut(&parent_obj)
            {
                subsurface_children.push(surface_obj);
            }
        }
        _ => {}
    }
    vec![]
}

// ── wl_subsurface ────────────────────────────────────────────────────────────

const WL_SUBSURFACE_DESTROY: u16 = 0;
const WL_SUBSURFACE_SET_POSITION: u16 = 1;
const WL_SUBSURFACE_PLACE_ABOVE: u16 = 2;
const WL_SUBSURFACE_PLACE_BELOW: u16 = 3;
const WL_SUBSURFACE_SET_SYNC: u16 = 4;
const WL_SUBSURFACE_SET_DESYNC: u16 = 5;

/// Error code values from `wl_subsurface.error`.
const WL_SUBSURFACE_ERROR_BAD_SURFACE: u32 = 0;

fn dispatch_subsurface(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    cmd_write: u32,
) -> Vec<Vec<u8>> {
    let mut ipc_cmds: Vec<Vec<u8>> = vec![];

    match msg.opcode {
        WL_SUBSURFACE_DESTROY => {
            destroy_subsurface(obj_id, client, &mut ipc_cmds, cmd_write);
        }
        WL_SUBSURFACE_SET_POSITION => {
            let x = read_i32(&msg.data, 0).unwrap_or(0);
            let y = read_i32(&msg.data, 4).unwrap_or(0);
            let sync = subsurface_is_sync(client, obj_id);
            if let Some(ObjectEntry::Subsurface { pending_position, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_position = Some((x, y));
            }
            if !sync {
                // Desync: apply immediately.
                if let Some((x, y)) = take_pending_position(client, obj_id) {
                    apply_subsurface_position(client, obj_id, x, y);
                    if let Some(cmd) = subsurface_state_command(client, obj_id) {
                        let _ = stem::syscall::port_send_all(cmd_write, &cmd);
                    }
                }
            }
        }
        WL_SUBSURFACE_PLACE_ABOVE | WL_SUBSURFACE_PLACE_BELOW => {
            let sibling = read_u32(&msg.data, 0).unwrap_or(0);
            let above = msg.opcode == WL_SUBSURFACE_PLACE_ABOVE;
            let sync = subsurface_is_sync(client, obj_id);
            if let Some(ObjectEntry::Subsurface { pending_place, .. }) =
                client.objects.get_mut(&obj_id)
            {
                *pending_place = Some((sibling, above));
            }
            if !sync {
                if let Some((sibling, above)) = take_pending_place(client, obj_id) {
                    if !apply_subsurface_place(client, obj_id, sibling, above) {
                        client.send_protocol_error(
                            obj_id,
                            WL_SUBSURFACE_ERROR_BAD_SURFACE,
                            "place_above/below sibling not a sibling subsurface",
                        );
                    } else if let Some(cmd) = subsurface_state_command(client, obj_id) {
                        let _ = stem::syscall::port_send_all(cmd_write, &cmd);
                    }
                }
            }
        }
        WL_SUBSURFACE_SET_SYNC => {
            if let Some(ObjectEntry::Subsurface { sync, .. }) = client.objects.get_mut(&obj_id) {
                *sync = true;
            }
        }
        WL_SUBSURFACE_SET_DESYNC => {
            if let Some(ObjectEntry::Subsurface { sync, .. }) = client.objects.get_mut(&obj_id) {
                *sync = false;
            }
            // Flush any pending state immediately.
            if let Some((x, y)) = take_pending_position(client, obj_id) {
                apply_subsurface_position(client, obj_id, x, y);
            }
            if let Some((sibling, above)) = take_pending_place(client, obj_id) {
                if !apply_subsurface_place(client, obj_id, sibling, above) {
                    client.send_protocol_error(
                        obj_id,
                        WL_SUBSURFACE_ERROR_BAD_SURFACE,
                        "place_above/below sibling not a sibling subsurface",
                    );
                }
            }
            if let Some(cmd) = subsurface_state_command(client, obj_id) {
                let _ = stem::syscall::port_send_all(cmd_write, &cmd);
            }
        }
        _ => {}
    }

    ipc_cmds
}

/// Tear down a subsurface, removing it from the parent's child list and
/// detaching the role from the child wl_surface.  An IPC notification is
/// emitted so the scene can drop the parent/offset relationship.
fn destroy_subsurface(
    obj_id: u32,
    client: &mut WaylandClient,
    _ipc_cmds: &mut Vec<Vec<u8>>,
    cmd_write: u32,
) {
    let (child_wl_surface, parent_wl_surface) = match client.objects.get(&obj_id) {
        Some(ObjectEntry::Subsurface { child_wl_surface, parent_wl_surface, .. }) => {
            (*child_wl_surface, *parent_wl_surface)
        }
        _ => {
            client.destroy(obj_id);
            return;
        }
    };
    let child_bloom = client.bloom_surface_id(child_wl_surface).unwrap_or(0);

    // Detach role from child.
    if let Some(ObjectEntry::Surface { subsurface_obj, .. }) =
        client.objects.get_mut(&child_wl_surface)
    {
        if *subsurface_obj == Some(obj_id) {
            *subsurface_obj = None;
        }
    }
    // Remove from parent's children stack.
    if let Some(ObjectEntry::Surface { subsurface_children, .. }) =
        client.objects.get_mut(&parent_wl_surface)
    {
        subsurface_children.retain(|&id| id != child_wl_surface);
    }
    client.destroy(obj_id);

    if child_bloom != 0 {
        let cmd = ipc::encode_set_subsurface(child_bloom, 0, 0, 0, 0);
        let _ = stem::syscall::port_send_all(cmd_write, &cmd);
    }
}

fn subsurface_is_sync(client: &WaylandClient, obj_id: u32) -> bool {
    match client.objects.get(&obj_id) {
        Some(ObjectEntry::Subsurface { sync, .. }) => *sync,
        _ => true,
    }
}

fn take_pending_position(client: &mut WaylandClient, obj_id: u32) -> Option<(i32, i32)> {
    match client.objects.get_mut(&obj_id) {
        Some(ObjectEntry::Subsurface { pending_position, .. }) => pending_position.take(),
        _ => None,
    }
}

fn take_pending_place(client: &mut WaylandClient, obj_id: u32) -> Option<(u32, bool)> {
    match client.objects.get_mut(&obj_id) {
        Some(ObjectEntry::Subsurface { pending_place, .. }) => pending_place.take(),
        _ => None,
    }
}

fn apply_subsurface_position(client: &mut WaylandClient, obj_id: u32, new_x: i32, new_y: i32) {
    if let Some(ObjectEntry::Subsurface { x, y, .. }) = client.objects.get_mut(&obj_id) {
        *x = new_x;
        *y = new_y;
    }
}

/// Reorder the parent's `subsurface_children` list per `place_above` / `place_below`.
///
/// Returns `false` when the requested sibling is not actually a sibling of the
/// child being moved (Wayland spec calls for a `bad_surface` protocol error).
/// `sibling == 0` is interpreted as "relative to the parent itself" (place at
/// top or bottom of the children stack).
fn apply_subsurface_place(
    client: &mut WaylandClient,
    obj_id: u32,
    sibling: u32,
    above: bool,
) -> bool {
    let (child, parent) = match client.objects.get(&obj_id) {
        Some(ObjectEntry::Subsurface { child_wl_surface, parent_wl_surface, .. }) => {
            (*child_wl_surface, *parent_wl_surface)
        }
        _ => return false,
    };
    let Some(ObjectEntry::Surface { subsurface_children, .. }) = client.objects.get_mut(&parent)
    else {
        return false;
    };

    let Some(child_idx) = subsurface_children.iter().position(|&c| c == child) else {
        return false;
    };
    subsurface_children.remove(child_idx);

    if sibling == 0 || sibling == parent {
        // Relative to parent: above => top, below => bottom.
        if above {
            subsurface_children.push(child);
        } else {
            subsurface_children.insert(0, child);
        }
        return true;
    }

    let Some(sibling_idx) = subsurface_children.iter().position(|&c| c == sibling) else {
        // Sibling not part of this parent's children: protocol error.  Restore
        // child to the top to keep state consistent.
        subsurface_children.push(child);
        return false;
    };
    let insert_at = if above { sibling_idx + 1 } else { sibling_idx };
    subsurface_children.insert(insert_at, child);
    true
}

/// Build a `WCMD_SET_SUBSURFACE` for the given wl_subsurface object reflecting
/// its current parent/position/stacking state.
fn subsurface_state_command(client: &WaylandClient, obj_id: u32) -> Option<[u8; 24]> {
    let (child_wl_surface, parent_wl_surface, x, y) = match client.objects.get(&obj_id)? {
        ObjectEntry::Subsurface { child_wl_surface, parent_wl_surface, x, y, .. } => {
            (*child_wl_surface, *parent_wl_surface, *x, *y)
        }
        _ => return None,
    };
    let child_bloom = client.bloom_surface_id(child_wl_surface)?;
    let parent_bloom = client.bloom_surface_id(parent_wl_surface)?;
    let z_above = subsurface_z_above(client, parent_wl_surface, child_wl_surface);
    Some(ipc::encode_set_subsurface(child_bloom, parent_bloom, x, y, z_above))
}

/// Return the stacking offset of `child_wl_surface` relative to its parent.
///
/// Children are numbered 1..=N from bottom-to-top, so the first child is
/// `+1` (just above the parent) and the topmost child is `+N`.  Returns `0`
/// when the child is not currently in the parent's stack (caller filters).
fn subsurface_z_above(
    client: &WaylandClient,
    parent_wl_surface: u32,
    child_wl_surface: u32,
) -> i32 {
    let Some(ObjectEntry::Surface { subsurface_children, .. }) =
        client.objects.get(&parent_wl_surface)
    else {
        return 0;
    };
    match subsurface_children.iter().position(|&c| c == child_wl_surface) {
        Some(idx) => (idx as i32) + 1,
        None => 0,
    }
}

/// On parent `wl_surface.commit`, atomically apply pending state of every
/// synchronized subsurface child and emit IPC updates so the scene matches.
fn flush_synchronized_subsurfaces(
    parent_wl_surface: u32,
    client: &mut WaylandClient,
    out: &mut Vec<Vec<u8>>,
) {
    // Snapshot current children list (cloned to release the borrow).
    let children: Vec<u32> = match client.objects.get(&parent_wl_surface) {
        Some(ObjectEntry::Surface { subsurface_children, .. }) => subsurface_children.clone(),
        _ => return,
    };
    // Track which subsurface objects had their stacking applied so we can
    // re-emit z_above for siblings whose ordinal might have shifted.
    let mut any_place = false;
    for child_wl_surface in &children {
        let sub_obj = match client.objects.get(child_wl_surface) {
            Some(ObjectEntry::Surface { subsurface_obj: Some(s), .. }) => *s,
            _ => continue,
        };
        let sync = subsurface_is_sync(client, sub_obj);
        if !sync {
            continue;
        }
        if let Some((x, y)) = take_pending_position(client, sub_obj) {
            apply_subsurface_position(client, sub_obj, x, y);
        }
        if let Some((sibling, above)) = take_pending_place(client, sub_obj) {
            let _ = apply_subsurface_place(client, sub_obj, sibling, above);
            any_place = true;
        }
    }
    // Re-emit current state for every synchronized child after the parent
    // commit so the scene picks up parent-relative offsets / stacking changes.
    let final_children: Vec<u32> = match client.objects.get(&parent_wl_surface) {
        Some(ObjectEntry::Surface { subsurface_children, .. }) => subsurface_children.clone(),
        _ => return,
    };
    for child_wl_surface in &final_children {
        let sub_obj = match client.objects.get(child_wl_surface) {
            Some(ObjectEntry::Surface { subsurface_obj: Some(s), .. }) => *s,
            _ => continue,
        };
        if !subsurface_is_sync(client, sub_obj) && !any_place {
            // Desync subsurfaces only need state push when stacking shifted.
            continue;
        }
        if let Some(cmd) = subsurface_state_command(client, sub_obj) {
            out.push(cmd.to_vec());
        }
    }
}

// ── Blossom command translation ───────────────────────────────────────────────

/// Execute blossom commands by sending Wayland events back to the client.
///
/// Commands that produce outgoing IPC to the main thread are written to
/// `cmd_write` directly.
pub fn send_blossom_commands(client: &mut WaylandClient, cmds: &[BlossomCommand], cmd_write: u32) {
    use crate::wayland::wire::encode_array;

    for cmd in cmds {
        match cmd {
            BlossomCommand::SendXdgToplevelConfigure {
                xdg_toplevel,
                width,
                height,
                states,
                ..
            } => {
                // xdg_toplevel.configure(width: int, height: int, states: array)
                // opcode 0
                let state_bytes: Vec<u8> = states
                    .iter()
                    .flat_map(|s| {
                        let v: u32 = xdg_state_atom_value(s);
                        v.to_ne_bytes().to_vec()
                    })
                    .collect();
                let mut payload = alloc::vec::Vec::new();
                payload.extend_from_slice(&(*width as i32).to_ne_bytes());
                payload.extend_from_slice(&(*height as i32).to_ne_bytes());
                payload.extend_from_slice(&encode_array(&state_bytes));
                client.send(*xdg_toplevel, 0, &payload);
            }
            BlossomCommand::SendXdgSurfaceConfigure { xdg_surface, serial, .. } => {
                // xdg_surface.configure(serial: uint)
                // opcode 0
                blossom_debug!(
                    "wayland-server: xdg_surface.configure serial={} sent to obj={}",
                    serial,
                    xdg_surface
                );
                client.send(*xdg_surface, 0, &serial.to_ne_bytes());
            }
            BlossomCommand::SendXdgPopupConfigure { xdg_popup, x, y, width, height, .. } => {
                // xdg_popup.configure(x: int, y: int, width: int, height: int)
                // opcode 0
                blossom_debug!(
                    "wayland-server: xdg_popup.configure obj={} x={} y={} width={} height={}",
                    xdg_popup,
                    x,
                    y,
                    width,
                    height
                );
                let mut payload = alloc::vec::Vec::new();
                payload.extend_from_slice(&x.to_ne_bytes());
                payload.extend_from_slice(&y.to_ne_bytes());
                payload.extend_from_slice(&width.to_ne_bytes());
                payload.extend_from_slice(&height.to_ne_bytes());
                client.send(*xdg_popup, 0, &payload);
            }
            BlossomCommand::MarkSurfaceReadyForMapping { surface } => {
                blossom_debug!("wayland-server: surface {} ready for mapping", surface);
                // No outgoing Wayland event needed; the compositor will map
                // the surface based on the commit IPC command.
            }
            BlossomCommand::SetToplevelChrome { surface, titlebar_height, frame_thickness } => {
                blossom_debug!(
                    "wayland-server: surface {} titlebar height {} frame {}",
                    surface,
                    titlebar_height,
                    frame_thickness
                );
                let msg = ipc::encode_set_chrome(*surface, *titlebar_height, *frame_thickness);
                let _ = stem::syscall::port_send_all(cmd_write, &msg);
            }
            BlossomCommand::CloseToplevel { toplevel } => {
                // xdg_toplevel.close()
                // opcode 1 (no payload)
                client.send(*toplevel, 1, &[]);
            }
            BlossomCommand::DismissPopup { xdg_popup, .. } => {
                // xdg_popup.popup_done()
                // opcode 1 (no payload, destructor)
                blossom_debug!(
                    "wayland-server: xdg_popup.popup_done sent to obj={}",
                    xdg_popup
                );
                client.send(*xdg_popup, 1, &[]);
            }
            BlossomCommand::SendPing { wm_base, serial, .. } => {
                // xdg_wm_base.ping(serial: uint)
                // opcode 0
                blossom_debug!("wayland-server: xdg_wm_base.ping serial={} sent", serial);
                client.send(*wm_base, 0, &serial.to_ne_bytes());
            }
        }
    }
}

/// Wayland numeric values for xdg_toplevel state atoms.
fn xdg_state_atom_value(atom: &blossom::XdgToplevelStateAtom) -> u32 {
    use blossom::XdgToplevelStateAtom::*;
    match atom {
        Maximized => 1,
        Fullscreen => 2,
        Resizing => 3,
        Activated => 4,
        TiledLeft => 5,
        TiledRight => 6,
        TiledTop => 7,
        TiledBottom => 8,
    }
}

// ── zwlr_layer_shell_v1 / zwlr_layer_surface_v1 ─────────────────────────────
//
// Implements the wlr-layer-shell protocol (background / bottom / top /
// overlay layers used for panels, wallpapers, lock screens, etc.).  This
// implementation is deliberately minimal:
//
// * Single-output: anchors and exclusive zones are computed against the
//   compositor's primary output (the same `OutputInfo` Bloom advertises via
//   wl_output and zwp_linux_dmabuf_v1).
// * Stacking: each layer maps to a fixed `z_order` band (see
//   `blossom::LayerShellLayer::z_order`).  Within a layer, the most recently
//   committed surface wins.
// * Configure/ack lifecycle is handled per-layer-surface (no blossom helper).
// * Geometry is computed by `blossom::compute_layer_placement` so that the
//   same logic can be unit tested without a running compositor.
//
// Bloom does not yet implement DnD-style popup attachment for layer
// surfaces, the keyboard interactivity hint, or per-layer exclusive zones
// for siblings — these are accepted on the wire as no-ops to keep simple
// clients (panels, wallpapers) functional.

/// zwlr_layer_shell_v1 request opcodes.
const ZWLR_LAYER_SHELL_GET_LAYER_SURFACE: u16 = 0;
const ZWLR_LAYER_SHELL_DESTROY: u16 = 1;
const ZWLR_LAYER_SHELL_GET_POPUP: u16 = 2;

/// zwlr_layer_shell_v1.error codes.
const ZWLR_LAYER_SHELL_ERROR_ROLE: u32 = 0;
const ZWLR_LAYER_SHELL_ERROR_INVALID_LAYER: u32 = 1;
const ZWLR_LAYER_SHELL_ERROR_ALREADY_CONSTRUCTED: u32 = 2;

fn dispatch_layer_shell(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    output: &crate::display::OutputInfo,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        ZWLR_LAYER_SHELL_DESTROY => {
            client.destroy(obj_id);
        }
        ZWLR_LAYER_SHELL_GET_LAYER_SURFACE => {
            // get_layer_surface(id: new_id, surface: object<wl_surface>,
            //                   output: object<wl_output>|null,
            //                   layer: uint, namespace: string)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let wl_surface_obj = read_u32(&msg.data, 4).unwrap_or(0);
            let _output_obj = read_u32(&msg.data, 8).unwrap_or(0);
            let layer_wire = read_u32(&msg.data, 12).unwrap_or(u32::MAX);
            // namespace string follows; we don't currently need it.

            let layer = match blossom::LayerShellLayer::from_wire(layer_wire) {
                Some(l) => l,
                None => {
                    blossom_warn!(
                        "wayland-server: zwlr_layer_shell.get_layer_surface invalid layer={}",
                        layer_wire
                    );
                    client.send_protocol_error(
                        obj_id,
                        ZWLR_LAYER_SHELL_ERROR_INVALID_LAYER,
                        "invalid layer",
                    );
                    return vec![];
                }
            };

            // Validate the wl_surface and that no other role has been assigned.
            let (bloom_surface_id, role_conflict) = match client.objects.get(&wl_surface_obj) {
                Some(ObjectEntry::Surface {
                    bloom_surface_id,
                    xdg_surface_obj,
                    layer_surface_obj,
                    subsurface_obj,
                    ..
                }) => (
                    *bloom_surface_id,
                    xdg_surface_obj.is_some()
                        || layer_surface_obj.is_some()
                        || subsurface_obj.is_some(),
                ),
                _ => {
                    client.send_protocol_error(obj_id, ZWLR_LAYER_SHELL_ERROR_ROLE, "invalid wl_surface");
                    return vec![];
                }
            };
            if role_conflict {
                blossom_warn!(
                    "wayland-server: get_layer_surface refused: wl_surface={} already has a role",
                    wl_surface_obj
                );
                client.send_protocol_error(
                    obj_id,
                    ZWLR_LAYER_SHELL_ERROR_ROLE,
                    "wl_surface already has a role",
                );
                return vec![];
            }

            // The layer-shell spec also forbids attaching a buffer before the
            // initial commit/configure handshake.  Reject if the surface
            // already has a pending buffer.
            let has_pending_buffer = matches!(
                client.objects.get(&wl_surface_obj),
                Some(ObjectEntry::Surface { pending_buffer: Some(_), .. })
            );
            if has_pending_buffer {
                client.send_protocol_error(
                    obj_id,
                    ZWLR_LAYER_SHELL_ERROR_ALREADY_CONSTRUCTED,
                    "wl_surface already has buffer attached before role",
                );
                return vec![];
            }

            client.insert(
                new_id,
                ObjectEntry::LayerSurface {
                    wl_surface_obj,
                    bloom_surface_id,
                    state: blossom::LayerSurfaceState::new(layer),
                },
            );
            // Link the wl_surface back to this layer_surface.
            if let Some(ObjectEntry::Surface { layer_surface_obj, .. }) =
                client.objects.get_mut(&wl_surface_obj)
            {
                *layer_surface_obj = Some(new_id);
            }
            blossom_debug!(
                "wayland-server: zwlr_layer_surface obj={} created for surface={} layer={:?} output={}x{}",
                new_id,
                bloom_surface_id,
                layer,
                output.width,
                output.height
            );
        }
        ZWLR_LAYER_SHELL_GET_POPUP => {
            // Layer-surface popups are not implemented in v1; accept and
            // ignore so the binding object can be cleaned up by the client.
            let _ = read_u32(&msg.data, 0);
        }
        _ => {}
    }
    vec![]
}

/// zwlr_layer_surface_v1 request opcodes.
const ZWLR_LAYER_SURFACE_SET_SIZE: u16 = 0;
const ZWLR_LAYER_SURFACE_SET_ANCHOR: u16 = 1;
const ZWLR_LAYER_SURFACE_SET_EXCLUSIVE_ZONE: u16 = 2;
const ZWLR_LAYER_SURFACE_SET_MARGIN: u16 = 3;
const ZWLR_LAYER_SURFACE_SET_KEYBOARD_INTERACTIVITY: u16 = 4;
const ZWLR_LAYER_SURFACE_GET_POPUP: u16 = 5;
const ZWLR_LAYER_SURFACE_ACK_CONFIGURE: u16 = 6;
const ZWLR_LAYER_SURFACE_DESTROY: u16 = 7;
const ZWLR_LAYER_SURFACE_SET_LAYER: u16 = 8;

fn dispatch_layer_surface(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
    cmd_write: u32,
    _output: &crate::display::OutputInfo,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        ZWLR_LAYER_SURFACE_SET_SIZE => {
            let w = read_u32(&msg.data, 0).unwrap_or(0);
            let h = read_u32(&msg.data, 4).unwrap_or(0);
            with_layer_state_mut(client, obj_id, |s| s.config.size = (w, h));
        }
        ZWLR_LAYER_SURFACE_SET_ANCHOR => {
            let a = read_u32(&msg.data, 0).unwrap_or(0);
            with_layer_state_mut(client, obj_id, |s| s.config.anchor = a);
        }
        ZWLR_LAYER_SURFACE_SET_EXCLUSIVE_ZONE => {
            let z = read_i32(&msg.data, 0).unwrap_or(0);
            with_layer_state_mut(client, obj_id, |s| s.config.exclusive_zone = z);
        }
        ZWLR_LAYER_SURFACE_SET_MARGIN => {
            let t = read_i32(&msg.data, 0).unwrap_or(0);
            let r = read_i32(&msg.data, 4).unwrap_or(0);
            let b = read_i32(&msg.data, 8).unwrap_or(0);
            let l = read_i32(&msg.data, 12).unwrap_or(0);
            with_layer_state_mut(client, obj_id, |s| {
                s.config.margin_top = t;
                s.config.margin_right = r;
                s.config.margin_bottom = b;
                s.config.margin_left = l;
            });
        }
        ZWLR_LAYER_SURFACE_SET_KEYBOARD_INTERACTIVITY => {
            // Accepted as a no-op until input routing supports per-surface
            // keyboard focus rules (Bloom currently uses scene-wide focus).
        }
        ZWLR_LAYER_SURFACE_GET_POPUP => {
            // Accepted as no-op (see comment on ZWLR_LAYER_SHELL_GET_POPUP).
        }
        ZWLR_LAYER_SURFACE_SET_LAYER => {
            let l = read_u32(&msg.data, 0).unwrap_or(u32::MAX);
            match blossom::LayerShellLayer::from_wire(l) {
                Some(layer) => {
                    with_layer_state_mut(client, obj_id, |s| s.config.layer = layer);
                }
                None => {
                    client.send_protocol_error(
                        obj_id,
                        ZWLR_LAYER_SHELL_ERROR_INVALID_LAYER,
                        "invalid layer",
                    );
                }
            }
        }
        ZWLR_LAYER_SURFACE_ACK_CONFIGURE => {
            let serial = read_u32(&msg.data, 0).unwrap_or(0);
            let recognised = with_layer_state_mut(client, obj_id, |s| {
                if let Some(pos) = s.pending_configures.iter().position(|&p| p == serial) {
                    for _ in 0..=pos {
                        s.pending_configures.pop_front();
                    }
                    s.configured = true;
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false);
            if !recognised {
                blossom_warn!(
                    "wayland-server: zwlr_layer_surface.ack_configure unknown serial={}",
                    serial
                );
            }
        }
        ZWLR_LAYER_SURFACE_DESTROY => {
            // Unmap the surface and detach the role from its wl_surface so
            // the same wl_surface could later be re-used with a different
            // role (per spec).  Destroy IPC for the underlying scene surface
            // is not emitted here — the wl_surface itself remains live until
            // it is destroyed by the client.
            let (wl_surface_obj, bloom_surface_id) = match client.objects.get(&obj_id) {
                Some(ObjectEntry::LayerSurface { wl_surface_obj, bloom_surface_id, .. }) => {
                    (*wl_surface_obj, *bloom_surface_id)
                }
                _ => (0, 0),
            };
            if wl_surface_obj != 0 {
                if let Some(ObjectEntry::Surface { layer_surface_obj, .. }) =
                    client.objects.get_mut(&wl_surface_obj)
                {
                    if *layer_surface_obj == Some(obj_id) {
                        *layer_surface_obj = None;
                    }
                }
            }
            // Tell the main thread to stop treating this surface as a layer
            // surface (clear its z_order and dest_rect contribution by
            // sending a zero-everything update).  This keeps stacking sane
            // if the underlying wl_surface is later reused.
            if bloom_surface_id != 0 {
                let cmd = ipc::encode_set_layer_surface(
                    bloom_surface_id, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                );
                let _ = stem::syscall::port_send_all(cmd_write, &cmd);
            }
            client.destroy(obj_id);
            blossom_debug!(
                "wayland-server: zwlr_layer_surface obj={} destroyed (surface={})",
                obj_id,
                bloom_surface_id
            );
        }
        _ => {}
    }
    vec![]
}

/// Apply a mutation to a layer-surface's accumulated state, returning the
/// closure's value if the object exists.
fn with_layer_state_mut<R>(
    client: &mut WaylandClient,
    obj_id: u32,
    f: impl FnOnce(&mut blossom::LayerSurfaceState) -> R,
) -> Option<R> {
    match client.objects.get_mut(&obj_id) {
        Some(ObjectEntry::LayerSurface { state, .. }) => Some(f(state)),
        _ => None,
    }
}

/// Drive the layer-surface configure/ack handshake from `wl_surface.commit`.
///
/// Returns `true` when the commit may continue down the regular import /
/// damage / commit path; `false` if a protocol error has been sent and the
/// caller should bail out.
///
/// On the very first commit (the spec-mandated empty commit that triggers
/// the initial configure) we send a `configure(serial, w, h)` event with the
/// computed size and queue the serial for the client to ack before any
/// buffer may be attached.  On subsequent commits we push a fresh
/// `WCMD_SET_LAYER_SURFACE` so the main thread can recompute placement
/// (size/anchor/margins may have changed since the last commit).
fn on_layer_surface_commit(
    layer_obj_id: u32,
    client: &mut WaylandClient,
    has_buffer: bool,
    out: &mut Vec<Vec<u8>>,
) -> bool {
    // Snapshot what we need without holding a borrow.
    let (config, initial_done, configured, bloom_surface_id) =
        match client.objects.get(&layer_obj_id) {
            Some(ObjectEntry::LayerSurface { state, bloom_surface_id, .. }) => (
                state.config,
                state.initial_commit_done,
                state.configured,
                *bloom_surface_id,
            ),
            _ => return true,
        };

    if has_buffer && !configured {
        // Spec: the client must complete the initial configure handshake
        // before attaching a buffer.
        blossom_warn!(
            "wayland-server: zwlr_layer_surface obj={} buffer committed before ack_configure",
            layer_obj_id
        );
        client.send_protocol_error(
            layer_obj_id,
            blossom::layer_surface_error::INVALID_SURFACE_STATE,
            "buffer committed before ack_configure",
        );
        return false;
    }

    // Always push the latest layer-surface state to the main thread so the
    // scene picks up size/anchor/layer changes on this commit.
    out.push(
        ipc::encode_set_layer_surface(
            bloom_surface_id,
            config.layer as u32,
            config.anchor,
            config.exclusive_zone,
            config.margin_top,
            config.margin_right,
            config.margin_bottom,
            config.margin_left,
            config.size.0,
            config.size.1,
            1, // active
        )
        .to_vec(),
    );

    if !initial_done {
        // First commit: trigger the initial configure event.  The configure
        // size is computed against the primary output by the main thread,
        // but we need to send it now from the wayland thread; fall back to
        // (0, 0) which the client interprets as "compositor has no
        // preference, please choose your own size".  Real placement comes
        // from the WCMD_SET_LAYER_SURFACE above; the configure here only
        // exists to satisfy the lifecycle.
        let serial = next_layer_serial(client);
        let cw = config.size.0;
        let ch = config.size.1;
        send_layer_surface_configure(client, layer_obj_id, serial, cw, ch);
        if let Some(ObjectEntry::LayerSurface { state, .. }) = client.objects.get_mut(&layer_obj_id)
        {
            state.initial_commit_done = true;
            state.pending_configures.push_back(serial);
        }
    }

    true
}

/// Allocate a fresh serial for layer-surface configures.  Re-uses the
/// per-client buffer-key counter as a monotonic source — the values do not
/// need to share a namespace with xdg-shell serials.
fn next_layer_serial(client: &mut WaylandClient) -> u32 {
    client.alloc_buf_key()
}

/// Send `zwlr_layer_surface_v1.configure(serial, width, height)` (opcode 0).
fn send_layer_surface_configure(
    client: &WaylandClient,
    layer_obj_id: u32,
    serial: u32,
    width: u32,
    height: u32,
) {
    let mut payload = alloc::vec::Vec::with_capacity(12);
    payload.extend_from_slice(&serial.to_ne_bytes());
    payload.extend_from_slice(&width.to_ne_bytes());
    payload.extend_from_slice(&height.to_ne_bytes());
    client.send(layer_obj_id, 0, &payload);
    blossom_debug!(
        "wayland-server: zwlr_layer_surface.configure obj={} serial={} {}x{}",
        layer_obj_id,
        serial,
        width,
        height
    );
}

// ── wl_data_device_manager ────────────────────────────────────────────────────

/// wl_data_device_manager request opcodes.
const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE: u16 = 0;
const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE: u16 = 1;

fn dispatch_data_device_manager(msg: &WireMsg, client: &mut WaylandClient) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE => {
            if let Some(new_id) = read_u32(&msg.data, 0) {
                client
                    .insert(new_id, ObjectEntry::DataSource { mime_types: alloc::vec::Vec::new() });
                blossom_debug!("wayland-server: data_source obj={} created", new_id);
            }
        }
        WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE => {
            // get_data_device(new_id: new_id<wl_data_device>, seat: object<wl_seat>)
            let new_id = match read_u32(&msg.data, 0) {
                Some(id) => id,
                None => return vec![],
            };
            let seat_obj = read_u32(&msg.data, 4).unwrap_or(0);
            client.insert(new_id, ObjectEntry::DataDevice { seat_obj });
            client.data_device_obj = Some(new_id);
            blossom_debug!(
                "wayland-server: data_device obj={} created for seat={}",
                new_id,
                seat_obj
            );
        }
        _ => {}
    }
    vec![]
}

// ── wl_data_source ────────────────────────────────────────────────────────────

/// wl_data_source request opcodes.
const WL_DATA_SOURCE_OFFER: u16 = 0;
const WL_DATA_SOURCE_DESTROY: u16 = 1;
const WL_DATA_SOURCE_SET_ACTIONS: u16 = 2;

fn dispatch_data_source(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_SOURCE_OFFER => {
            // offer(mime_type: string)
            if let Some((mime_bytes, _)) = read_string(&msg.data, 0) {
                let mime = String::from_utf8_lossy(mime_bytes).into_owned();
                if let Some(ObjectEntry::DataSource { mime_types }) =
                    client.objects.get_mut(&obj_id)
                {
                    mime_types.push(mime);
                }
            }
        }
        WL_DATA_SOURCE_DESTROY => {
            blossom_debug!("wayland-server: data_source obj={} destroyed", obj_id);
            client.destroy(obj_id);
        }
        WL_DATA_SOURCE_SET_ACTIONS => {
            // set_actions(dnd_actions: uint) — record for DnD negotiation.
            blossom_debug!("wayland-server: data_source.set_actions obj={}", obj_id);
        }
        _ => {}
    }
    vec![]
}

// ── wl_data_device ────────────────────────────────────────────────────────────

/// wl_data_device request opcodes.
const WL_DATA_DEVICE_START_DRAG: u16 = 0;
const WL_DATA_DEVICE_SET_SELECTION: u16 = 1;
const WL_DATA_DEVICE_RELEASE: u16 = 2;

fn dispatch_data_device(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_DEVICE_SET_SELECTION => {
            // set_selection(source: object<wl_data_source>|null, serial: uint)
            let source_obj = read_u32(&msg.data, 0).unwrap_or(0);
            let mime_types = if source_obj != 0 {
                match client.objects.get(&source_obj) {
                    Some(ObjectEntry::DataSource { mime_types }) => mime_types.clone(),
                    _ => alloc::vec::Vec::new(),
                }
            } else {
                alloc::vec::Vec::new()
            };
            blossom_debug!(
                "wayland-server: data_device.set_selection source={} mimes={}",
                source_obj,
                mime_types.len()
            );
            // Signal to the server to broadcast the new selection.
            client.pending_clipboard_set = Some((source_obj, mime_types));
        }
        WL_DATA_DEVICE_RELEASE => {
            blossom_debug!("wayland-server: data_device obj={} released", obj_id);
            client.destroy(obj_id);
            client.data_device_obj = None;
        }
        WL_DATA_DEVICE_START_DRAG => {
            // start_drag(source: object<wl_data_source>|null,
            //            origin: object<wl_surface>,
            //            icon:   object<wl_surface>|null,
            //            serial: uint)
            let source_obj = read_u32(&msg.data, 0).unwrap_or(0);
            let origin_surface = read_u32(&msg.data, 4).unwrap_or(0);
            let icon_surface = read_u32(&msg.data, 8).unwrap_or(0);
            let serial = read_u32(&msg.data, 12).unwrap_or(0);
            let mime_types = if source_obj != 0 {
                match client.objects.get(&source_obj) {
                    Some(ObjectEntry::DataSource { mime_types }) => mime_types.clone(),
                    _ => alloc::vec::Vec::new(),
                }
            } else {
                alloc::vec::Vec::new()
            };
            blossom_debug!(
                "wayland-server: data_device.start_drag source={} origin={} icon={} serial={} mimes={}",
                source_obj,
                origin_surface,
                icon_surface,
                serial,
                mime_types.len()
            );
            client.pending_start_drag = Some((source_obj, origin_surface, icon_surface, serial));
        }
        _ => {}
    }
    vec![]
}

// ── wl_data_offer ─────────────────────────────────────────────────────────────

/// wl_data_offer request opcodes.
const WL_DATA_OFFER_ACCEPT: u16 = 0;
const WL_DATA_OFFER_RECEIVE: u16 = 1;
const WL_DATA_OFFER_DESTROY: u16 = 2;
const WL_DATA_OFFER_FINISH: u16 = 3;
const WL_DATA_OFFER_SET_ACTIONS: u16 = 4;

fn dispatch_data_offer(msg: &WireMsg, client: &mut WaylandClient, obj_id: u32) -> Vec<Vec<u8>> {
    match msg.opcode {
        WL_DATA_OFFER_RECEIVE => {
            // receive(mime_type: string, fd: fd)
            // The write end of a pipe is sent as ancillary data by the client.
            let mime = match read_string(&msg.data, 0) {
                Some((bytes, _)) => String::from_utf8_lossy(bytes).into_owned(),
                None => return vec![],
            };
            let write_fd = match client.pending_fds.pop_front() {
                Some(fd) => fd,
                None => {
                    blossom_warn!("wayland-server: data_offer.receive: no fd received");
                    return vec![];
                }
            };
            blossom_debug!(
                "wayland-server: data_offer.receive obj={} mime=\"{}\" fd={}",
                obj_id,
                mime,
                write_fd
            );
            // Signal to the server to forward the fd to the clipboard or DnD source.
            // Include offer_obj so the server can decide which source to route to.
            client.pending_offer_receive = Some((obj_id, mime, write_fd));
        }
        WL_DATA_OFFER_ACCEPT => {
            // accept(serial: uint, mime_type: string|null)
            // serial is first 4 bytes; mime_type follows.
            let mime = if msg.data.len() > 4 {
                match read_string(&msg.data, 4) {
                    Some((bytes, _)) => String::from_utf8_lossy(bytes).into_owned(),
                    None => String::new(),
                }
            } else {
                String::new()
            };
            blossom_debug!(
                "wayland-server: data_offer.accept obj={} mime=\"{}\"",
                obj_id,
                mime
            );
            client.pending_dnd_accept = Some(mime);
        }
        WL_DATA_OFFER_FINISH => {
            blossom_debug!("wayland-server: data_offer.finish obj={}", obj_id);
            client.pending_dnd_finish = true;
        }
        WL_DATA_OFFER_SET_ACTIONS => {
            // set_actions(dnd_actions: uint, preferred_action: uint)
            // Accepted; server uses preferred_action when sending action event back.
            blossom_debug!("wayland-server: data_offer.set_actions obj={}", obj_id);
        }
        WL_DATA_OFFER_DESTROY => {
            blossom_debug!("wayland-server: data_offer obj={} destroyed", obj_id);
            client.destroy(obj_id);
        }
        _ => {}
    }
    vec![]
}

// ── wp_presentation ─────────────────────────────────────────────────────────
//
// Implements the `wp_presentation` global from the Wayland Presentation Time
// protocol (https://wayland.app/protocols/presentation-time).  Bloom uses
// best-effort software timing here: timestamps come from the same monotonic
// clock as `wl_callback.done`, and no `presented` flag bits are claimed (no
// VSYNC/HW_CLOCK/HW_COMPLETION/ZERO_COPY) because true vblank reporting is
// not yet wired through the display pipeline.  When real vsync timing lands
// this implementation should switch to that source and start advertising the
// appropriate flags; see the `wp_presentation` audit issue.

/// wp_presentation request opcodes.
const WP_PRESENTATION_DESTROY: u16 = 0;
const WP_PRESENTATION_FEEDBACK: u16 = 1;

/// wp_presentation_feedback request opcodes — none defined; the object has
/// only events (`sync_output`, `presented`, `discarded`).  All requests are
/// silently ignored.

fn dispatch_presentation(
    msg: &WireMsg,
    client: &mut WaylandClient,
    obj_id: u32,
) -> Vec<Vec<u8>> {
    match msg.opcode {
        WP_PRESENTATION_DESTROY => {
            client.destroy(obj_id);
        }
        WP_PRESENTATION_FEEDBACK => {
            // feedback(surface: object<wl_surface>, callback: new_id<wp_presentation_feedback>)
            let surface_obj = read_u32(&msg.data, 0).unwrap_or(0);
            let fb_id = read_u32(&msg.data, 4).unwrap_or(0);
            if fb_id == 0 {
                return vec![];
            }
            // Always create the feedback object so the client's wire-side ID
            // table stays consistent — even if the surface lookup fails we
            // settle the feedback immediately with `discarded`.
            client.insert(fb_id, ObjectEntry::PresentationFeedback);
            let surface_ok = matches!(
                client.objects.get(&surface_obj),
                Some(ObjectEntry::Surface { .. })
            );
            if surface_ok {
                client.add_pending_presentation_feedback(surface_obj, fb_id);
                blossom_debug!(
                    "wayland-server: wp_presentation.feedback surface_obj={} fb={}",
                    surface_obj,
                    fb_id
                );
            } else {
                // No valid surface — the content this feedback would describe
                // can never be presented, so discard it immediately.
                client.send(fb_id, 2, &[]);
                client.destroy(fb_id);
                blossom_warn!(
                    "wayland-server: wp_presentation.feedback for unknown surface_obj={} → discarded",
                    surface_obj
                );
            }
        }
        _ => {}
    }
    vec![]
}

fn dispatch_presentation_feedback(
    _msg: &WireMsg,
    _client: &mut WaylandClient,
    _obj_id: u32,
) -> Vec<Vec<u8>> {
    // wp_presentation_feedback has no requests in v1; the server initiates
    // both `presented` and `discarded` (each of which is a destructor event).
    vec![]
}

#[cfg(test)]
mod tests {
    use super::region_bounding_rect;

    #[test]
    fn empty_region_returns_none() {
        assert_eq!(region_bounding_rect(&[]), None);
    }

    #[test]
    fn subtract_only_returns_none() {
        // Only subtract ops — no add rects, so no bounding box.
        let rects = [(0, 0, 100, 100, false)];
        assert_eq!(region_bounding_rect(&rects), None);
    }

    #[test]
    fn single_add_rect() {
        let rects = [(0, 0, 480, 320, true)];
        assert_eq!(region_bounding_rect(&rects), Some((0, 0, 480, 320)));
    }

    #[test]
    fn non_zero_origin_add_rect() {
        let rects = [(10, 20, 100, 200, true)];
        assert_eq!(region_bounding_rect(&rects), Some((10, 20, 100, 200)));
    }

    #[test]
    fn two_add_rects_union() {
        // Two non-overlapping rects: bounding box covers both.
        let rects = [(0, 0, 50, 50, true), (100, 100, 50, 50, true)];
        assert_eq!(region_bounding_rect(&rects), Some((0, 0, 150, 150)));
    }

    #[test]
    fn subtract_ignored_in_bounding_rect() {
        // Add covers full surface; subtract reduces visible area but we report
        // the bounding box of add ops only (conservative V1 approximation).
        let rects = [(0, 0, 200, 200, true), (50, 50, 100, 100, false)];
        assert_eq!(region_bounding_rect(&rects), Some((0, 0, 200, 200)));
    }

    #[test]
    fn negative_origin_clamped_to_zero() {
        // Wayland allows negative coords (off-screen); clamp to 0 for scene.
        let rects = [(-10, -20, 100, 100, true)];
        assert_eq!(region_bounding_rect(&rects), Some((0, 0, 90, 80)));
    }

    #[test]
    fn degenerate_zero_size_add_ignored() {
        let rects = [(0, 0, 0, 0, true), (10, 10, 50, 50, true)];
        assert_eq!(region_bounding_rect(&rects), Some((10, 10, 50, 50)));
    }
}
