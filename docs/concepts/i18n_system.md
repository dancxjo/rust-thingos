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

Userland catalogs live in:

```text
thingos/stem/i18n/catalogs/
```

Kernel catalogs live in the same XLIFF 1.2 shape, but are compiled by the
kernel build script so early boot text is available before VFS and userland:

```text
thingos/kernel/i18n/catalogs/
```

Each locale is an XLIFF 1.2 file named with its ISO locale, for example:

```text
en.xlf
eo.xlf
la.xlf
syc.xlf
```

Adding a language should be just adding another file such as `fr.xlf` with
`target-language="fr"`. `thingos/stem/build.rs` and
`thingos/kernel/build.rs` discover `*.xlf`, compile static no_std lookup
tables, and register the locale automatically.

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

The boot default is set by `language=la` in generated `limine.conf`.
The kernel parses that value before first framebuffer paint,
uses it for early boot text, and exposes the selected locale at `/dev/locale`.
Bristle mirrors `/dev/locale` into `/session/locale` during startup so
userland inherits the same kernel-selected locale.

## Notes

- Keep kernel code `no_std`; this i18n path is for `stem` userspace/runtime
  text and no_std-compatible libraries.
- Prefer source-string macros in new UI code.
- Use `note: "..."` when translators need context, especially for short labels
  or words that can be verbs or nouns.
