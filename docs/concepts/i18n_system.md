# Internationalization (i18n) System

Thing-OS UI text is localized through lightweight `stem` macros and XLIFF
catalogs. New code should keep English source text in Rust and use the macros
only as tags for extraction and runtime lookup.

## Authoring Text

Use source-string macros for new UI:

```rust
const TITLE: stem::i18n::LocalizedText = stem::t!("Font Explorer");
const CLOSE: stem::i18n::LocalizedText =
    stem::t!("Close", note: "Window close button label");

let label = stem::tr!("Close");
let status = stem::tf!("Copied {} files", count);
```

Legacy key/fallback forms remain supported while older code is migrated:

```rust
const TITLE: stem::i18n::LocalizedText = stem::t!("ui.fonts.title", "Fonts");
let label = stem::tr!("ui.window.close", "Close");
let status = stem::tf!("copy.status", "Copied {} files", count);
```

`LocalizedText::get()`, `tr!`, and `tf!` resolve against the current locale and
fall back through English, then the source/fallback string in code.

## Catalogs

Catalogs live in:

```text
thingos/stem/i18n/catalogs/
```

Each locale is an XLIFF 1.2 file named with its ISO locale, for example:

```text
en.xlf
eo.xlf
la.xlf
syc.xlf
```

Adding a language should be just adding another file such as `fr.xlf` with
`target-language="fr"`. `thingos/stem/build.rs` discovers `*.xlf`, compiles
static no_std lookup tables, and registers the locale automatically.

The build also writes an extracted English template to:

```text
$OUT_DIR/i18n_extracted.en.xlf
```

That generated file is intended for tooling and translators; checked-in
translation catalogs are the source of truth for runtime translations.

## Runtime Switching

Bristle handles the global F1 hotkey. Pressing F1 cycles the session locale and
writes the active ISO code to:

```text
/session/locale
```

`stem::i18n` syncs from that file on Thing-OS when translating text or checking
the i18n generation counter. UI code that caches rendered text should compare
`stem::i18n::generation()` and redraw when it changes.

The boot default is set by `locale=syc` in generated `limine.conf`. Bristle
reads that kernel command-line value during startup and seeds `/session/locale`.

## Notes

- Keep kernel code `no_std`; this i18n path is for `stem` userspace/runtime
  text and no_std-compatible libraries.
- Prefer source-string macros in new UI code.
- Use `note: "..."` when translators need context, especially for short labels
  or words that can be verbs or nouns.
