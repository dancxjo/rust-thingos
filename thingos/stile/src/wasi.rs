//! Versioned ABI adapter for programmable WASI themes.
//!
//! This module deliberately models only the deterministic host handshake. It
//! does not embed a WASI runtime; platform code supplies `WasmThemeHost`.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::paint::{
    CompiledTheme, PaintCommand, PaintList, ThemeControl, ThemeControlRect, ThemeIcon, ThemeRect,
    ThemeRuntimeError, WasiThemeRuntime, WindowChromeRequest,
};

pub const STILE_WASI_VERSION: u32 = 1;
pub const STILE_WASI_RENDER_CHROME: &str = "render_chrome";
pub const STILE_WASI_MAX_INPUT_BYTES: usize = 16 * 1024;
pub const STILE_WASI_MAX_OUTPUT_BYTES: usize = 64 * 1024;

pub trait WasmThemeHost {
    fn alloc(&mut self, len: u32) -> Result<u32, ThemeRuntimeError>;
    fn write_memory(&mut self, ptr: u32, bytes: &[u8]) -> Result<(), ThemeRuntimeError>;
    fn call_render_chrome(
        &mut self,
        input_ptr: u32,
        input_len: u32,
    ) -> Result<u32, ThemeRuntimeError>;
    fn read_memory(&self, ptr: u32, len: u32, out: &mut Vec<u8>) -> Result<(), ThemeRuntimeError>;
}

pub struct WasmTheme<H> {
    host: H,
}

impl<H> WasmTheme<H> {
    pub const fn new(host: H) -> Self {
        Self { host }
    }

    pub fn into_host(self) -> H {
        self.host
    }
}

impl<H: WasmThemeHost> WasiThemeRuntime for WasmTheme<H> {
    fn render_window_chrome<'a>(
        &mut self,
        module: CompiledTheme<'_>,
        request: WindowChromeRequest<'a>,
        out: &mut PaintList<'a>,
    ) -> Result<(), ThemeRuntimeError> {
        if module.entrypoint != STILE_WASI_RENDER_CHROME {
            return Err(ThemeRuntimeError::InvalidModule);
        }

        let request = JsonWindowChromeRequest::from_request(request);
        let input =
            serde_json::to_vec(&request).map_err(|_| ThemeRuntimeError::SerializationFailed)?;
        if input.len() > STILE_WASI_MAX_INPUT_BYTES {
            return Err(ThemeRuntimeError::MemoryLimitExceeded);
        }

        let input_ptr = self.host.alloc(input.len() as u32)?;
        self.host.write_memory(input_ptr, &input)?;
        let output_ptr = self.host.call_render_chrome(input_ptr, input.len() as u32)?;

        let mut len_bytes = Vec::new();
        self.host.read_memory(output_ptr, 4, &mut len_bytes)?;
        if len_bytes.len() != 4 {
            return Err(ThemeRuntimeError::InvalidResponse);
        }
        let output_len =
            u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]) as usize;
        if output_len > STILE_WASI_MAX_OUTPUT_BYTES {
            return Err(ThemeRuntimeError::MemoryLimitExceeded);
        }

        let mut output = Vec::new();
        self.host.read_memory(output_ptr + 4, output_len as u32, &mut output)?;
        let paint = serde_json::from_slice::<JsonPaintList>(&output)
            .map_err(|_| ThemeRuntimeError::InvalidResponse)?;
        paint.append_to(out)?;
        Ok(())
    }
}

#[derive(Serialize)]
struct JsonWindowChromeRequest<'a> {
    version: u32,
    width: i32,
    height: i32,
    active: bool,
    title: Option<&'a str>,
    frame: i32,
    titlebar_height: i32,
    visual_rect: JsonRect,
    content_rect: JsonRect,
    hovered: bool,
    primary_button_down: bool,
    pointer_x: i32,
    pointer_y: i32,
    shaded: bool,
    fullscreen: bool,
    controls: Vec<JsonControl>,
}

