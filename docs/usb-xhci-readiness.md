# USB xHCI / USB Storage Readiness

Status date: 2026-05-01

## Current reality

The xHCI driver contains code for root-port scanning, device enumeration,
USB Mass Storage Bulk-Only Transport, and a raw `/dev/block/usb0` provider.
That code builds, but the QEMU acceptance path is not proven yet.

A QEMU smoke run with a `qemu-xhci` controller and `usb-storage` device showed
the xHCI driver being catalogued, but the driver did not start and did not log
descriptor enumeration. The first runtime blocker is therefore xHCI PCI device
binding or driver spawning, before debugging USB enumeration itself.

## Issue 4: Enumerate USB devices on xHCI root ports

Summary: implement root-port detection, port reset, slot enablement, endpoint
zero setup, and basic USB descriptor enumeration for devices attached to xHCI
root ports.

| Item | Status | Notes |
| --- | --- | --- |
| PORTSC scan | Implemented, not runtime-proven | `scan_and_enumerate` walks root ports and reads PORTSC. |
| Connect status change handling | Partial | Port Status Change events are handled while waiting for events, but there is no complete hotplug state machine. |
| Port reset | Implemented, not runtime-proven | Connected ports are reset and polled for completion. |
| Port speed detection | Implemented, not runtime-proven | Speed is decoded from PORTSC and logged. |
| Enable Slot command | Implemented, not runtime-proven | Command ring path exists. |
| Address Device command | Implemented, not runtime-proven | Input context setup and Address Device command exist. |
| Endpoint zero transfer ring | Implemented, not runtime-proven | EP0 ring is allocated and installed into the input context. |
| Control transfers | Partial, not runtime-proven | GET_DESCRIPTOR(Device), GET_DESCRIPTOR(Configuration), and SET_CONFIGURATION exist. |
| Device descriptor logging | Implemented, not observed in QEMU | VID, PID, USB version, class, subclass, protocol, and config count log sites exist. |
| Configuration/interface/endpoint logging | Implemented, not observed in QEMU | Interface and endpoint descriptor parsing/logging exists. |
| QEMU acceptance | Not ready | Current smoke did not reach `xhci: starting userspace xHCI driver`. |

Readiness verdict: **not ready for acceptance**. The code path exists, but the
runtime blocker is earlier: the userspace xHCI driver is not being started for
the QEMU xHCI device.

## Issue 5: Implement read-only USB Mass Storage Bulk-Only Transport

Summary: implement a read-only USB Mass Storage driver for devices using the
SCSI transparent command set over Bulk-Only Transport.

| Item | Status | Notes |
| --- | --- | --- |
| Match class `08`, subclass `06`, protocol `50` | Implemented, not runtime-proven | Configuration parsing identifies mass-storage interfaces. |
| Bulk IN endpoint discovery | Implemented, not runtime-proven | Bulk endpoint parsing exists. |
| Bulk OUT endpoint discovery | Implemented, not runtime-proven | Bulk endpoint parsing exists. |
| Configure bulk endpoints | Implemented, not runtime-proven | Endpoint contexts and rings are prepared after enumeration. |
| CBW structure | Implemented, not runtime-proven | BOT command wrapper path exists. |
| CSW structure | Implemented, not runtime-proven | BOT status wrapper validation path exists. |
| SCSI INQUIRY | Implemented, not runtime-proven | Vendor/product log site exists. |
| SCSI TEST UNIT READY | Implemented, not runtime-proven | Command path exists. |
| SCSI REQUEST SENSE | Not implemented | Required by scope, still missing. |
| SCSI READ CAPACITY(10) | Implemented, not runtime-proven | Capacity and sector-size log site exists. |
| SCSI READ(10) | Implemented, not runtime-proven | LBA read path and first-sector dump log site exist. |
| QEMU acceptance | Not ready | No `ums:` logs were observed because xHCI did not start. |

