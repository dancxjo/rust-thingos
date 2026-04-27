//! Internationalization (i18n) support for Thing-OS.
//!
//! This module provides a first-class i18n system where every UI-visible string
//! is represented as a (key, fallback) pair, enabling runtime locale switching.
//!
//! ## Design
//!
//! - `TextKey`: Stable identifier for translatable strings, usually the
//!   English source string for new UI
//! - `LocalizedText`: Combines a key with a fallback string
//! - `LocaleId`: Identifies a locale compiled from `i18n/catalogs/*.xlf`
//! - `Catalog`: Maps source strings and compatibility keys to translations
//! - `Translator`: Provides translation services with fallback chain
//!
//! ## Usage
//!
//! ```ignore
//! use stem::i18n::LocalizedText;
//!
//! // Define text with the English source string in code.
//! const WINDOW_TITLE: LocalizedText = stem::t!("Window title", note: "Main window title");
//!
//! // Get translated string for current locale
//! let title = WINDOW_TITLE.get();
//! ```

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use spin::Mutex;

pub mod generated;

/// A stable identifier for a translatable string.
///
/// Text keys are used to look up translations in locale catalogs.
/// They should be hierarchical and descriptive (e.g., "ui.fonts.title").
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextKey(&'static str);

impl TextKey {
    /// Create a new text key from a static string.
    pub const fn new(key: &'static str) -> Self {
        Self(key)
    }

    /// Get the key string.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

/// A locale identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocaleId(&'static str);

impl LocaleId {
    /// Create a new locale identifier.
    pub const fn new(id: &'static str) -> Self {
        Self(id)
    }

    /// Get the locale string.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }

    /// English locale.
    pub const EN: LocaleId = LocaleId("en");
    /// Compatibility alias for English (US).
    pub const EN_US: LocaleId = LocaleId("en-US");
    /// Esperanto locale.
    pub const EO: LocaleId = LocaleId("eo");
    /// Latin locale.
    pub const LA: LocaleId = LocaleId("la");
    /// Classical Syriac locale.
    pub const SYC: LocaleId = LocaleId("syc");
}

/// A localizable text with a key and fallback string.
///
/// This is the primary type used throughout the UI code.
/// It combines a stable text key with a fallback string that is used
/// when no translation is available. Fallbacks are Esperanto in the current
/// single-locale system.
#[derive(Clone, Copy, Debug)]
pub struct LocalizedText {
    /// The translation key
    pub key: TextKey,
    /// The fallback text (typically English)
    pub fallback: &'static str,
    /// Optional translator note. The runtime does not use this, but the build
    /// extractor emits it into generated XLIFF templates.
    pub note: Option<&'static str>,
}

impl LocalizedText {
    /// Create a new localized text.
    pub const fn new(key: &'static str, fallback: &'static str) -> Self {
        Self { key: TextKey::new(key), fallback, note: None }
    }

    /// Create a new localized text with translator context.
    pub const fn new_with_note(
        key: &'static str,
        fallback: &'static str,
        note: &'static str,
    ) -> Self {
        Self { key: TextKey::new(key), fallback, note: Some(note) }
    }

    /// Get the translated string for the current locale.
    ///
    /// Falls back to the fallback string if no translation is available.
    pub fn get(&self) -> &str {
        TRANSLATOR.translate(self.key).unwrap_or(self.fallback)
    }

    /// Get the translation key.
    pub const fn key(&self) -> TextKey {
        self.key
    }

    /// Get the fallback string.
    pub const fn fallback(&self) -> &'static str {
        self.fallback
    }

    /// Get the optional translator note.
    pub const fn note(&self) -> Option<&'static str> {
        self.note
    }
}

/// Convenient macro for creating LocalizedText instances.
///
/// # Example
/// ```ignore
/// const TITLE: LocalizedText = t!("ui.window.title", "Window Title");
/// ```
#[macro_export]
macro_rules! t {
    ($source:literal) => {
        $crate::i18n::LocalizedText::new($source, $source)
    };
    ($source:literal, note: $note:literal) => {
        $crate::i18n::LocalizedText::new_with_note($source, $source, $note)
    };
    ($key:expr, $fallback:expr) => {
        $crate::i18n::LocalizedText::new($key, $fallback)
    };
    ($key:expr, $fallback:expr, note: $note:literal) => {
        $crate::i18n::LocalizedText::new_with_note($key, $fallback, $note)
    };
}

