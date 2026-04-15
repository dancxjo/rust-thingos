# Semantic Self-Hosting Status Audit (April 2026)

This is a point-in-time audit of milestone progress for **partial/full semantic self-hosting**.

## Executive Status

| Milestone area | Status | Where it stands today |
|---|---|---|
| In-system validation of canonical Kind registry | **Partial (host-validated, not in-guest)** | `kindc` drift checks and kernel boundary-contract tests validate canonical Kind IDs and generated schema alignment during host builds/CI. |
| Generation of live bindings/docs/metadata from inside ThingOS | **Not yet** | Binding generation (`kindc`) is currently a host tool path (`cargo run -p kindc`, `just kindc-gen`), not a running-ThingOS workflow. |
| Self-hosted userland/toolchain/component build/test flows | **Partial** | `xtask` produces and caches a Linux-hosted cross-compiler and uses it in host-side build flows. A ThingOS-native rustc path exists as optional recovery (`BUILD_THINGOS_NATIVE_RUSTC=1`) but is not the default validated path. |
| Documentation on self-hosting paths and limitations | **Partial** | Rust bootstrap status is documented (`docs/build/bootstrapping.md`, `docs/build/status/rustc_build.md`), but this milestone-wide semantic self-hosting status needed a unified audit. |

## Evidence Snapshot

- Canonical Kind schema pipeline and drift guardrails are in place:
  - `tools/kindc/README.md`
  - `docs/build/kind-schema-workflow.md`
  - `.github/workflows/kindc-drift.yml`
  - `kernel/src/boundary_contract.rs`
- Toolchain/bootstrap is still primarily host-driven:
  - `xtask/src/rustc_thingos.rs` builds a Linux-hosted stage-1 cross-compiler cache.
  - `xtask/src/image.rs` stages many userspace binaries, but no canonical in-guest `kindc`/schema regeneration service is part of default ISO bring-up.
  - `docs/build/bootstrapping.md` and `docs/build/status/rustc_build.md` document that ThingOS-native toolchain hosting remains incomplete.

## Gap Analysis Against Issue Success Criteria

To satisfy the milestone success criteria end-to-end, the project still needs:

1. **In-guest schema revalidation path**  
   A supported ThingOS runtime flow that can re-run canonical Kind validation in the running system, not only in host CI.

2. **In-guest artifact regeneration path**  
   Live regeneration/update of bindings/docs/metadata from inside ThingOS (or an equivalent in-OS service).

3. **Default, validated ThingOS-native rebuild loop**  
   Userland/component rebuild and test flows that run inside ThingOS as a first-class path, without relying on host-side orchestration as the source of truth.

4. **Schema-governed runtime wiring proof**  
   A demonstrable runtime contract showing kernel/std/userland definitions are governed by the canonical live schema in-system, not only by checked-in generated artifacts.

## Practical Near-Term Definition of “Partial Self-Hosting”

Given current implementation, “partial self-hosting” is accurately described as:

- canonical schema + drift checks enforced in repo/CI,
- host-driven regeneration and rebuild,
- early optional path toward ThingOS-native compiler artifacts,
- but no complete in-guest semantic regeneration/revalidation pipeline yet.