impl<'a> JsonWindowChromeRequest<'a> {
    fn from_request(request: WindowChromeRequest<'a>) -> Self {
        Self {
            version: STILE_WASI_VERSION,
            width: request.visual_rect.w,
            height: request.visual_rect.h,
            active: request.state.active,
            title: request.title,
            frame: request.frame,
            titlebar_height: request.titlebar_height,
            visual_rect: JsonRect::from_rect(request.visual_rect),
            content_rect: JsonRect::from_rect(request.content_rect),
            hovered: request.state.hovered,
            primary_button_down: request.state.primary_button_down,
            pointer_x: request.state.pointer_x,
            pointer_y: request.state.pointer_y,
            shaded: request.state.shaded,
            fullscreen: request.state.fullscreen,
            controls: request
                .controls
                .iter()
                .flatten()
                .copied()
                .map(JsonControl::from_control)
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
struct JsonRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl JsonRect {
    fn from_rect(rect: ThemeRect) -> Self {
        Self { x: rect.x, y: rect.y, w: rect.w, h: rect.h }
    }
}

#[derive(Serialize)]
struct JsonControl {
    kind: &'static str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl JsonControl {
    fn from_control(control: ThemeControlRect) -> Self {
        let kind = match control.control {
            ThemeControl::Shade => "minimize",
            ThemeControl::Fullscreen => "maximize",
            ThemeControl::Close => "close",
        };
        Self { kind, x: control.rect.x, y: control.rect.y, w: control.rect.w, h: control.rect.h }
    }
}

#[derive(Deserialize)]
struct JsonPaintList {
    commands: Vec<JsonPaintCommand>,
}

impl JsonPaintList {
    fn append_to<'a>(self, out: &mut PaintList<'a>) -> Result<(), ThemeRuntimeError> {
        for command in self.commands {
            out.commands.push(command.into_paint()?);
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum JsonPaintCommand {
    #[serde(rename = "rect")]
    Rect {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: String,
    },
    VerticalGradient {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        top: String,
        bottom: String,
    },
    HorizontalGradient {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        left: String,
        center: String,
        right: String,
    },
    Stroke {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        thickness: i32,
        color: String,
    },
    Shadow {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        offset_x: i32,
        offset_y: i32,
        blur_radius: i32,
        color: String,
    },
    PushClip {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    },
    PopClip,
    Text {
        x: i32,
        y: i32,
        px: f32,
        text: String,
        color: String,
    },
    Icon {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        icon: String,
        color: String,
    },
}

impl JsonPaintCommand {
    fn into_paint<'a>(self) -> Result<PaintCommand<'a>, ThemeRuntimeError> {
        match self {
            Self::Rect { x, y, w, h, color } => Ok(PaintCommand::FillRect {
                rect: ThemeRect::new(x, y, w, h),
                color: parse_color(&color)?,
            }),
            Self::VerticalGradient { x, y, w, h, top, bottom } => {
                Ok(PaintCommand::VerticalGradient {
                    rect: ThemeRect::new(x, y, w, h),
                    top: parse_color(&top)?,
                    bottom: parse_color(&bottom)?,
                })
            }
            Self::HorizontalGradient { x, y, w, h, left, center, right } => {
                Ok(PaintCommand::HorizontalGradient {
                    rect: ThemeRect::new(x, y, w, h),
                    left: parse_color(&left)?,
                    center: parse_color(&center)?,
                    right: parse_color(&right)?,
                })
            }
            Self::Stroke { x, y, w, h, thickness, color } => Ok(PaintCommand::StrokeRect {
                rect: ThemeRect::new(x, y, w, h),
                thickness,
                color: parse_color(&color)?,
            }),
            Self::Shadow { x, y, w, h, offset_x, offset_y, blur_radius, color } => {
                Ok(PaintCommand::Shadow {
                    rect: ThemeRect::new(x, y, w, h),
                    offset_x,
                    offset_y,
                    blur_radius,
                    color: parse_color(&color)?,
                })
            }
            Self::PushClip { x, y, w, h } => {
                Ok(PaintCommand::PushClip { rect: ThemeRect::new(x, y, w, h) })
            }
            Self::PopClip => Ok(PaintCommand::PopClip),
            Self::Text { x, y, px, text, color } => Ok(PaintCommand::Text {
                x,
                y,
                px_size_bits: px.to_bits(),
                text: Cow::Owned(text),
                color: parse_color(&color)?,
            }),
            Self::Icon { x, y, w, h, icon, color } => Ok(PaintCommand::Icon {
                rect: ThemeRect::new(x, y, w, h),
                icon: parse_icon(&icon)?,
                color: parse_color(&color)?,
            }),
        }
    }
}

fn parse_icon(icon: &str) -> Result<ThemeIcon, ThemeRuntimeError> {
    if icon.eq_ignore_ascii_case("shade") || icon.eq_ignore_ascii_case("minimize") {
        Ok(ThemeIcon::Shade)
    } else if icon.eq_ignore_ascii_case("unshade") {
        Ok(ThemeIcon::Unshade)
    } else if icon.eq_ignore_ascii_case("fullscreen") || icon.eq_ignore_ascii_case("maximize") {
        Ok(ThemeIcon::Fullscreen)
    } else if icon.eq_ignore_ascii_case("restore") {
        Ok(ThemeIcon::Restore)
    } else if icon.eq_ignore_ascii_case("close") {
        Ok(ThemeIcon::Close)
    } else {
        Err(ThemeRuntimeError::InvalidResponse)
    }
}

fn parse_color(color: &str) -> Result<u32, ThemeRuntimeError> {
    let hex = color.strip_prefix('#').ok_or(ThemeRuntimeError::InvalidResponse)?;
    let value = u32::from_str_radix(hex, 16).map_err(|_| ThemeRuntimeError::InvalidResponse)?;
    match hex.len() {
        6 => Ok(0xFF000000 | value),
        8 => Ok(value),
        _ => Err(ThemeRuntimeError::InvalidResponse),
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;
    use crate::paint::{ThemeState, WindowChromeRequest};

    struct MockHost {
        memory: Vec<u8>,
        next: u32,
        response: &'static [u8],
    }

    impl MockHost {
        fn new(response: &'static [u8]) -> Self {
            Self { memory: vec![0; 4096], next: 64, response }
        }
    }

    impl WasmThemeHost for MockHost {
        fn alloc(&mut self, len: u32) -> Result<u32, ThemeRuntimeError> {
            let ptr = self.next;
            self.next = self.next.saturating_add(len).saturating_add(8);
            if self.memory.len() < self.next as usize {
                self.memory.resize(self.next as usize, 0);
            }
            Ok(ptr)
        }

        fn write_memory(&mut self, ptr: u32, bytes: &[u8]) -> Result<(), ThemeRuntimeError> {
            let start = ptr as usize;
            let end = start + bytes.len();
            self.memory[start..end].copy_from_slice(bytes);
            Ok(())
        }

        fn call_render_chrome(
            &mut self,
            input_ptr: u32,
            input_len: u32,
        ) -> Result<u32, ThemeRuntimeError> {
            let input = &self.memory[input_ptr as usize..(input_ptr + input_len) as usize];
            assert!(core::str::from_utf8(input).unwrap().contains("Calculator"));
            let ptr = self.alloc(self.response.len() as u32 + 4)?;
            let len = (self.response.len() as u32).to_le_bytes();
            self.write_memory(ptr, &len)?;
            self.write_memory(ptr + 4, self.response)?;
            Ok(ptr)
        }

        fn read_memory(
            &self,
            ptr: u32,
            len: u32,
            out: &mut Vec<u8>,
        ) -> Result<(), ThemeRuntimeError> {
            let start = ptr as usize;
            let end = start + len as usize;
            if end > self.memory.len() {
                return Err(ThemeRuntimeError::InvalidResponse);
            }
            out.extend_from_slice(&self.memory[start..end]);
            Ok(())
        }
    }

    fn request<'a>() -> WindowChromeRequest<'a> {
        WindowChromeRequest {
            visual_rect: ThemeRect::new(0, 0, 800, 600),
            content_rect: ThemeRect::new(4, 32, 792, 564),
            frame: 4,
            titlebar_height: 32,
            controls: [None, None, None],
            state: ThemeState {
                active: true,
                hovered: false,
                primary_button_down: false,
                pointer_x: -1,
                pointer_y: -1,
                shaded: false,
                fullscreen: false,
            },
            title: Some("Calculator"),
        }
    }

    #[test]
    fn wasm_theme_adapter_reads_buffered_json_response() {
        let response = br##"{"commands":[{"type":"rect","x":0,"y":0,"w":800,"h":32,"color":"#2A241F"},{"type":"text","x":12,"y":22,"px":15.0,"text":"Calculator","color":"#F5E6D3"}]}"##;
        let mut runtime = WasmTheme::new(MockHost::new(response));
        let mut out = PaintList::default();
        runtime
            .render_window_chrome(
                CompiledTheme { module: &[], entrypoint: STILE_WASI_RENDER_CHROME },
                request(),
                &mut out,
            )
            .unwrap();
        assert_eq!(out.commands.len(), 2);
        assert!(matches!(out.commands[0], PaintCommand::FillRect { .. }));
        assert!(matches!(out.commands[1], PaintCommand::Text { .. }));
    }

    #[test]
    fn wasm_theme_adapter_rejects_malformed_output() {
        let mut runtime = WasmTheme::new(MockHost::new(b"not-json"));
        let mut out = PaintList::default();
        let err = runtime
            .render_window_chrome(
                CompiledTheme { module: &[], entrypoint: STILE_WASI_RENDER_CHROME },
                request(),
                &mut out,
            )
            .unwrap_err();
        assert_eq!(err, ThemeRuntimeError::InvalidResponse);
        assert!(out.commands.is_empty());
    }
}