/// Return translated text.
///
/// New call sites can use `stem::tr!("English text")`; legacy call sites can
/// keep using `stem::tr!("stable.key", "English text")`.
#[macro_export]
macro_rules! tr {
    ($source:literal) => {
        $crate::i18n::translate($crate::i18n::TextKey::new($source)).unwrap_or($source)
    };
    ($source:literal, note: $note:literal) => {
        $crate::i18n::translate($crate::i18n::TextKey::new($source)).unwrap_or($source)
    };
    ($key:expr, $fallback:expr) => {
        $crate::i18n::translate($crate::i18n::TextKey::new($key)).unwrap_or($fallback)
    };
    ($key:expr, $fallback:expr, note: $note:literal) => {
        $crate::i18n::translate($crate::i18n::TextKey::new($key)).unwrap_or($fallback)
    };
}

/// Format translated text.
#[macro_export]
macro_rules! tf {
    ($key:expr, $fallback:literal, note: $note:literal $(, $arg:expr)* $(,)?) => {
        $crate::i18n::format_message(
            $crate::tr!($key, $fallback, note: $note),
            &[$(&$arg as &dyn core::fmt::Display),*],
        )
    };
    ($key:expr, $fallback:literal $(, $arg:expr)* $(,)?) => {
        $crate::i18n::format_message(
            $crate::tr!($key, $fallback),
            &[$(&$arg as &dyn core::fmt::Display),*],
        )
    };
    ($source:literal, note: $note:literal $(, $arg:expr)* $(,)?) => {{
        $crate::i18n::format_message(
            $crate::tr!($source, note: $note),
            &[$(&$arg as &dyn core::fmt::Display),*],
        )
    }};
    ($source:literal $(, $arg:expr)* $(,)?) => {{
        $crate::i18n::format_message(
            $crate::tr!($source),
            &[$(&$arg as &dyn core::fmt::Display),*],
        )
    }};
}

/// A translation catalog for a specific locale.
///
/// Maps text keys to translated strings.
pub struct Catalog {
    locale: LocaleId,
    translations: BTreeMap<&'static str, &'static str>,
}

impl Catalog {
    /// Create a new empty catalog for a locale.
    pub const fn new(locale: LocaleId) -> Self {
        Self { locale, translations: BTreeMap::new() }
    }

    /// Create a catalog from a static translation table.
    pub fn from_table(locale: LocaleId, table: &[(&'static str, &'static str)]) -> Self {
        let mut catalog = Self::new(locale);
        for (key, translation) in table {
            catalog.translations.insert(key, translation);
        }
        catalog
    }

    /// Get a translation for a key.
    pub fn get(&self, key: TextKey) -> Option<&'static str> {
        self.translations.get(key.as_str()).copied()
    }

    /// Get the locale of this catalog.
    pub fn locale(&self) -> LocaleId {
        self.locale
    }
}

/// Global translator that manages locale switching and translation lookups.
pub struct Translator {
    initialized: AtomicBool,
    /// Increments each time locale changes (for UI cache invalidation)
    generation: AtomicU64,
    current_locale: Mutex<LocaleId>,
    catalogs: Mutex<BTreeMap<LocaleId, Catalog>>,
}

