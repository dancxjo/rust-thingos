use core::sync::atomic::{AtomicUsize, Ordering};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/kernel_i18n_catalogs_gen.rs"));
}

static CURRENT: AtomicUsize = AtomicUsize::new(default_locale_index());

pub fn set_from_cmdline(cmdline: &str) {
    for token in cmdline.split_ascii_whitespace() {
        if let Some(value) = token.strip_prefix("language=") {
            if let Some(locale) = parse_locale(value) {
                CURRENT.store(locale, Ordering::Relaxed);
                return;
            }
        }
    }
}

pub fn set_language(value: &str) {
    if let Some(locale) = parse_locale(value) {
        CURRENT.store(locale, Ordering::Relaxed);
    }
}

pub fn current() -> &'static str {
    generated::LOCALE_IDS
        .get(CURRENT.load(Ordering::Relaxed))
        .copied()
        .unwrap_or(generated::DEFAULT_LOCALE)
}

pub fn translate<'a>(key: &str, fallback: &'a str) -> &'a str {
    let current_idx = CURRENT.load(Ordering::Relaxed);
    if let Some(table) = catalog_at(current_idx) {
        if let Some(value) = lookup(table, key) {
            return value;
        }
    }

    let default_idx = default_locale_index();
    if default_idx != current_idx {
        if let Some(table) = catalog_at(default_idx) {
            if let Some(value) = lookup(table, key) {
                return value;
            }
        }
    }

    fallback
}

fn parse_locale(raw: &str) -> Option<usize> {
    let normalized = normalize_locale(raw.trim_matches('"').trim())?;
    generated::LOCALE_IDS.iter().position(|locale| *locale == normalized)
}

fn normalize_locale(value: &str) -> Option<&str> {
    match value {
        "en_US" | "en-US" => Some("en"),
        "" => None,
        locale => Some(locale),
    }
}

const fn default_locale_index() -> usize {
    let mut idx = 0;
    while idx < generated::LOCALE_IDS.len() {
        if str_eq(generated::LOCALE_IDS[idx], generated::DEFAULT_LOCALE) {
            return idx;
        }
        idx += 1;
    }
    0
}

const fn str_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return false;
    }
    let mut idx = 0;
    while idx < left.len() {
        if left[idx] != right[idx] {
            return false;
        }
        idx += 1;
    }
    true
}

fn catalog_at(index: usize) -> Option<&'static [(&'static str, &'static str)]> {
    let locale = generated::LOCALE_IDS.get(index)?;
    generated::CATALOGS
        .iter()
        .find(|(catalog_locale, _)| catalog_locale == locale)
        .map(|(_, table)| *table)
}

fn lookup(table: &[(&'static str, &'static str)], key: &str) -> Option<&'static str> {
    table.iter().find_map(|(candidate, value)| (*candidate == key).then_some(*value))
}

#[cfg(test)]
fn reset_for_test(locale: &str) {
    if let Some(locale) = parse_locale(locale) {
        CURRENT.store(locale, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_locale_is_latin() {
        reset_for_test("la");
        assert_eq!(current(), "la");
        assert_eq!(
            translate("kernel.boot.hint.terminal", "Press F12 for a terminal"),
            "Preme F12 prō terminālī"
        );
    }

    #[test]
    fn cmdline_language_sets_kernel_locale() {
        set_from_cmdline("loglevel=info language=syc");
        assert_eq!(current(), "syc");
        assert_eq!(translate("kernel.boot.booting", "BOOTING..."), "ܫܪܐ...");

        set_from_cmdline("loglevel=info language=la");
        assert_eq!(current(), "la");
        assert_eq!(translate("kernel.boot.booting", "BOOTING..."), "INITIUM AGITUR...");
    }

    #[test]
    fn non_language_switch_is_not_a_language_selector() {
        reset_for_test("la");
        set_from_cmdline("loglevel=info lang=syc");
        assert_eq!(current(), "la");
    }

    #[test]
    fn source_text_milestones_are_loaded_from_catalogs() {
        set_language("la");
        assert_eq!(translate("Memory Map OK", "Memory Map OK"), "Tabula memoriae bona");
        assert_eq!(translate("Entering Scheduler", "Entering Scheduler"), "Ordinātor initur");
    }

    #[test]
    fn unknown_locale_does_not_replace_current_locale() {
        reset_for_test("la");
        set_from_cmdline("loglevel=info language=missing");
        assert_eq!(current(), "la");
    }
}
