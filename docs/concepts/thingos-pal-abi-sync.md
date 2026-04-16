# ThingOS `std` PAL ABI sync checklist

When changing ThingOS syscall numbers or C ABI payload layout, keep `std` PAL in lockstep.

## Source of truth

- Syscall IDs and flag constants: `abi/src/numbers.rs`
- ABI payload structs: `abi/src/types/system.rs` and `abi/src/time.rs`
- Shared PAL mirrors: `library/std/src/sys/pal/thingos/abi.rs`

## Required checks

1. Update ABI source files first (`abi/src/...`).
2. Update `thingos` PAL shared mirrors in `library/std/src/sys/pal/thingos/abi.rs` if layout changed.
3. Keep syscall usage imported from `library/std/src/sys/thingos_syscall_numbers.rs` (no local `SYS_*` constants).
4. Run:

```bash
python3 scripts/audit_thingos_pal_abi.py
```

This audit is intentionally small and only enforces the highest-risk drift points.