impl Translator {
    /// Create a new translator.
    const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            generation: AtomicU64::new(0),
            current_locale: Mutex::new(LocaleId::new(generated::DEFAULT_LOCALE)),
            catalogs: Mutex::new(BTreeMap::new()),
        }
    }

    /// Initialize the translator with default catalogs.
    pub fn init(&self) {
        let mut catalogs = self.catalogs.lock();

        catalogs.clear();
        for &(locale, table) in generated::CATALOGS {
            let locale = LocaleId::new(locale);
            catalogs.insert(locale, Catalog::from_table(locale, table));
        }
        drop(catalogs);

        let mut current = self.current_locale.lock();
        if !generated::LOCALE_IDS.contains(&current.as_str()) {
            *current = LocaleId::new(generated::DEFAULT_LOCALE);
        }
        self.initialized.store(true, Ordering::Release);
    }

    fn ensure_initialized(&self) {
        if !self.initialized.load(Ordering::Acquire) {
            self.init();
        }
    }

    /// Get the current locale.
    pub fn current_locale(&self) -> LocaleId {
        self.ensure_initialized();
        self.sync_from_session();
        self.current_locale_local()
    }

    fn current_locale_local(&self) -> LocaleId {
        *self.current_locale.lock()
    }

    /// Set the current locale.
    pub fn set_locale(&self, locale: LocaleId) {
        self.ensure_initialized();
        if !self.catalogs.lock().contains_key(&locale) {
            return;
        }

        let mut current = self.current_locale.lock();
        if *current != locale {
            *current = locale;
            self.generation.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Cycle to the next locale.
    pub fn cycle_locale(&self) -> LocaleId {
        self.ensure_initialized();
        self.sync_from_session();
        let current = self.current_locale_local();
        let locales = generated::LOCALE_IDS;
        if locales.is_empty() {
            return current;
        }
        let current_idx =
            locales.iter().position(|locale| *locale == current.as_str()).unwrap_or(0);
        let next = locales[(current_idx + 1) % locales.len()];
        self.set_locale(LocaleId::new(next));
        self.current_locale_local()
    }

    /// Get the current generation counter.
    ///
    /// This increments each time the locale changes, allowing UIs to detect
    /// when they need to redraw with new translations.
    pub fn generation(&self) -> u64 {
        self.ensure_initialized();
        self.sync_from_session();
        self.generation.load(Ordering::Relaxed)
    }

    /// Translate a text key to the current locale.
    ///
    /// Returns None if no translation is found (caller should use fallback).
    pub fn translate(&self, key: TextKey) -> Option<&'static str> {
        self.ensure_initialized();
        self.sync_from_session();
        let locale = self.current_locale_local();
        let catalogs = self.catalogs.lock();

        // Try current locale.
        if let Some(catalog) = catalogs.get(&locale) {
            if let Some(translation) = catalog.get(key) {
                return Some(translation);
            }
        }

        let default_locale = LocaleId::new(generated::DEFAULT_LOCALE);
        if locale != default_locale {
            if let Some(catalog) = catalogs.get(&default_locale) {
                if let Some(translation) = catalog.get(key) {
                    return Some(translation);
                }
            }
        }

        None
    }

    #[cfg(target_os = "thingos")]
    fn sync_from_session(&self) {
        use abi::syscall::vfs_flags::O_RDONLY;

        use crate::syscall::vfs::{vfs_close, vfs_open, vfs_read};

        let Ok(fd) = vfs_open("/session/locale", O_RDONLY) else {
            return;
        };
        let mut buf = [0u8; 32];
        let Ok(n) = vfs_read(fd, &mut buf) else {
            let _ = vfs_close(fd);
            return;
        };
        let _ = vfs_close(fd);
        let Ok(text) = core::str::from_utf8(&buf[..n]) else {
            return;
        };
        let locale = text.trim();
        if locale.is_empty() {
            return;
        }
        let locale = LocaleId::new(leak_locale(locale));
        if !self.catalogs.lock().contains_key(&locale) {
            return;
        }
        let mut current = self.current_locale.lock();
        if *current != locale {
            *current = locale;
            self.generation.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[cfg(not(target_os = "thingos"))]
    fn sync_from_session(&self) {}
}

#[cfg(target_os = "thingos")]
fn leak_locale(locale: &str) -> &'static str {
    match locale {
        "en" => "en",
        "en-US" => "en-US",
        "eo" => "eo",
        "la" => "la",
        "syc" => "syc",
        _ => generated::LOCALE_IDS
            .iter()
            .copied()
            .find(|candidate| *candidate == locale)
            .unwrap_or(generated::DEFAULT_LOCALE),
    }
}

/// Format a translated runtime pattern with `{}` placeholders.
///
/// This intentionally supports the common positional placeholder used by the
/// current utility strings. Complex Rust formatting remains a compile-time
/// feature and should keep using `format!` directly.
pub fn format_message(pattern: &str, args: &[&dyn core::fmt::Display]) -> String {
    use core::fmt::Write;

    let mut out = String::new();
    let mut chars = pattern.chars().peekable();
    let mut arg_idx = 0usize;

    while let Some(ch) = chars.next() {
        match ch {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                out.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                out.push('}');
            }
            '{' if chars.peek() == Some(&'}') => {
                chars.next();
                if let Some(arg) = args.get(arg_idx) {
                    let _ = write!(&mut out, "{}", arg);
                    arg_idx += 1;
                } else {
                    out.push_str("{}");
                }
            }
            _ => out.push(ch),
        }
    }

    out
}

/// Global translator instance.
static TRANSLATOR: Translator = Translator::new();

/// Initialize the i18n system.
///
/// This should be called once at application startup.
pub fn init() {
    TRANSLATOR.init();
}

