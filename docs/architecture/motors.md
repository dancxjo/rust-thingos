# Motors: canonical loadable binary model

## What is a Motor?

A **Motor** is a loadable ELF artifact plus canonical metadata exported at
`THINGOS_MOTOR`.

- **Motor artifact**: the compiled ELF image and embedded descriptor.
- **Motor instance**: a running/bound realization of that artifact.

The descriptor is compiler/runtime-facing metadata for discovery and entry. It
does not replace ELF itself.

## Descriptor contract (ABI v1)

The descriptor is `abi::motor::MotorDescriptor` and includes:

- motor ABI version
- motor name
- hosting modes (`PROGRAM`, `LIFECYCLE`, `DRIVER`)
- declared interfaces (id + version + optional entry symbol)

Current interface ids:

- `ProgramV1` (`MOTOR_INTERFACE_PROGRAM_V1`)
- `LifecycleV1` (`MOTOR_INTERFACE_LIFECYCLE_V1`)
- `DriverV1` (`MOTOR_INTERFACE_DRIVER_V1`)

For `ProgramV1`, an empty `entry_symbol` means **use the ELF default entry**,
so regular plain-`main` style binaries remain valid.

## Runtime discovery

- Driver orchestration (`devd`) prefers `THINGOS_MOTOR` + `DriverV1`.
- Legacy driver symbols (`THING_DRIVER_V1`, `THINGOS_DRIVER`) remain supported
  as an explicit transitional path.
- Program launching continues through normal executable flow; `ProgramV1`
  metadata is now available for Motor-native discovery.

## Migration notes

- One ordinary program example now exports a Motor descriptor:
  `userspace/hello_stdio`.
- One real driver example now exports a Motor descriptor:
  `drivers/hwrng`.
- Existing driver descriptors are still present while the transition completes.
