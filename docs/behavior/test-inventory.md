# Behavior Test Inventory

This inventory records the current split between user-facing Gherkin coverage
and lower-level unit coverage after the BDD cleanup.

## Gherkin Coverage Shape

- Desktop and compositor behavior is covered by user stories in
  `bloom-compositor.feature`: first paint from VFS/session state, live desktop
  settings, boot framebuffer fallback, virtio GPU presentation, hardware cursor
  responsiveness, pointer debug mode, Wayland protocol surface, client data
  flows, and pistil renderer loading.
- Wayland protocol state remains in `blossom-xdg-shell.feature`, where the
  scenarios express client-visible xdg-shell lifecycle rules. The matching
  `blossom` unit tests cover the same state machine directly.
- USB storage is split by user intent: `xhci-driver.feature` proves userland can
  discover, configure, read, and publish USB mass storage; `usb-fat-storage.feature`
  proves the resulting FAT media can be mounted, listed, and read.
- Shell utilities are expressed as pipeline workflows in `bin-utils.feature`
  rather than one scenario per utility option.
- Sprout graphics boot is expressed as desktop session orchestration in
  `sprout-graphics-pipeline.feature`, grouping compositor, clients, Leaf, and
  RTC-backed clock readiness into two stories.
- ServiceLoop, shutdown, scheduler, networking, entropy, PCI, and boot progress
  features remain mostly aligned with system contracts and were not rewritten in
  this pass.

## Implemented-System Mapping

- Bloom scenarios map to `thingos/bloom/src` service, scene, input, damage, and
  Wayland command paths, plus display providers in `thingos/drivers/display_*`.
- USB mass-storage scenarios map to `thingos/drivers/usb/xhci/src/main.rs`,
  `thingos/utils/fatd/src/main.rs`, and the FAT parser in `thingos/utils/fat`.
- Sprout desktop orchestration maps to `thingos/sprout` and the launched
  userland clients under `thingos/utils`.
- Shell pipeline behavior maps to `thingos/utils/sh` plus the small text tools
  under `thingos/utils`.

## Unit Test Complement

- `thingos/blossom/src/lib.rs` unit tests are the right home for xdg-shell state
  machine edge cases such as duplicate roles, configure serial validation,
  popup placement, and cleanup. BDD should keep only client-visible lifecycle
  stories.
- `thingos/bloom/src/damage.rs` and `thingos/bloom/src/accel2d_batch.rs` unit
  tests cover compositor internals that would be brittle as boot-level BDD.
  BDD now asserts the user-observable cursor, damage, and presentation effects.
- `thingos/utils/fat/src/lib.rs` now complements the USB FAT BDD with parser
  coverage for invalid BPB data, FAT16 file round trips, and FAT32 root file
  round trips. BDD remains focused on mounted media behavior.
- Driver binaries such as `fatd` and `xhci` currently do not run under host
  `cargo test` because they depend on `stem` runtime lang items. Keep pure
  parsing and protocol logic in testable libraries where practical.
