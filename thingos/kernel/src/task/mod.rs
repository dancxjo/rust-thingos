//! Kernel task management: `Process`, `Thread<R>`, and transitional bridges.
//!
//! # Ontology note
//!
//! The `Process` struct in this module is a **compatibility projection** —
//! it bundles `Job`, `Space`, `Authority`, `Place`, and `Task` concerns that
//! are canonical in the ThingOS typed-world ontology.  New system semantics
//! must not be added to `Process` directly; they belong to one of the five
//! canonical owners.
//!
//! See `docs/architecture/ontology.md` for the full typed-world definitions
//! and `docs/architecture/unix-projection.md` for the projection rules.
//! See `docs/architecture/process-projection.md` for the enforced rule that
//! `Process` is projection/backing only, not architectural truth.
pub mod bridge;
pub mod exec;
pub mod identity;
pub mod loader;
pub mod registry;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::vec::Vec;

use abi::types::StackInfo;
use spin::Mutex;

pub use crate::sched::Scheduler;
use crate::simd::SimdState;
use crate::{BootRuntime, BootTasking, sched as scheduler};

pub type ThreadId = crate::sched::state::ThreadId;
/// Backward-compatible alias — prefer `ThreadId` in new code.
pub type TaskId = ThreadId;
pub use crate::sched::state::{Affinity, ThreadPriority, ThreadState};
/// Backward-compatible alias — prefer `ThreadPriority` in new code.
pub type TaskPriority = ThreadPriority;
/// Backward-compatible alias — prefer `ThreadState` in new code.
pub type TaskState = ThreadState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupArg {
    None,
    BootRegistry,
    DeviceId(u64),
    Raw(usize),
}

impl StartupArg {
    pub fn to_raw(self) -> usize {
        match self {
            StartupArg::None => 0,
            StartupArg::BootRegistry => 0x600000,
            StartupArg::DeviceId(id) => id as usize,
            StartupArg::Raw(val) => val,
        }
    }
}

/// Address-space subdivision of a `Process`.
///
/// This struct groups all memory/address-space concerns that are conceptually
/// owned by **Space**, not by `Process`, `Job`, or `Task`.  It lives inside
/// `Process` as a transitional measure: the fields are here because a
/// first-class `Space` kernel object does not yet exist, but they are
/// deliberately separated so that future extraction into `Space` is obvious
/// and mechanical.
///
/// # Conceptual future ownership
///
/// In the emerging ThingOS object model:
/// - **Task** owns execution context (registers, stack, scheduling state).
/// - **Job** owns process lifecycle (creation, exit, reaping).
/// - **Space** owns address-space state: the page-table root, the virtual
///   memory map, user-stack / program-image layout, and all mapping metadata.
///
/// New kernel code that needs to add memory-ownership state should add it
/// **here**, not directly to `Process`.  When `Space` is introduced as a
/// first-class object this subdivision is the extraction seam.
///
/// # Future extraction seams
///
/// Likely follow-on cuts once a first-class `Space` is introduced:
/// - Promote `ProcessAddressSpace` into `Space` and share it across processes
///   that map the same image (copy-on-write / shared mappings).
/// - Separate the `exec` transition helpers that currently live in
///   `kernel::task::exec` and reach into this struct directly.
/// - Expose canonical `Space` identity at syscall boundaries
///   (e.g. `SYS_VM_MAP` currently operates on the implicit current space).
/// - Untangle `fork`/`exec` assumptions: today `exec` replaces both
///   `mappings` and `aspace_raw` in-place; a real `Space` swap would instead
///   atomically replace the entire `ProcessAddressSpace`.
pub struct ProcessAddressSpace {
    /// First-class `Space` kernel object (Phase 1 — transitional).
    ///
    /// Wraps the same `mappings` `Arc` and carries a stable
    /// [`crate::space::Space::id`] (`SpaceId`) for diagnostics and future
    /// handle-based ABI exposure.
    ///
    /// In Phase 1 this lives inside `ProcessAddressSpace` (alongside
    /// `mappings` and `aspace_raw`) so that **no call site that constructs
    /// `ProcessAddressSpace` needs to change** — the constructors
    /// (`empty`, `from_parts`) automatically create the corresponding
    /// `Space` object.  Once the extraction is complete, this field will
    /// become the sole VM anchor and `mappings`/`aspace_raw` will move
    /// inside `Space`.
    ///
    /// # Usage rules
    ///
    /// * Read `space.space_obj.id` when you need a stable `SpaceId`.
    /// * Read `space.space_obj.mapping_count()` for a diagnostic region count.
    /// * Use `space.mappings_arc()` when a cached clone is needed.
    /// * Use `space.aspace_raw()` / `space.set_aspace_raw()` for page-table
    ///   token reads/writes.
    pub space_obj: alloc::sync::Arc<crate::space::Space>,
}

