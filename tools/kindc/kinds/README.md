# Scheduler-adjacent canonical schema guidance

These `.kind` files define canonical semantic records for cross-boundary truth.

- Keep fields focused on identity, ownership, lifecycle, and stable links.
- Do **not** encode runtime hot-path caches (run-queue snapshots, mapping counters,
  scheduler-local denormalized data) in these canonical records.
- Runtime cache/projection data belongs in kernel/userland runtime structures and
  bridge layers, not in canonical Kind schemas.
