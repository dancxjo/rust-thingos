# Feature: ISO9660 boot filesystem mount

> Last run: 2026-05-01 15:28:03

## Scenarios

| Scenario | Steps | Status | Link |
|----------|-------|--------|------|
| iso9660d mounts the boot filesystem at /media/cdrom | 0/1 | ❌ | [View Details](iso9660d-mounts-the-boot-filesystem-at-media-cdrom/README.md) |
| iso9660d serves multiple sequential reads correctly via parallel dispatch | 0/1 | ❌ | [View Details](iso9660d-serves-multiple-sequential-reads-correctly-via-parallel-dispatch/README.md) |
| iso9660d handles concurrent reads and readdir without response mis-correlation | 0/1 | ❌ | [View Details](iso9660d-handles-concurrent-reads-and-readdir-without-response-mis-correlation/README.md) |
| boot ISO files are accessible at /media/cdrom via QEMU -cdrom | 0/1 | ❌ | [View Details](boot-iso-files-are-accessible-at-media-cdrom-via-qemu-cdrom/README.md) |
| ELF binaries are loaded correctly via the memfd bulk-transfer path | 0/1 | ❌ | [View Details](elf-binaries-are-loaded-correctly-via-the-memfd-bulk-transfer-path/README.md) |
| Repeated process spawns use the kernel page cache (no redundant IPC reads) | 0/1 | ❌ | [View Details](repeated-process-spawns-use-the-kernel-page-cache-no-redundant-ipc-reads/README.md) |