impl ProcessAddressSpace {
    /// Create a fresh, empty address space subdivision (no mappings, no page-table root).
    ///
    /// Used when spawning a new process before ELF loading assigns a real
    /// address space.  A fresh `Space` kernel object is created automatically
    /// and shares the same `Arc<Mutex<MappingList>>`.
    pub fn empty() -> Self {
        let space_obj = crate::space::Space::new_empty();
        ProcessAddressSpace { space_obj }
    }

    /// Create an address space subdivision from an existing mappings `Arc` and
    /// a raw page-table token.
    ///
    /// Used by the spawn path after the ELF loader has populated the address
    /// space and the architecture runtime has converted the handle to a raw token.
    /// A fresh `Space` kernel object is created automatically and shares the
    /// provided `mappings` `Arc`.
    pub fn from_parts(
        mappings: alloc::sync::Arc<spin::Mutex<crate::memory::mappings::MappingList>>,
        aspace_raw: u64,
    ) -> Self {
        let space_obj = crate::space::Space::from_parts(mappings, aspace_raw);
        ProcessAddressSpace { space_obj }
    }

    /// Clone the shared mappings arc used by this space.
    pub fn mappings_arc(
        &self,
    ) -> alloc::sync::Arc<spin::Mutex<crate::memory::mappings::MappingList>> {
        alloc::sync::Arc::clone(&self.space_obj.mappings)
    }

    /// Borrow the shared mappings list for lock-based access.
    pub fn mappings(&self) -> &alloc::sync::Arc<spin::Mutex<crate::memory::mappings::MappingList>> {
        &self.space_obj.mappings
    }

    /// Return the current architecture-specific page-table token.
    pub fn aspace_raw(&self) -> u64 {
        self.space_obj.aspace_raw()
    }

    /// Update the architecture-specific page-table token.
    pub fn set_aspace_raw(&self, aspace_raw: u64) {
        self.space_obj.set_aspace_raw(aspace_raw);
    }
}

/// Backward-compatible alias for the kernel Job object.
///
/// Transitional code that still references `ProcessLifecycle` now points to
/// the first-class lifecycle owner in `crate::job::Job`.
pub type ProcessLifecycle = crate::job::Job;

/// Delivery strategy used when a message is enqueued into a process inbox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageDeliveryKind {
    /// One sender targeted one specific recipient.
    Direct,
    /// One sender targeted a group and this recipient was reached via fanout.
    GroupBroadcast,
}

/// Delivery metadata carried alongside a queued process message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessMessageMetadata {
    /// Thread ID of the sender at enqueue time.
    pub sender_tid: ThreadId,
    /// Provisional sender Job ID (currently the sender process PID), if known.
    pub sender_job: Option<u32>,
    /// Target process-group ID for group fanout deliveries.
    pub target_group: Option<u32>,
    /// Delivery strategy used to enqueue the message.
    pub delivery_kind: MessageDeliveryKind,
    /// Per-broadcast sequence index assigned during fanout.
    pub broadcast_sequence: Option<u64>,
}

/// Message record stored in a process inbox.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessMessage {
    /// Canonical typed message envelope.
    pub message: crate::message::Message,
    /// Metadata about how this message was delivered.
    pub metadata: ProcessMessageMetadata,
}

/// Process inbox enqueue errors for prototype typed delivery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MessageEnqueueError {
    /// Inbox reached its bounded capacity.
    InboxFull { capacity: usize },
}

/// Fixed prototype inbox capacity per process.
pub const PROCESS_MESSAGE_INBOX_CAPACITY: usize = 64;

/// Unix legacy compatibility state carried by a `Process`.
///
/// This struct is the **explicit quarantine boundary** for all Unix-derived
/// state that is kept for compatibility but is **not** architectural truth in
/// ThingOS.  It lives inside [`Process`] as a named subdivision so that:
///
/// * A reader immediately sees which concepts are transitional legacy.
/// * New code is forced to touch `unix_compat.*` rather than top-level
///   `Process` fields, making every new use of legacy state visible at code
///   review time.
/// * The extraction seam for each field is documented here, not scattered.
///
/// # ⚠️ Do not add new fields here
///
/// New kernel code MUST NOT add fields to `ProcessUnixCompat` unless
/// absolutely forced by a Unix compatibility requirement.  All new public
/// paths should use Task / Job / Group / Place / Authority vocabulary instead.
///
/// # Field inventory and future homes
///
/// | Field            | Unix origin                    | Future ThingOS home              |
/// |------------------|--------------------------------|----------------------------------|
/// | `signals`        | POSIX per-process signal state | Message / Inbox / Group broadcast|
/// | `pgid`           | Process-group ID               | Group (Phase 5)                  |
/// | `sid`            | Session ID                     | Group / Presence / Place (Phase 5)|
/// | `session_leader` | Session-leader flag            | Group / Presence (Phase 5)       |
/// | `spawn_record`   | Spawn-time argv + ELF auxv     | `kernel::spawn::bridge::SpawnRecord` |
/// | `env`            | Inherited environment map      | Place / Authority context        |
///
/// # Note on Presence
///
/// Presence (terminal attachment, UI/console ownership, person-in-place) has
/// not yet been introduced.  When it is, controlling-TTY state currently
/// embedded in `signals` (SIGTTOU/SIGTTIN, job-control stop) will move there,
/// **not** into Place.
pub struct ProcessUnixCompat {
    /// Per-process signal state: dispositions, pending set, stop/alarm state.
    ///
    /// Includes the controlling-terminal glue (SIGTTOU/SIGTTIN, job-control
    /// stop signals) that properly belongs to Group / Presence once those
    /// concepts are introduced.
    ///
    /// FUTURE: → Message / Inbox / Group broadcast
    pub signals: crate::signal::ProcessSignals,

