pub mod id;
pub mod sys;

// HandleId trait removed — use ThingId::from_u64/to_u64_lossy directly
pub use id::ThingId;

pub mod symbol;
