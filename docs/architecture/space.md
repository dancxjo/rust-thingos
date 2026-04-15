# Space as the Canonical Memory Object

`Space` is the canonical owner of address-space identity and virtual-memory
state in ThingOS. `ProcessAddressSpace` remains a transitional container, but
the architectural truth is `Space`.

## 1) Canonical definition

### What a Space is

A `Space` answers:

> Where does memory live, and under what mapping rules?

### What Space owns

- stable address-space identity (`SpaceId`)
- mapping set (`MappingList`)
- page-table identity / architecture address-space token (`aspace_raw`)
- (future) memory-view policy, COW lineage, fault/accounting metadata

### What Space does not own

- lifecycle / parent-child / reaping semantics (`Job`)
- scheduling / runnable execution semantics (`Task`)
- credentials (`Authority`)
- cwd / namespace / world location (`Place`)

### How it differs from Job and Task

- **Space**: memory identity and view
- **Task**: schedulable execution context (registers, stack, priority, run state)
- **Job**: lifecycle state (spawn/exit/wait/reap, group lifecycle)

## 2) Responsibility boundary

Use this boundary when reviewing code:

- “Does this describe mapped bytes, mapping rules, or page-table identity?” → **Space**
- “Does this describe who runs next or CPU execution context?” → **Task**
- “Does this describe process lifetime, exit, parent/child, wait semantics?” → **Job**

### Explicit examples

Belongs to **Space**:
- VM mapping set
- page-table root / address-space token
- memory-view and sharing policy

Does **not** belong to Space:
- lifecycle transitions and exit status
- scheduling policy and run-queue state

## 3) Projection model (Unix compatibility)

Unix process memory assumptions are a projection, not ontology:

- Unix says “a process has an address space.”
- ThingOS says “a compatibility process currently *references* a canonical
  `Space`.”

Today this projection is implemented through `Process.space: ProcessAddressSpace`
with `space_obj: Arc<kernel::space::Space>` as the canonical anchor.
`kernel::space::bridge` is the required conversion path to public
`thingos::space::Space` views.

## 4) Migration guidance (current transitional seams)

The following fields are conceptually Space-owned and should continue to migrate
toward direct `Space` ownership:

- `ProcessAddressSpace.mappings` → `Space::mappings`
- `ProcessAddressSpace.aspace_raw` → `Space::aspace_raw`
- `ProcessAddressSpace.space_obj.id` → canonical identity (`SpaceId`)
- snapshot/diagnostic fields (`space_id`, `space_mapping_count`,
  `space_sharing_count`) → derived from `Arc<Space>`

Target direction:

- replace `Process.space: ProcessAddressSpace` with `Process.space: Arc<Space>`
- model `exec` as Space replacement instead of in-place field mutation
- keep public representations Space-first via `kernel::space::bridge`

## Transitional status (clarity for reviewers)

`ProcessAddressSpace` is an extraction seam, not the architectural owner.
When in doubt, ask:

> “If `Process` disappeared as a compatibility object, would this state still be
> needed to define memory identity?”

If yes, it belongs to **Space**.