    /// Prototype typed message inbox used by direct and group fanout delivery.
    ///
    /// This queue is deliberately bounded to force explicit partial-failure
    /// behavior in early broadcast prototypes.
    pub message_inbox: VecDeque<ProcessMessage>,

    /// Process group ID.
    ///
    /// FUTURE: → Group (Phase 5)
    pub pgid: u32,

    /// Session ID.
    ///
    /// FUTURE: → Group / Presence / Place (Phase 5)
    pub sid: u32,

    /// True when this process is the leader of its session.
    ///
    /// FUTURE: → Group / Presence (Phase 5)
    pub session_leader: bool,

    /// Immutable typed spawn metadata (`argv` + `auxv`).
    ///
    /// This replaces direct `argv`/`auxv` storage so readers can only consume
    /// spawn-time context through a typed record.
    ///
    /// FUTURE: → first-class SpawnRecord object attached to Job
    spawn_record: crate::spawn::bridge::SpawnRecord,

    /// Environment variables inherited at spawn/exec time.
    ///
    /// FUTURE: → Place / Authority context propagation
    pub env: BTreeMap<Vec<u8>, Vec<u8>>,
    // `argv` and `auxv` are intentionally no longer stored as standalone
    // mutable blobs; they are represented by `spawn_record`.
}

impl ProcessUnixCompat {
    /// Create Unix compatibility state for an **isolated** process (no parent).
    ///
    /// Sets `pgid` and `sid` to `pid`.  Pass `is_session_leader = true` when
    /// this process bootstraps a new session (e.g. the root init process);
    /// `false` otherwise.
    pub fn isolated(pid: u32, is_session_leader: bool) -> Self {
        ProcessUnixCompat {
            signals: crate::signal::ProcessSignals::new(),
            message_inbox: VecDeque::new(),
            pgid: pid,
            sid: pid,
            session_leader: is_session_leader,
            spawn_record: crate::spawn::bridge::SpawnRecord::empty(),
            env: BTreeMap::new(),
        }
    }

    /// Create Unix compatibility state for a **child** process that inherits
    /// session/group membership from `parent`.
    ///
    /// The child is never a session leader, inherits `pgid`/`sid` from the
    /// parent, and inherits the parent's environment map.
    pub fn inherit(parent: &ProcessUnixCompat) -> Self {
        ProcessUnixCompat {
            signals: crate::signal::ProcessSignals::new(),
            message_inbox: VecDeque::new(),
            pgid: parent.pgid,
            sid: parent.sid,
            session_leader: false,
            spawn_record: crate::spawn::bridge::SpawnRecord::empty(),
            env: parent.env.clone(),
        }
    }

    /// Enqueue one typed message into the process inbox.
    pub fn enqueue_message(&mut self, msg: ProcessMessage) -> Result<(), MessageEnqueueError> {
        if self.message_inbox.len() >= PROCESS_MESSAGE_INBOX_CAPACITY {
            return Err(MessageEnqueueError::InboxFull {
                capacity: PROCESS_MESSAGE_INBOX_CAPACITY,
            });
        }
        self.message_inbox.push_back(msg);
        Ok(())
    }

    /// Pop the next queued typed message from the process inbox.
    pub fn dequeue_message(&mut self) -> Option<ProcessMessage> {
        self.message_inbox.pop_front()
    }

    /// Current inbox depth for diagnostics and tests.
    pub fn message_inbox_len(&self) -> usize {
        self.message_inbox.len()
    }

    /// Borrow immutable typed spawn metadata.
    pub fn spawn_record(&self) -> &crate::spawn::bridge::SpawnRecord {
        &self.spawn_record
    }

    /// Replace immutable typed spawn metadata atomically.
    pub fn set_spawn_record(&mut self, spawn_record: crate::spawn::bridge::SpawnRecord) {
        self.spawn_record = spawn_record;
    }

    /// Convenience wrapper to rebuild spawn metadata from raw parts.
    pub fn set_spawn_context(&mut self, argv: Vec<Vec<u8>>, auxv: Vec<(u64, u64)>) {
        self.spawn_record = crate::spawn::bridge::SpawnRecord::new(argv, auxv);
    }
}

