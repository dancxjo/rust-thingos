//! Authority module: canonical permission context bridging the current
//! `Process`-shaped credential model toward the future `Authority` ontology.
//!
//! # What Authority is
//!
//! `Authority` is the fourth canonical axis introduced by the phased migration:
//!
//! * **Task**      — execution
//! * **Job**       — lifecycle
//! * **Group**     — coordination
//! * **Authority** — permission context (this module)
//!
//! An `Authority` is the explicit, first-class answer to the question
//! *"under what power does this action occur?"*.  The bridge now exposes
//! principal (`uid`/`gid`) plus capability bits from `Process` backing, while
//! preserving a human-readable `name` label.
//!
//! # Canonical entry points for new authorization code
//!
//! New authorization checks **must** use the helpers in `bridge`:
//!
//! | Helper | Purpose |
//! |--------|---------|
//! | `bridge::authority_for_current()` | Obtain the current task's Authority without touching Process fields directly |
//! | `bridge::authority_from_snapshot(snap)` | Build an Authority from a `ProcessSnapshot` (used by procfs paths) |
//! | `bridge::check_privilege(auth, priv)` | Gate a privileged operation through Authority semantics |
//!
//! New code that needs to answer "is the caller allowed to do X?" should call
//! `authority_for_current()` followed by `check_privilege(...)`, not reach
//! into Process fields.
//!
//! # Transitional mapping
//!
//! The current kernel still stores all permission-bearing state inside
//! `Process` (including transitional `authority` backing). `kernel::authority::bridge`
//! is the **single translation point**
//! from the internal `ProcessSnapshot` into the canonical `Authority` type.
//!
//! # Future direction
//!
//! Once uid/gid-like fields, capability masks, or a service-account substructure
//! are added to `Process` (Phase 5 authority substructure), this module will
//! surface them through the bridge.  Until then the Unix-shaped internal
//! structures remain operational and only their *meaning at system boundaries*
//! is replaced.

pub mod bridge;
