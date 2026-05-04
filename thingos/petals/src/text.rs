use alloc::string::String;

const ELLIPSIS: &str = "...";

pub fn ellipsize_ascii(text: &str, capacity: usize) -> String {
    if text.chars().count() <= capacity {
        return String::from(text);
    }
    if capacity == 0 {
        return String::new();
    }
    if capacity <= ELLIPSIS.len() {
        return ELLIPSIS.chars().take(capacity).collect();
    }

    let keep = capacity - ELLIPSIS.len();
    let mut out = String::new();
    for ch in text.chars().take(keep) {
        out.push(ch);
    }
    out.push_str(ELLIPSIS);
    out
}

#[cfg(test)]
mod tests {
    use super::ellipsize_ascii;

    #[test]
    fn clipped_text_uses_ellipsis_marker() {
        assert_eq!(ellipsize_ascii("Applications", 8), "Appli...");
    }

    #[test]
    fn clipped_text_respects_tiny_capacities() {
        assert_eq!(ellipsize_ascii("Applications", 0), "");
        assert_eq!(ellipsize_ascii("Applications", 1), ".");
        assert_eq!(ellipsize_ascii("Applications", 2), "..");
        assert_eq!(ellipsize_ascii("Applications", 3), "...");
    }

    #[test]
    fn text_that_fits_is_unchanged() {
        assert_eq!(ellipsize_ascii("Calc", 4), "Calc");
    }
}