/// Compatibility process aggregation object.
///
/// A `Process` is a **thin runtime aggregator/wiring shell** that composes
/// canonical first-class owners (`Job`, `Space`, `Authority`, `Place`, etc.)
/// for compatibility surfaces and scheduler plumbing.  It is not semantic
/// architectural authority.
///
/// Every user process has exactly one `Process`, shared via
/// `Arc<Mutex<Process>>` by all threads that belong to it.  Kernel-only
/// threads have `process_info = None`.
///
/// # Aggregation model
///
/// | Concern / resource      | Canonical owner                | `Process` role today                              |
/// |-------------------------|--------------------------------|---------------------------------------------------|
/// | Lifecycle               | `crate::job::Job`              | Holds/composes `job` for runtime wiring           |
/// | VM address space        | `crate::space::Space`          | Holds/composes `space` extraction seam            |
/// | Authority               | `thingos::authority::Authority`| Transitional backing + compatibility projection   |
/// | World context           | `thingos::place::Place`        | Transitional backing + compatibility projection   |
/// | Coordination / presence | `thingos::group::Group` + `thingos::presence::Presence` | Exposes compatibility/session projections |
/// | Unix compatibility      | legacy compat boundary         | Quarantined in `unix_compat`                      |
///
/// # Locking rules
///
/// The `Process` mutex must **never** be acquired while the scheduler lock
/// (`SCHEDULER.lock()`) is held — reverse order causes deadlock.
///
/// # Responsibility classification (Phase 9 migration inventory)
///
/// Fields in this struct are grouped by their intended canonical destination
/// in the phased migration.  Fields are **not** yet extracted; they remain
/// here as transitional backing.  All public-facing access should go through
/// the appropriate bridge module.
///
/// **Lifecycle** (Job — `kernel::job::bridge`):
/// * `job` — first-class [`crate::job::Job`], contains `ppid`,
///   `thread_ids`, `exec_in_progress`, and `children_done`.
///   New code must not add lifecycle state directly to `Process`; add it to
///   `crate::job::Job` instead.
///
/// **Place** (world/visibility context — `kernel::place::bridge`):
/// * `cwd` — current working directory path → `Place::cwd`
/// * `namespace` — VFS mount-table view → `Place::namespace`
/// * `root` — effective filesystem root (`chroot` / `pivot-root`) → `Place::root`
///
/// **Unix legacy compatibility** (quarantined — see [`ProcessUnixCompat`]):
/// * `unix_compat` — all Unix-derived state (`signals`, `pgid`, `sid`,
///   `session_leader`, `spawn_record`, `env`).
///   These fields are NOT architectural truth; they are kept behind an
///   explicit compatibility boundary.  New code MUST NOT add to
///   `ProcessUnixCompat` without a Unix compatibility justification.
///
/// **Group** (coordination domain — `kernel::group::bridge`):
/// * `unix_compat.pgid` / `unix_compat.sid` / `unix_compat.session_leader`
///   — also used by Group bridge today
///
/// **Authority** (permission context — `kernel::authority::bridge`):
/// * `exec_path` — used as authority name fallback today
/// * `authority` — transitional principal/capability backing (`uid`,`gid`,`capability_mask`)
///
/// **Identity** (shared between Job and Space — not yet extracted):
/// * `pid` — TGID; doubles as lifecycle ID (→ `Job`) and address-space tag
///   (→ `Space`).  Split deferred until `Space` identity is separated.
///
/// **Space** (address-space ownership — future `Space` kernel object):
/// * `space` — grouped under [`ProcessAddressSpace`]; contains `mappings` and
///   `aspace_raw`.  New code must not attach additional memory-ownership state
///   directly to `Process`; add it to `ProcessAddressSpace` instead.
///
/// See `docs/concepts/process-object.md` for the full design document.
/// See `docs/migration/concept-mapping.md` §2 (Process → Job + Space + Authority + Place + Task(s))
/// for the canonical legacy→ThingOS mapping and naming rules.
/// See `docs/architecture/process-projection.md` for review-blocking rules that
/// prevent re-centering architecture on `Process`.
pub struct Process {
    /// Thread Group ID — the PID of the thread-group leader.
    ///
    /// `pid` is kept at the top level of `Process` rather than inside
    /// `job` because it currently doubles as both the lifecycle identity
    /// (→ future `Job`) and the address-space identity (→ future `Space`).
    /// Once those two responsibilities are separated, `pid` will migrate into
    /// `ProcessLifecycle` alongside the other `Job`-bound fields.
    pub pid: u32,

    // ── Lifecycle subdivision (future `Job` kernel object) ────────────────────
    // Lifecycle concerns are grouped here rather than scattered across `Process`.
    // This subdivision is the extraction seam for a future first-class `Job`
    // object.  Do NOT add new lifecycle state directly to top-level `Process` —
    // add it to `ProcessLifecycle` instead.
    /// First-class job object containing lifecycle ownership.
    pub job: crate::job::Job,