/// Get the current locale.
pub fn current_locale() -> LocaleId {
    TRANSLATOR.current_locale()
}

/// Set the current locale.
pub fn set_locale(locale: LocaleId) {
    TRANSLATOR.set_locale(locale);
}

/// Cycle to the next locale.
pub fn cycle_locale() -> LocaleId {
    TRANSLATOR.cycle_locale()
}

/// Get the current i18n generation counter.
pub fn generation() -> u64 {
    TRANSLATOR.generation()
}

/// Translate a text key.
pub fn translate(key: TextKey) -> Option<&'static str> {
    TRANSLATOR.translate(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_key() {
        let key = TextKey::new("ui.test.key");
        assert_eq!(key.as_str(), "ui.test.key");
    }

    #[test]
    fn test_locale_id() {
        assert_eq!(LocaleId::EN.as_str(), "en");
    }

    #[test]
    fn test_localized_text() {
        let text = LocalizedText::new("test.key", "Fallback Text");
        assert_eq!(text.key().as_str(), "test.key");
        assert_eq!(text.fallback(), "Fallback Text");
        assert_eq!(text.note(), None);

        let noted = LocalizedText::new_with_note("test.key", "Fallback Text", "Translator note");
        assert_eq!(noted.note(), Some("Translator note"));
    }

    #[test]
    fn test_catalog() {
        let table = [("key1", "Translation 1"), ("key2", "Translation 2")];
        let catalog = Catalog::from_table(LocaleId::EO, &table);

        assert_eq!(catalog.get(TextKey::new("key1")), Some("Translation 1"));
        assert_eq!(catalog.get(TextKey::new("key2")), Some("Translation 2"));
        assert_eq!(catalog.get(TextKey::new("key3")), None);
    }

    #[test]
    fn test_translator_locale_switching() {
        let translator = Translator::new();
        translator.init();

        assert_eq!(translator.current_locale(), LocaleId::EN);

        translator.set_locale(LocaleId::LA);
        assert_eq!(translator.current_locale(), LocaleId::LA);

        translator.set_locale(LocaleId::new("missing"));
        assert_eq!(translator.current_locale(), LocaleId::LA);

        translator.set_locale(LocaleId::EO);
        assert_eq!(translator.current_locale(), LocaleId::EO);
    }

    #[test]
    fn test_translator_cycle() {
        let translator = Translator::new();
        translator.init();

        assert_eq!(translator.current_locale(), LocaleId::EN);

        translator.cycle_locale();
        assert_eq!(translator.current_locale(), LocaleId::EO);

        translator.cycle_locale();
        assert_eq!(translator.current_locale(), LocaleId::LA);
    }

    #[test]
    fn test_translator_generation() {
        let translator = Translator::new();
        translator.init();

        let gen1 = translator.generation();
        translator.set_locale(LocaleId::EO);
        let gen2 = translator.generation();
        translator.set_locale(LocaleId::new("missing"));
        let gen3 = translator.generation();

        assert_eq!(gen2, gen1 + 1);
        assert_eq!(gen3, gen2);
    }

    #[test]
    fn test_translation() {
        let translator = Translator::new();
        translator.init();

        assert_eq!(translator.current_locale(), LocaleId::EN);
        assert_eq!(translator.translate(TextKey::new("ui.fonts.title")), Some("Fonts"));
        assert_eq!(translator.translate(TextKey::new("Fonts")), Some("Fonts"));
        translator.set_locale(LocaleId::EO);
        assert_eq!(translator.translate(TextKey::new("ui.fonts.title")), Some("Tiparoj"));
        assert_eq!(translator.translate(TextKey::new("Fonts")), Some("Tiparoj"));
        translator.set_locale(LocaleId::LA);
        assert_eq!(translator.translate(TextKey::new("ui.fonts.title")), Some("Typī"));
        translator.set_locale(LocaleId::SYC);
        assert_eq!(translator.translate(TextKey::new("ui.fonts.title")), Some("ܓܘ̈ܦܐ"));
        let catalogs = translator.catalogs.lock();
        assert_eq!(
            catalogs
                .get(&LocaleId::LA)
                .and_then(|catalog| catalog.get(TextKey::new("ui.fonts.explorer"))),
            Some("Explōrātor typōrum")
        );
    }

    #[test]
    fn test_format_message() {
        assert_eq!(format_message("copy {} to {}", &[&"a", &"b"]), "copy a to b");
        assert_eq!(format_message("{{}} {}", &[&7]), "{} 7");
    }
}
