//! Declarative macros for creating handles.

/// Create a Handle<T> from a struct literal.
///
/// Example:
/// ```ignore
/// let w = handle!(Window { x: 10, y: 20, ... });
/// ```
#[macro_export]
macro_rules! handle {
    ($val:expr) => {{ $crate::Handle::new($val) }};
}

/// Backward-compatible alias for [`handle!`].
#[macro_export]
macro_rules! thing {
    ($val:expr) => {{ $crate::handle!($val) }};
}

/// Create a Handle<Edge>.
///
/// Example:
/// ```ignore
/// let e = edge!(from, pred, to, flags);
/// ```
#[macro_export]
macro_rules! edge {
    ($from:expr, $pred:expr, $to:expr, $flags:expr $(,)?) => {{
        $crate::Handle::new($crate::graphable::Edge {
            from: $from,
            predicate: $pred,
            to: $to,
            flags: $flags,
        })
    }};
}