    // ── Unix legacy compatibility boundary ───────────────────────────────────
    // ALL Unix-derived compatibility state lives here, behind an explicit
    // named boundary.  This makes legacy baggage visible in every code review.
    //
    // DO NOT move fields out of `unix_compat` back into top-level `Process`.
    // DO NOT add new Unix compatibility state to top-level `Process` directly.
    // Any unavoidable Unix compatibility code must live inside this boundary.
    /// Unix legacy compatibility state — signals, pgid, sid, spawn_record, env.
    ///
    /// See [`ProcessUnixCompat`] for the full field inventory, future homes,
    /// and the rules that govern this boundary.
    pub unix_compat: ProcessUnixCompat,

    // ── Resource table ────────────────────────────────────────────────────────
    // Future: handle_table will move to a resource-authority domain.  For now it
    // remains as transitional Process baggage.
    /// File descriptor table — fds 0/1/2 pre-populated at spawn time.
    pub handle_table: crate::vfs::handle_table::HandleTable,

    /// Per-process IPC handle table.
    pub ipc_table: crate::ipc::IpcHandleTable,

    // ── Place context (Phase 8 — world/visibility boundary) ──────────────────
    // These fields answer "in what world does this execution happen?".
    // They feed `kernel::place::bridge` → `thingos::place::Place`.
    // They remain in `Process` as transitional backing; the canonical surface
    // is through the place bridge, not direct field access from new code.
    /// VFS namespace identity for this process.
    ///
    /// Backed by [`crate::vfs::NamespaceRef`] and surfaced through
    /// `kernel::place::bridge` as canonical `Place::namespace`.
    ///
    /// Feeds `Place::namespace` through `kernel::place::bridge`.
    ///
    /// See `docs/concepts/namespaces.md` for the behaviour matrix and roadmap.
    pub namespace: crate::vfs::NamespaceRef,

    /// Current working directory.
    ///
    /// Feeds `Place::cwd` through `kernel::place::bridge`.
    /// New code must not read this field directly for world-context purposes;
    /// use `crate::place::bridge::place_from_snapshot` instead.
    pub cwd: alloc::string::String,

    /// Effective filesystem root for this process.
    ///
    /// This is the transitional backing for canonical `Place::root` and is
    /// expected to track chroot/pivot-root style world-context changes.
    pub root: alloc::string::String,

    /// Path of the currently-running executable image.
    pub exec_path: alloc::string::String,

    /// Transitional backing for canonical `Authority`.
    pub authority: ProcessAuthority,

    // ── Space context (future `Space` kernel object) ──────────────────────────
    // Address-space concerns are grouped here rather than scattered across
    // `Process`.  This subdivision is the extraction seam for a future
    // first-class `Space` object.  Do NOT attach new memory-ownership state
    // directly to `Process` — add it to `ProcessAddressSpace` instead.
    /// Address-space subdivision — transitional backing for canonical `Space`.
    ///
    /// Contains the VM mapping list, the architecture-specific page-table
    /// token, and the first-class [`crate::space::Space`] object wrapper.
    /// See [`ProcessAddressSpace`] for the full design rationale.
    pub space: ProcessAddressSpace,
}

/// Backward-compatible alias — prefer `Process` in new code.
///
/// `ProcessInfo` is a **compatibility/runtime adapter name**, not a canonical
/// semantic owner.  Canonical ownership remains in generated kinds
/// (`thingos::job`, `thingos::space`, `thingos::group`, `thingos::task`) and
/// bridge modules convert between this transitional backing and those kinds.
pub type ProcessInfo = Process;

/// Snapshot of Unix compatibility fields exposed through the Process adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessUnixCompatProjection<'a> {
    pub argv: &'a [Vec<u8>],
    pub pgid: u32,
    pub sid: u32,
    pub session_leader: bool,
}

/// Transitional process-backed authority state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProcessAuthority {
    /// Principal user id.
    pub uid: u32,
    /// Principal group id.
    pub gid: u32,
    /// Capability bits granted to this process.
    pub capability_mask: u64,
}

impl ProcessAuthority {
    /// Privileged bootstrap/default authority.
    pub const fn root() -> Self {
        Self { uid: 0, gid: 0, capability_mask: u64::MAX }
    }

    /// Inherit principal and capability bits from parent.
    pub const fn inherit(parent: Self) -> Self {
        parent
    }
}

impl Process {
    /// Transitional runtime identity for scheduler/plumbing code.
    pub fn runtime_pid(&self) -> u32 {
        self.pid
    }

    /// Transitional runtime parent linkage for scheduler/plumbing code.
    pub fn runtime_parent_pid(&self) -> u32 {
        self.job.ppid
    }

    /// Explicit identity layering for one runtime task belonging to this process.
    pub fn identity_layers_for_tid(
        &self,
        tid: TaskId,
    ) -> crate::task::identity::TaskIdentityLayers {
        crate::task::identity::TaskIdentityLayers::for_process_task(
            tid,
            crate::task::identity::CompatibilityIds {
                pid: self.pid,
                pgid: self.unix_compat.pgid,
                sid: self.unix_compat.sid,
            },
            self.space.space_obj.id,
        )
    }

