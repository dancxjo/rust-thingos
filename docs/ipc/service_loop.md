# Inbox-Backed Service Loops

> Status: Active
> Decision type: Architecture rule + supporting userspace primitive

## 1. The rule

A ThingOS service is an **inbox-backed actor**.

> A ThingOS service is an inbox-backed actor.
>
> It may additionally wait on FDs, IRQs, child exits, or provider ports,
> but its **control plane** is its inbox.

This applies to every long-lived userspace process that exists to provide a
service: drivers, providers, daemons, supervisors, compositors, the lot. It
does *not* apply to short-lived helper utilities or one-shot shell binaries.

Compliance is enforced via the
[guardrail PR checklist](../concepts/thingos-guardrails.md#pr-review-checklist).

## 2. The four-tier model

Every ThingOS service interacts with the rest of the system through one of
exactly four tiers. The tiers are listed from most-preferred (use first) to
least-preferred (legacy, do not extend):

1. **Control plane → inbox messages.**
   Lifecycle, configuration, RPC-style requests, broadcast notifications.
   Carried by `SYS_MSG_SEND` / `SYS_MSG_RECV` / `SYS_MSG_BROADCAST` and typed
   by [`KindId`](../../thingos/abi/src/kinds.rs).

2. **Readiness aggregation → `vfs_poll` / `WaitSet`.**
   Block on *any* mix of FDs, IRQs, task exits, and (legacy) port handles.
   In userspace, prefer
   [`stem::wait_set::WaitSet`](../../thingos/stem/src/wait_set.rs) wrapped by
   [`stem::service_loop::ServiceLoop`](../../thingos/stem/src/service_loop.rs).

3. **Kernel compatibility → provider ports, pipes, sockets, POSIX glue.**
   Bulk byte streams, the narrow VFS-provider RPC path, POSIX shims. Keep
   these where they are; bridge their FDs into the service's `WaitSet` rather
   than weaving bespoke selects around them.

4. **Bulk data → memfd / rings / DMA.**
   Frame buffers, packet rings, audio buffers, anything where copying through
   message payloads would be wasteful. Out-of-band by design; coordination
   still happens on tier 1.

A fifth bucket — **Legacy: direct `PortHandle` choreography** — is
explicitly *not* a tier. New code must not extend it. Existing call sites
(see Phase 4 below) are tracked for migration.

## 3. The userspace primitive: `ServiceLoop`

`stem::service_loop::ServiceLoop` is the small, opinionated wrapper that
makes the rule cheap to follow. It is:

- A `WaitSet` with the calling task's inbox FD pre-registered as the first
  source.
- A scratch payload buffer sized at construction (`max_payload`).
- A `next_event(timeout)` call that dispatches **inbox-first** when the
  inbox and one or more secondary sources fire on the same wake.

Non-goals (intentional, baked into doc-comments):

- Not an async runtime. No futures, no executors, no tasks.
- Does not own bulk data plumbing (memfd / rings / DMA).
- Does not hide ports — services that still need provider-port RPC keep a
  `PortHandle` and register its FD-bridged readable end as a secondary
  `WaitToken`.

### Resolved design questions

The following choices are baked into the implementation and the BDD spec:

1. **Fairness vs. inbox-first** → **Inbox-first.** When the inbox and a
   secondary source fire on the same wake, the inbox event is returned
   first. This makes "control plane" mean what it says.
2. **Backpressure** → **One message per `next_event`.** Services that need
   to batch can call `drain_inbox()` explicitly. Keeps the loop predictable.
3. **Payload buffer** → **Owned by `ServiceLoop`.** `ServiceEvent::Message`
   borrows it; callers that need to keep payloads copy them out.
4. **Naming** → `ServiceLoop`. `Looper` is *not* an alias.

## 4. Migration plan (fossil sites)

Each migration is its own PR, ordered by risk (lowest first):

1. `drivers/virtio_gpu` raw `thread::spawn(irq_thread)` →
   `ServiceLoop` + `add_irq`.
2. `drivers/driver_wasm_host` inline blocking I/O → `ServiceLoop` (inbox
   becomes its control plane: shutdown, reload, config).
3. Sprout `ManagedTask` (four `PortHandle`s + `resp_fd`) → keep ports as the
   data/RPC plane, lift lifecycle (`start`, `stop`, `restart`, `health`,
   `boot_ready`) onto `KindId`-typed inbox messages.
4. Bristle's four-port-handles-packed-in-one-`usize` → unpack into named
   fields, register each as a token on a `ServiceLoop`. The packing was a
   spawn-ABI artifact; the new loop replaces the select shape that motivated
   it.
5. VFS `ProviderLoop` → keep the provider port (the narrow kernel RPC path)
   but rebuild the loop body around a `ServiceLoop`: provider port is one
   secondary token, control messages arrive on the inbox.
6. Generic driver port-based RPC request/response pairs → same treatment as
   Sprout: control on inbox, data on existing pipes/ports/rings.

Each migration:

- Adds/updates the relevant `.feature` file under
  `docs/behavior/features/`.
- Runs `just behave` and `python3 scripts/audit_platform_boundary.py`.
- Touches no kernel syscalls — this whole thing rides on existing
  primitives.

## 5. What this rule deliberately does *not* do

- Does **not** delete `PortHandle`, pipes, sockets, or provider ports.
- Does **not** introduce an async runtime, executors, or futures.
- Does **not** add new syscalls — `ServiceLoop` is a pure userspace
  composition over `msg_inbox_open_self` + `WaitSet`.
- Does **not** require all migrations to land in one PR — each fossil site
  is independently shippable and revertible.

## 6. Cross-references

- Convergence direction: [`convergence_strategy.md`](./convergence_strategy.md)
- Readiness model: [`../concepts/readiness.md`](../concepts/readiness.md)
- Inbox vs. port semantics: [`./inbox_vs_port_semantics.md`](./inbox_vs_port_semantics.md)
- Guardrails (PR checklist): [`../concepts/thingos-guardrails.md`](../concepts/thingos-guardrails.md)
- Implementation: `thingos/stem/src/service_loop.rs`
- Spec: `docs/behavior/features/service-loop.feature`