Readiness verdict: **not ready for acceptance**. Most of the vertical slice is
present in code, but `REQUEST SENSE` is still missing and no BOT command has
been proven against QEMU.

## Issue 6: Expose USB Mass Storage as a ThingOS block device

Summary: wire read-only USB Mass Storage into the existing block-device and VFS
stack so USB drives appear as mountable block devices.

| Item | Status | Notes |
| --- | --- | --- |
| Expose discovered USB mass-storage LUNs | Partial, not runtime-proven | A single raw provider is created for the first discovered device. |
| `/dev/block/usb0` | Implemented, not runtime-proven | Provider path exists. |
| `/dev/disk/usb0` | Not implemented | Alias path is not created. |
| `/dev/disk/by-bus/usb0` | Not implemented | Stable bus alias is not created. |
| Reuse existing block-device abstractions | Partial | Uses the VFS provider loop shape, but does not yet plug into all AHCI-style disk discovery paths. |
| Partition scanner reads LBA 0 | Not ready | Raw LBA reads are implemented, but partition scanning has not been wired or proven. |
| FAT partition mounts read-only | Not ready | No successful `/dev/block/usb0` runtime path has been proven. |
| `ls` and `cat` on mounted USB files | Not ready | Depends on block exposure, partition discovery, and filesystem mount. |

Readiness verdict: **not ready for acceptance**. The first raw block-device
provider exists in code, but the disk namespace, partition scan, and mount flow
are not ready.

## Golden path status

| Step | Status |
| --- | --- |
| Bootable USB image | Not addressed by these changes |
| xHCI detected | Not ready in QEMU smoke; driver catalogued but not started |
| xHCI initialized | Implemented in code, not proven in current smoke |
| USB stick enumerated | Implemented in code, not runtime-proven |
| Mass storage READ(10) | Implemented in code, not runtime-proven |
| `/dev/block/usb0` | Implemented in code, not runtime-proven |
| Mount `/mnt/usb` | Not ready |

Golden path verdict: **not ready**.

## Ready now

- `lsusb` utility coverage exists and passes the behavior scenario.
- xHCI register and DMA bring-up code exists.
- Root-port enumeration code exists.
- USB descriptor parsing code exists.
- USB Mass Storage BOT read-only code mostly exists.
- A first `/dev/block/usb0` raw provider exists.

## Not ready yet

- QEMU xHCI driver binding/spawning.
- Runtime proof of root-port reset and descriptor enumeration.
- Runtime proof of Enable Slot, Address Device, and EP0 control transfers.
- `REQUEST SENSE`.
- Runtime proof of Mass Storage BOT commands.
- `/dev/disk/*` aliases for USB storage.
- Partition scanner integration for USB storage.
- FAT mount proof from USB storage.
- End-to-end golden path from QEMU USB stick to mounted files.

## Recommended next steps

1. Fix xHCI binding under QEMU.
   Confirm that the QEMU xHCI PCI function appears in ThingOS device discovery,
   has class code `0x0c0330`, and matches the xHCI driver descriptor.

2. Once the driver starts, prove the command/event path.
   The first useful runtime checkpoint is Enable Slot completing successfully.

3. Prove EP0 enumeration.
   Target logs for VID, PID, USB version, configuration count, interface
   descriptors, and endpoint descriptors.

4. Add `REQUEST SENSE`.
   This closes the remaining required SCSI command in the read-only BOT scope.

5. Prove BOT with QEMU.
   Target `INQUIRY`, `READ CAPACITY(10)`, and `READ(10)` LBA 0.

6. Wire the raw USB block provider into the existing disk path.
   Start with `/dev/block/usb0`, then add `/dev/disk/usb0` and
   `/dev/disk/by-bus/usb0`.

7. Connect partition and filesystem mounting.
   The next meaningful acceptance target is reading LBA 0 through the partition
   scanner, then mounting a FAT partition read-only.