    /// Return `true` when `tid` is the thread-group leader TID for this job.
    pub fn is_job_leader_tid(&self, tid: TaskId) -> bool {
        self.identity_layers_for_tid(tid).is_job_leader()
    }

    /// Remove one thread from this job's lifecycle membership list.
    pub fn remove_thread_from_job(&mut self, tid: TaskId) {
        self.job.thread_ids.retain(|&t| t != tid);
    }

    /// Drain all tracked thread IDs from this job lifecycle.
    pub fn take_job_thread_ids(&mut self) -> Vec<TaskId> {
        core::mem::take(&mut self.job.thread_ids)
    }

    /// Job-exit observer inbox attached to this lifecycle, if any.
    pub fn job_exit_observer_inbox(&self) -> Option<crate::inbox::InboxId> {
        self.job.exit_observer_inbox
    }

    /// Runtime helper: gather live thread states for this job from a TID map.
    pub fn runtime_thread_states(
        &self,
        tid_state: &alloc::collections::BTreeMap<TaskId, TaskState>,
    ) -> Vec<TaskState> {
        self.job.thread_ids.iter().filter_map(|&tid| tid_state.get(&tid).copied()).collect()
    }

    /// Effective exit code for one runtime thread in this job.
    ///
    /// Leader threads prefer the Job-owned leader exit code once present.
    /// Non-leader threads use their thread-local exit code.
    pub fn effective_exit_code_for_tid(
        &self,
        tid: TaskId,
        thread_exit_code: Option<i32>,
    ) -> Option<i32> {
        if self.is_job_leader_tid(tid) {
            self.job.leader_exit_code.or(thread_exit_code)
        } else {
            thread_exit_code
        }
    }

    /// Compatibility projection wrapper for Unix-derived fields.
    pub fn unix_compat_projection(&self) -> ProcessUnixCompatProjection<'_> {
        ProcessUnixCompatProjection {
            argv: self.unix_compat.spawn_record().argv(),
            pgid: self.unix_compat.pgid,
            sid: self.unix_compat.sid,
            session_leader: self.unix_compat.session_leader,
        }
    }

    /// Canonical generated-kind projection for lifecycle/accounting state.
    ///
    /// `thingos::job::Job` currently carries only lifecycle state (schema v1/v2),
    /// so this adapter intentionally populates just `state`.
    pub fn canonical_job(&self, thread_states: &[TaskState]) -> thingos::job::Job {
        thingos::job::Job {
            state: crate::job::bridge::job_state_from_lifecycle(&self.job, thread_states),
        }
    }

    /// Canonical generated-kind projection for this process's `Space`.
    pub fn canonical_space(&self) -> thingos::space::Space {
        crate::space::bridge::space_from_arc(&self.space.space_obj)
    }

    /// Build a compatibility snapshot for one runtime task in this process.
    ///
    /// This keeps snapshot assembly bounded to `Process` aggregation/wiring so
    /// scheduler call sites do not read transitional ownership fields ad hoc.
    pub fn compatibility_snapshot_for_task(
        &self,
        tid: TaskId,
        task_name: alloc::string::String,
        task_state: TaskState,
        thread_exit_code: Option<i32>,
        foreground_pgid: Option<u32>,
        tid_state: &alloc::collections::BTreeMap<TaskId, TaskState>,
    ) -> crate::sched::hooks::ProcessSnapshot {
        let thread_states = self.runtime_thread_states(tid_state);
        let unix_compat = self.unix_compat_projection();
        let space = self.canonical_space();
        let job = self.canonical_job(&thread_states);
        crate::sched::hooks::ProcessSnapshot {
            pid: self.runtime_pid(),
            ppid: self.runtime_parent_pid(),
            tid,
            name: task_name,
            state: task_state,
            argv: unix_compat.argv.to_vec(),
            exec_path: self.exec_path.clone(),
            uid: self.authority.uid,
            gid: self.authority.gid,
            capability_mask: self.authority.capability_mask,
            exit_code: if job.state == thingos::job::JobState::Exited {
                self.effective_exit_code_for_tid(tid, thread_exit_code)
            } else {
                None
            },
            pgid: unix_compat.pgid,
            sid: unix_compat.sid,
            session_leader: unix_compat.session_leader,
            foreground_pgid,
            cwd: self.cwd.clone(),
            namespace_label: self.namespace.label(),
            root_path: self.root.clone(),
            thread_states,
            space_id: space.id,
            space_mapping_count: space.mapping_count,
            space_sharing_count: space.sharing_count,
        }
    }
}

