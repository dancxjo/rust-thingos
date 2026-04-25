# Unified Content Provider System

## Overview

The Content Provider system provides a unified abstraction for accessing files and resources from multiple sources (boot modules, ISO filesystems, and future sources like network or persistent storage). All content is materialized into the graph as first-class Things with stable identities, provenance tracking, and consistent access patterns.

## Architecture

### Core Principles

1. **Files are Things**: Every file, regardless of source, is a graph node with stable identity
2. **Sources are Things**: Content sources (Limine, ISO, etc.) are discoverable graph nodes
3. **Loading is continuous**: Content can be added, updated, or removed dynamically
4. **Unified access**: Consumers don't care about the source origin

### Graph Model

#### ContentSource Nodes

**Kind**: `content.Source`

Represents a source of content (Limine boot modules, ISO9660 disk, future: network, filesystem).

**Properties**:
- `content.source.kind` (symbol): Type of source
  - `"limine_module"`: Boot modules provided by Limine bootloader
  - `"iso9660_disk"`: ISO9660 filesystem on disk
- `content.source.name` (symbol): Human-readable name (e.g., "boot", "cdrom0")
- `content.source.priority` (u64): Priority for overlay resolution (higher wins)
  - Limine: 100 (default)
  - ISO: 50
- `content.source.state` (symbol): Current state
  - `"ready"`: Source is available and serving content
  - `"error"`: Source encountered an error
  - `"initializing"`: Source is being initialized
- `content.source.gen` (u64): Generation counter (increments on refresh/remount)

#### File Nodes

**Kind**: `content.File`

Represents a file from any content source.

**Properties**:
- `file.name` (symbol): File name (leaf name, not full path)
- `file.size` (u64): File size in bytes
- `file.hash` (u64): Content hash (SHA-256 first 8 bytes, little-endian)
- `file.mime` (symbol, optional): MIME type (e.g., "image/svg+xml", "application/font-sfnt")
- `file.source` (ThingId): Reference to ContentSource node

#### Directory Nodes

**Kind**: `content.Directory`

Represents a directory in the content graph (future enhancement).

**Properties**:
- `dir.name` (symbol): Directory name (leaf name)
- `dir.path` (symbol, optional): Full path for quick lookups

**Relationships**:
- `content.contains`: ContentSource/Directory → File/Directory
- `content.located_at`: File/Directory → Directory
- `content.provided_by`: File/Directory → ContentSource

## Implementation

### Limine Module Source

**Service**: `ingestd`

1. Creates a `content.Source` node with kind `"limine_module"` and priority 100
2. Watches for `boot.Module` nodes from the kernel
3. For each module, creates:
   - An `Asset` node (backward compatibility with existing consumers)
   - A `content.File` node with computed content hash and MIME type
4. Content is reachable via VFS path (e.g., `/boot/modules/{name}`)

### ISO9660 Source

**Service**: `iso_reader`

1. Probes for ATAPI CD-ROM devices
2. Scans ISO9660 filesystem
3. Creates a `content.Source` node with kind `"iso9660_disk"` and priority 50
4. For each file:
   - Creates a `boot.Module` node (backward compatibility)
   - Creates a `content.File` node with computed content hash and MIME type
5. Content is reachable via VFS path (e.g., `/dev/cdrom0/{path}`)

### Content Discovery

Consumers can discover content in two ways:

1. **By source**: Query `content.Source` nodes, then traverse `content.contains` relationships
2. **By type**: Query `content.File` nodes directly, filter by `file.source` if needed

### Overlay Resolution

When multiple sources provide the same file path:

1. Files from higher-priority sources take precedence
2. Both files exist in the graph with different `file.source` values
3. Consumers can choose to respect priority or access all versions

**Example**: If both Limine and ISO provide `/assets/cursor.svg`, the Limine version (priority 100) is preferred over ISO (priority 50).

## Usage Examples

### Finding All Available Sources

```rust
let mut sources = [ThingId::default(); 16];
if let Ok(count) = find(kinds::CONTENT_SOURCE, &mut sources) {
    for &source_id in &sources[..count] {
        let name = prop_get(source_id, keys::CONTENT_SOURCE_NAME);
        let state = prop_get(source_id, keys::CONTENT_SOURCE_STATE);
        // Use source...
    }
}
```

### Finding a File by Name

```rust
let mut files = [ThingId::default(); 512];
if let Ok(count) = find(kinds::CONTENT_FILE, &mut files) {
    for &file_id in &files[..count] {
        let name_sym = prop_get(file_id, keys::FILE_NAME).unwrap_or(0);
        let mut buf = [0u8; 256];
        if let Ok(len) = describe_symbol(name_sym as u32, &mut buf) {
            let name = core::str::from_utf8(&buf[..len]).unwrap_or("");
            if name.ends_with("cursor.svg") {
                // Found it! Use VFS to read the content.
                break;
            }
        }
    }
}
```

### Reading File Content

Files are accessed via standard VFS syscalls (`open`, `read`, `mmap`).

```rust
// Open and mmap file content
let fd = stem::syscall::open("/path/to/file", abi::numbers::OpenFlags::READ)?;
let size = stem::syscall::fstat(fd)?.size;
let ptr = stem::syscall::vm_map(&abi::syscall::VmMapRequest {
    backing: abi::syscall::VmBacking::File { fd, offset: 0 },
    addr: 0,
    size,
    flags: abi::syscall::VmFlags::READ,
})?;

let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, size) };

// Use content...
```

## Backward Compatibility

The system maintains full backward compatibility:

1. **Asset System**: `ingestd` still creates `Asset` nodes for all content
2. **Boot Modules**: `iso_reader` still creates `boot.Module` nodes
3. **Existing Consumers**: Services like `fontd`, `blossom`, etc. continue to work unchanged

The new `content.File` nodes augment (not replace) the existing system, providing a unified view while preserving all existing functionality.

## Future Enhancements

1. **Lazy Loading**: Create File nodes without immediately reading content
2. **Block Cache**: LRU cache for ISO sector reads
3. **Directory Trees**: Full hierarchical directory structure
4. **Network Sources**: HTTP/FTP content sources
5. **Persistent Storage**: Writable filesystem sources
6. **Hot Reload**: Dynamic remounting and cache invalidation
7. **Path Resolution**: Virtual filesystem with overlay semantics

## Testing

Run schema tests:
```bash
cargo test -p abi --test content_provider_schema
```

Build integration test:
```bash
cargo xtask iso --env x86_64
```

The ISO will include both Limine modules and the iso_reader service, demonstrating dual-source content access.
