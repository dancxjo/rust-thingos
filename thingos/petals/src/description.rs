#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Description {
    Pressable,
    Textual,
    Focusable,
    Scrollable,
    Container,
    ChromeButton,
}