/// Kernel representation of a single thread of execution.
///
/// Each `Thread<R>` corresponds to exactly one schedulable entity.  User
/// threads are created by `spawn_user_thread`; kernel threads by `spawn`.
///
/// # Fields split by concern
///
/// | Concern              | Fields                                               |
/// |----------------------|------------------------------------------------------|
/// | Identity             | `id`                                                 |
/// | Scheduler state      | `state`, `priority`, `timeslice_remaining`, …        |
/// | Execution context    | `ctx`, `kstack_*`, `aspace`, `user_fs_base`          |
/// | Per-thread flags     | `is_user`, `detached`, `pending_interrupt`           |
/// | Process reference    | `process_info` — shared `Arc<Mutex<Process>>`        |
/// | VM fast-path cache   | `mappings` — clone of `Process.mappings` (same Arc)  |
pub struct Thread<R: BootRuntime> {
    pub id: ThreadId,
    pub state: ThreadState,
    pub priority: ThreadPriority,
    pub exit_code: Option<i32>,
    /// Exit notification: level-triggered so status stays readable after wake.
    pub exit_waiters: crate::sched::WaitQueue,
    pub is_user: bool,
    pub wake_pending: bool,
    pub pending_interrupt: bool,
    pub affinity: Affinity,

    pub kstack_base: *mut u8,
    pub kstack_size: usize,
    pub kstack_top: u64,

    pub ctx: <R::Tasking as BootTasking>::Context,
    pub aspace: <R::Tasking as BootTasking>::AddressSpace,

    pub simd: SimdState,

    pub stack_info: Option<StackInfo>,

    /// VM mapping list — a clone of `Process.space.mappings` (same underlying `Arc`).
    ///
    /// Kept here for zero-lock fast access by the scheduler's per-CPU mapping
    /// cache (`CURRENT_MAPPINGS`).  Always updated atomically with
    /// `Process.space.mappings` during exec or thread creation.
    pub mappings: Arc<Mutex<crate::memory::mappings::MappingList>>,

    /// Remaining time slice in ticks before preemption.
    pub timeslice_remaining: u32,
    pub last_cpu: Option<usize>,

    /// Short human-readable name (e.g. "bristle", "idle/0").
    pub name: [u8; 32],
    pub name_len: u8,

    /// Owning process (shared across all threads in the group).
    ///
    /// `None` for pure kernel threads.
    pub process_info: Option<Arc<Mutex<Process>>>,

    /// Anti-starvation: tick when this thread was last enqueued.
    pub enqueued_at_tick: u64,

    /// Base priority before any aging boost.
    pub base_priority: ThreadPriority,

    /// Per-thread user-mode TLS base (FS_BASE on x86_64).
    pub user_fs_base: u64,

    /// `true` if this thread was created as detached (cannot be joined).
    pub detached: bool,

    /// Per-thread signal mask and thread-directed pending signals.
    pub signals: crate::signal::ThreadSignals,
}

/// Backward-compatible alias — prefer `Thread<R>` in new code.
pub type Task<R> = Thread<R>;

pub fn init<R: BootRuntime>() {
    crate::task::registry::init::<R>();
    crate::sched::init::<R>();
}
pub fn spawn<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: ThreadPriority,
    affinity: Affinity,
) -> ThreadId {
    crate::sched::spawn::<R>(entry, arg, priority, affinity)
}

pub fn spawn_with_priority<R: BootRuntime>(
    entry: extern "C" fn(usize) -> !,
    arg: StartupArg,
    priority: ThreadPriority,
) -> ThreadId {
    crate::sched::spawn_with_priority::<R>(entry, arg, priority)
}

pub unsafe fn block_current_erased() {
    unsafe {
        crate::sched::block_current_erased();
    }
}

pub unsafe fn wake_task_erased(tid: u64) {
    unsafe {
        crate::sched::wake_task_erased(tid);
    }
}

pub fn yield_now<R: BootRuntime>() -> bool {
    crate::sched::yield_now::<R>()
}

pub fn preempt_disable<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();
    {
        let lock = crate::sched::SCHEDULER.lock();
        let _lock_tracking = crate::sched::sched_lock_tracking_guard::<R>(rt.current_cpu_index());
        if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            sched.preempt_disable();
        }
    }
    rt.irq_restore(irq);
}

pub fn preempt_enable<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();

    let (switch_decision, deferred_prepare_ipis, deferred_registry_syncs) = {
        let lock = crate::sched::SCHEDULER.lock();
        let _lock_tracking = crate::sched::sched_lock_tracking_guard::<R>(rt.current_cpu_index());
        let result = if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            let switch = sched.preempt_enable();
            let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
            let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
            (switch, deferred_prepare_ipis, deferred_registry_syncs)
        } else {
            (None, alloc::vec::Vec::new(), alloc::vec::Vec::new())
        };
        result
    };
    crate::sched::apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
    crate::sched::send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);

    if let Some(decision) = switch_decision {
        let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
        let mut ghost_fs_base = 0;
        let Some(switch) =
            crate::sched::resolve_switch_params::<R>(decision, &mut ghost_ctx, &mut ghost_fs_base)
        else {
            rt.irq_restore(irq);
            return;
        };
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        let _cr3_after = rt.debug_active_aspace_root();

        unsafe {
            rt.tasking().switch_with_tls(
                &mut *switch.from_ctx,
                &*switch.to_ctx,
                switch.to_tid,
                switch.from_user_fs_base,
                switch.to_user_fs_base,
            );
        }
    }

    rt.irq_restore(irq);
}

