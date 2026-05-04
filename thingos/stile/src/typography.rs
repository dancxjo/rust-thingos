/// Typography scale for consistent hierarchy across UI surfaces.
///
/// Sizes are in logical pixels. The scale provides seven levels from large
/// display numerals down to fine-print captions.
///
/// ```text
///  SCALE_DISPLAY  48 px  Primary numeric readout (calculator result)
///  SCALE_H1       32 px  Hero heading (clock time)
///  SCALE_H2       20 px  Section heading (launcher panel title)
///  SCALE_H3       15 px  Component heading (tile label, panel header)
///  SCALE_BODY     13 px  Body text (window title, calc expression)
///  SCALE_LABEL    12 px  Secondary annotation (AM/PM, status bar, date)
///  SCALE_CAPTION  10 px  Tertiary detail (file paths, tiny metadata)
/// ```

/// Primary numeric readout — calculator result line or other large display numbers.
pub const SCALE_DISPLAY: f32 = 48.0;

/// Hero heading — for the most prominent heading in a view (e.g. clock time).
pub const SCALE_H1: f32 = 32.0;

/// Section heading — panel or launcher section title.
pub const SCALE_H2: f32 = 20.0;

/// Component heading — tile labels, panel headers, mode text.
pub const SCALE_H3: f32 = 15.0;

/// Body text — window titles, form labels, calc expression line.
pub const SCALE_BODY: f32 = 13.0;

/// Secondary annotation — AM/PM indicator, status bar, date line.
pub const SCALE_LABEL: f32 = 12.0;

/// Tertiary detail — file paths, icon captions, fine-print metadata.
pub const SCALE_CAPTION: f32 = 10.0;

/// Spacing rhythm aligned to a 4 px grid.
///
/// ```text
///  SPACE_XS   4 px  — between tightly related elements (icon + label)
///  SPACE_SM   8 px  — default intra-component gap / small padding
///  SPACE_MD  12 px  — standard component padding
///  SPACE_LG  16 px  — section padding or larger gap
///  SPACE_XL  24 px  — between major layout sections
/// ```

/// Tightest spacing — between tightly coupled elements such as an icon and its label.
pub const SPACE_XS: f32 = 4.0;

/// Small spacing — default intra-component gap or small button padding.
pub const SPACE_SM: f32 = 8.0;

/// Medium spacing — standard component or panel padding.
pub const SPACE_MD: f32 = 12.0;

/// Large spacing — section padding or emphasized separation.
pub const SPACE_LG: f32 = 16.0;

/// Extra-large spacing — separation between major layout regions.
pub const SPACE_XL: f32 = 24.0;
