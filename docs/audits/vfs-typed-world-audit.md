# VFS Typed-World Audit

**Issue:** Audit VFS Through Typed-World Lens  
**Status:** Initial audit  
**Scope:** `kernel/src/vfs/*`, `kernel/src/syscall/handlers/vfs.rs`, provider-mounted subtrees, and typed-world architecture docs

---

## 1) Inventory of major VFS concepts

| Concept | Current implementation surface | Notes |
|---|---|---|
| Global mount projection | `kernel/src/vfs/mod.rs` (`mount::mount`) | Boot mounts include `/`, `/dev`, `/proc`, `/sys`, `/tmp`, `/run`, `/services`, `/session`, `/data` |
| Node access form | `VfsNode` trait (`read/write/readdir/stat/poll`) | Primary compatibility access form over Things |
| Filesystem backend projection | `VfsDriver` trait (`lookup/create/mkdir/...`) | Backends define Place-shape for each mounted subtree |
| Handle/reference projection | per-process thing/fd table | `open` materializes compatibility handles (`fd`/`IpcThing`) |
| Device projection | `/dev` (`devfs`) | Built-ins (`console/null/zero/fb0/...`) + runtime registration |
| Process/system projection | `/proc` (`procfs`) | Mixed: compatibility process surface + canonical typed bridges (`task/job/group/authority/place`) |
| Hardware metadata projection | `/sys` (`sysfs`) | Read-only projection of kernel device registry + firmware metadata |
| Runtime/session/service places | `/run`, `/services`, `/session` | Coordination/rendezvous namespaces; often mounted as ramfs then populated by providers |
| Scratch/persistence compatibility places | `/tmp`, `/data` | Volatile/persistent-like path conventions as practical substrate |
| Userland provider projections | `SYS_FS_MOUNT` via `provider.rs`, `stem::syscall::vfs_mount` | Providers mount subtrees like `/mnt/iso`, `/net`, `/dev/audio/card0` |

---

## 2) Classification table (Thing / Form / Place lens)

| VFS concept | Typed-world mapping | Classification |
|---|---|---|
| Path lookup | Relation traversal in a caller-visible **Place** | **Clean canonical mapping** |
| Mounted subtree (`/dev`, `/sys`, `/services`, `/session`, `/run`) | Concrete **Place projection** | **Clean canonical mapping** |
| `VfsNode` instance | One access **Form** of an underlying **Thing** | **Clean canonical mapping** |
| `readdir` | Projection of visible relations in Place | **Projection view (not ontology truth)** |
| `stat` / mode bits / inode-like fields | Compatibility metadata form | **Compatibility projection only** |
| `open` + fd / `IpcThing` | Compatibility reference form to a Thing/Form | **Compatibility projection only** |
| `poll` readiness flags | Event/wait projection over typed events/state | **Projection form** |
| `/proc/<pid>/status`, cmdline, pid-centric shape | Unix process compatibility vocabulary | **Compatibility projection only** |
| `/proc/<pid>/task_state`, `job_state`, `authority`, `place` | Canonical typed concepts exported through file form | **Canonical concepts projected through VFS** |
| Global mount table + no mount privilege check today | Namespace/authority policy still transitional | **Transitional risk area** |

---

## 3) Required question answers

### Which VFS concepts map cleanly to Thing / Form / Place?
- **Thing:** underlying device/process/channel/service/resource entities referenced behind nodes and handles.
- **Form:** `VfsNode`, path strings, fd/`IpcThing`, `poll` readiness, `stat` payloads.
- **Place:** mounted namespace topology and path visibility (`/dev`, `/proc`, `/sys`, `/run`, `/services`, `/session`, `/tmp`, `/data`).

### Which parts of the VFS are compatibility projection only?
- POSIX-style metadata schema (`stat`, mode bits, inode-like identifiers).
- fd integer lifecycle as a reference mechanism.
- Unix process-shaped `/proc` slices (`status`, pid/cmdline/exe conventions).
- Byte-stream-centric semantics where used as compatibility transport.

### Where does “everything is a file” help?
- Uniform discovery and access (`open/read/write/readdir/poll`) across kernel and userland providers.
- Fast bring-up and debuggability (`cat`/`ls` style operability).
- Stable substrate for service composition via mounted namespaces (`/services`, `/run`, `/session`).

### Where does it distort the typed-world model?
- Treating compatibility forms (`fd`, inode-like metadata, pid-shaped proc views) as canonical identity.
- Collapsing distinct canonical concepts (Thing identity, Authority, Place, lifecycle) into filesystem-only vocabulary.
- Risk of conflating projection ordering (`readdir`/path layout) with ontology ownership.

### Which current pseudo-filesystems are best understood as Place projections?
- **Strong Place projections:** `/dev`, `/sys`, `/run`, `/services`, `/session`, `/tmp`, `/data`.
- **Mixed projection:** `/proc` (contains both compatibility process views and canonical typed bridges).
- **Provider sub-places:** mounted userland subtrees such as `/mnt/iso`, `/net`, `/dev/audio/card0`.

---

## 4) Risks

1. **Ontology drift toward file/fd truth:** reviewers may treat VFS compatibility artifacts as canonical model definitions.
2. **Mixed `/proc` semantics ambiguity:** canonical typed paths and Unix-compat paths coexist without a single explicit taxonomy marker.
3. **Namespace/authority gap:** current global mount table and ungated `SYS_FS_MOUNT` can blur Place ownership and authority boundaries.
4. **Projection leakage into API design:** new subsystems may overfit to byte/path forms before defining canonical typed semantics.

---

## 5) Concrete next-step recommendations (cleanup opportunities)

1. **Add canonical-vs-compat labels to `/proc` docs/comments** (especially for `status/cmdline/exe` versus typed `task/job/authority/place` files).
2. **Document Place-projection intent per mount** in one maintainer-facing map (boot mounts + common provider mounts).
3. **Gate `SYS_FS_MOUNT` with explicit authority policy** and tie it to namespace ownership milestones.
4. **Require canonical owner declaration in VFS PRs**: each new node/path should state underlying Thing/Place/Authority owner and projection role.
5. **Track migration of process-compat surfaces** toward typed snapshots where needed, keeping compatibility files but preventing semantic expansion.