pub fn resched_if_needed<R: BootRuntime>() {
    // Explicit safe-point check: yield if need_resched is set, but do NOT
    // run tick bookkeeping or decrement timeslices.
    let rt = crate::runtime::<R>();
    let irq = rt.irq_disable();

    let (switch_decision, deferred_prepare_ipis, deferred_registry_syncs) = {
        let lock = crate::sched::SCHEDULER.lock();
        let _lock_tracking = crate::sched::sched_lock_tracking_guard::<R>(rt.current_cpu_index());
        let result = if let Some(ptr) = *lock {
            let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
            let switch = sched.schedule_point(crate::sched::ScheduleReason::ReschedIfNeeded);
            let deferred_prepare_ipis = sched.drain_pending_prepare_schedule_ipis();
            let deferred_registry_syncs = core::mem::take(&mut sched.pending_registry_syncs);
            (switch, deferred_prepare_ipis, deferred_registry_syncs)
        } else {
            (None, alloc::vec::Vec::new(), alloc::vec::Vec::new())
        };
        result
    };
    crate::sched::apply_deferred_registry_syncs::<R>(deferred_registry_syncs);
    crate::sched::send_deferred_prepare_schedule_ipis::<R>(deferred_prepare_ipis);

    if let Some(decision) = switch_decision {
        let mut ghost_ctx = <R::Tasking as BootTasking>::Context::default();
        let mut ghost_fs_base = 0;
        let Some(switch) =
            crate::sched::resolve_switch_params::<R>(decision, &mut ghost_ctx, &mut ghost_fs_base)
        else {
            rt.irq_restore(irq);
            return;
        };
        let cr3_before = rt.debug_active_aspace_root();

        rt.tasking().activate_address_space(switch.to_aspace);

        let _cr3_after = rt.debug_active_aspace_root();

        unsafe {
            rt.tasking().switch_with_tls(
                &mut *switch.from_ctx,
                &*switch.to_ctx,
                switch.to_tid,
                switch.from_user_fs_base,
                switch.to_user_fs_base,
            );
        }
    }

    rt.irq_restore(irq);
}

pub fn dump_stats<R: BootRuntime>() {
    crate::sched::dump_stats::<R>();
}

/// Bootstrap a CPU for scheduling.
///
/// Must be called before the first yield on any CPU that doesn't already
/// have a current thread set (e.g. secondary CPUs).
fn bootstrap_cpu<R: BootRuntime>() {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let cpu_idx = crate::sched::current_cpu_index::<R>();

    crate::kinfo!("SMP: bootstrap_cpu start on CPU {}", cpu_idx);

    let lock = crate::sched::SCHEDULER.lock();
    let _lock_tracking = crate::sched::sched_lock_tracking_guard::<R>(cpu_idx);
    if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };

        if let Some(pc) = sched.state.per_cpu.get_mut(cpu_idx) {
            if pc.current.is_none() {
                // CPU hasn't been bootstrapped yet — set current to idle thread.
                if let Some(idle_id) = pc.idle_task {
                    pc.current = Some(idle_id);
                    rt.set_current_tid(idle_id);
                    sched.state.mark_cpu_online(cpu_idx);
                    crate::kinfo!(
                        "SMP: CPU {} bootstrapped with idle thread {} and is now schedulable",
                        cpu_idx,
                        idle_id
                    );

                    if let Some(mut t) = crate::task::registry::get_thread_mut::<R>(idle_id) {
                        t.state = ThreadState::Running;
                        rt.set_idle_task_current(true);
                    }
                } else {
                    crate::kerror!("SMP: CPU {} has no idle thread!", cpu_idx);
                }
            } else {
                crate::kdebug!(
                    "SMP: CPU {} bootstrap_cpu saw existing current thread {:?}",
                    cpu_idx,
                    pc.current
                );
            }
        }
    }
    rt.irq_restore(_irq);
}

pub fn run_scheduler<R: BootRuntime>() -> ! {
    crate::kinfo!("SMP: run_scheduler entry on CPU {}", crate::sched::current_cpu_index::<R>());
    // Bootstrap this CPU if needed (sets current thread for secondary CPUs).
    bootstrap_cpu::<R>();

    // Enable interrupts so this CPU can be preempted or woken from idle (HLT).
    crate::runtime::<R>().irq_restore(crate::IrqState(1));

    let mut idle_count: u64 = 0;
    loop {
        if !yield_now::<R>() {
            // No runnable work — halt until next IRQ (timer tick, device, IPI).
            crate::runtime::<R>().wait_for_interrupt();
            crate::sched::DIAG_HLT_WAKE.fetch_add(1, core::sync::atomic::Ordering::Relaxed);

            idle_count += 1;
            if idle_count % 1000 == 0 {
                let cpu = crate::sched::current_cpu_index::<R>();
                crate::ktrace!("SCHED: CPU {} idle pulse", cpu);
            }
        }
    }
}
