# stile-wasi ABI

`stile-wasi` is the deterministic boundary for programmable themes. A theme
module may describe how a known part should be painted, but it may not own
focus, input routing, composition, files, devices, time, or randomness.

Version 1 exposes one pure function:

```c
// export name: render_chrome
uint32_t render_chrome(uint32_t input_ptr, uint32_t input_len);
```

The module must export linear memory:

```wat
(memory (export "memory") 1)
```

## Host Protocol

1. The host serializes `WindowChromeRequest` as UTF-8 JSON.
2. The host allocates module memory and writes the JSON input bytes.
3. The host calls `render_chrome(input_ptr, input_len)`.
4. The module returns a pointer to a `WasmBuffer`.
5. The host reads a little-endian `u32` length at `output_ptr`.
6. The host reads `len` bytes at `output_ptr + 4`.
7. The host parses and validates the JSON `PaintList`.

Output buffer layout:

```text
offset +0: u32 len, little endian
offset +4: len bytes of UTF-8 JSON
```

## Constraints

The function is pure: same input must produce the same output.

The module must not perform filesystem, clock, random, network, device, or
environment access. The host is responsible for runtime limits:

- memory limit: 1-4 MiB recommended
- input limit: 16 KiB in the current `stile` adapter
- output limit: 64 KiB in the current `stile` adapter
- execution timeout or fuel budget required by the eventual runtime
- malformed, oversized, or semantically invalid output must be rejected

## Request JSON

Schema, version 1:

```json
{
  "version": 1,
  "width": 800,
  "height": 600,
  "active": true,
  "title": "Calculator",
  "frame": 4,
  "titlebar_height": 32,
  "visual_rect": { "x": 0, "y": 0, "w": 800, "h": 600 },
  "content_rect": { "x": 4, "y": 32, "w": 792, "h": 564 },
  "hovered": false,
  "primary_button_down": false,
  "pointer_x": -1,
  "pointer_y": -1,
  "shaded": false,
  "fullscreen": false,
  "controls": [
    { "kind": "minimize", "x": 704, "y": 4, "w": 24, "h": 24 },
    { "kind": "maximize", "x": 732, "y": 4, "w": 24, "h": 24 },
    { "kind": "close", "x": 760, "y": 4, "w": 24, "h": 24 }
  ]
}
```

`kind` values are `minimize`, `maximize`, and `close`. The host owns the actual
behavior attached to each control.

## Response JSON

Schema, version 1:

```json
{
  "commands": [
    { "type": "rect", "x": 0, "y": 0, "w": 800, "h": 32, "color": "#2A241F" },
    {
      "type": "vertical_gradient",
      "x": 0,
      "y": 0,
      "w": 800,
      "h": 32,
      "top": "#2A241F",
      "bottom": "#1E1A16"
    },
    {
      "type": "text",
      "x": 12,
      "y": 22,
      "px": 15.0,
      "text": "Calculator",
      "color": "#F5E6D3"
    }
  ]
}
```

Supported command types:

- `rect`: `{ x, y, w, h, color }`
- `vertical_gradient`: `{ x, y, w, h, top, bottom }`
- `horizontal_gradient`: `{ x, y, w, h, left, center, right }`
- `stroke`: `{ x, y, w, h, thickness, color }`
- `shadow`: `{ x, y, w, h, offset_x, offset_y, blur_radius, color }`
- `push_clip`: `{ x, y, w, h }`
- `pop_clip`: `{}`
- `text`: `{ x, y, px, text, color }`
- `icon`: `{ x, y, w, h, icon, color }`

Colors are `#RRGGBB` or `#AARRGGBB`. The host converts `#RRGGBB` to opaque ARGB.

The current native adapter is `stile::WasmTheme`. It implements the memory
handshake and validation against a runtime-supplied `WasmThemeHost`, but does
not embed a WASI runtime.
