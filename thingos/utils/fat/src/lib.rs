//! FAT Filesystem Parser (read-only)
//!
//! Minimal, read-only FAT16/FAT32 parser for reading files from block devices.
//! Supports short 8.3 filenames and cluster-chain traversal.
#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use stem::block::BlockDevice;

/// Logical sector size assumed for all FAT volumes handled by this library.
/// FAT volumes with other bytes-per-sector values are rejected by [`FatFs::probe`].
pub const FAT_SECTOR_SIZE: u64 = 512;

/// FAT variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FatType {
    Fat16,
    Fat32,
}

/// Parsed BIOS Parameter Block plus derived geometry of a FAT volume.
pub struct FatFs {
    pub fat_type: FatType,
    /// Bytes per logical sector (always 512 for volumes accepted by this library).
    pub bytes_per_sector: u32,
    pub sectors_per_cluster: u32,
    pub reserved_sectors: u32,
    pub num_fats: u32,
    /// FAT region size in logical sectors.
    pub fat_size_sectors: u32,
    /// First logical sector of the FAT-16 fixed root directory (unused for FAT32).
    pub root_dir_first_sector: u64,
    /// Number of 32-byte entries in the FAT-16 root directory (0 for FAT32).
    pub root_dir_entry_count: u32,
    /// First logical sector of the data area (cluster 2).
    pub data_start_sector: u64,
    /// First cluster of the root directory (FAT32 only; 0 for FAT16).
    pub root_first_cluster: u32,
    pub total_sectors: u64,
}

/// A directory entry returned by [`FatFs::list_dir`].
#[derive(Debug, Clone)]
pub struct FatDirEntry {
    pub name: String,
    pub first_cluster: u32,
    pub size: u32,
    pub is_directory: bool,
}

impl FatFs {
    /// Probe a block device for a FAT filesystem.
    ///
    /// Reads the boot sector (LBA 0) and validates the BIOS Parameter Block.
    /// Returns `None` if the volume is not a valid FAT16 or FAT32 filesystem,
    /// or if the logical sector size is not 512 bytes.
    pub fn probe(dev: &dyn BlockDevice) -> Option<Self> {
        let mut boot = [0u8; 512];
        dev.read_sectors(0, 1, &mut boot).ok()?;

        // Standard MBR / VBR signature
        if boot[510] != 0x55 || boot[511] != 0xAA {
            return None;
        }

        // We only support 512-byte sectors
        let bytes_per_sector = u16::from_le_bytes([boot[11], boot[12]]) as u32;
        if bytes_per_sector != 512 {
            return None;
        }

        let sectors_per_cluster = boot[13] as u32;
        // sectors_per_cluster must be a non-zero power of two per the FAT spec.
        if sectors_per_cluster == 0 || sectors_per_cluster.count_ones() != 1 {
            return None;
        }

        let reserved_sectors = u16::from_le_bytes([boot[14], boot[15]]) as u32;
        if reserved_sectors == 0 {
            return None;
        }

        let num_fats = boot[16] as u32;
        if num_fats == 0 || num_fats > 2 {
            return None;
        }

        let root_entry_count = u16::from_le_bytes([boot[17], boot[18]]) as u32;

        let total_sectors_16 = u16::from_le_bytes([boot[19], boot[20]]) as u64;
        let total_sectors_32 =
            u32::from_le_bytes([boot[32], boot[33], boot[34], boot[35]]) as u64;
        let total_sectors =
            if total_sectors_16 != 0 { total_sectors_16 } else { total_sectors_32 };
        if total_sectors == 0 {
            return None;
        }

        let fat_size_16 = u16::from_le_bytes([boot[22], boot[23]]) as u32;

        let (fat_type, fat_size_sectors, root_first_cluster) = if root_entry_count == 0 {
            // FAT32: root_entry_count is 0 and fat_size_16 is 0
            let fat_size_32 =
                u32::from_le_bytes([boot[36], boot[37], boot[38], boot[39]]);
            let root_cluster =
                u32::from_le_bytes([boot[44], boot[45], boot[46], boot[47]]);
            if fat_size_32 == 0 {
                return None;
            }
            (FatType::Fat32, fat_size_32, root_cluster)
        } else {
            if fat_size_16 == 0 {
                return None;
            }
            (FatType::Fat16, fat_size_16, 0u32)
        };

        // Root directory region (FAT16 only)
        let root_dir_sectors =
            (root_entry_count * 32 + bytes_per_sector - 1) / bytes_per_sector;
        let root_dir_first_sector =
            (reserved_sectors + num_fats * fat_size_sectors) as u64;

        // First sector of the data area (cluster 2)
        let data_start_sector = root_dir_first_sector + root_dir_sectors as u64;

        Some(FatFs {
            fat_type,
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            num_fats,
            fat_size_sectors,
            root_dir_first_sector,
            root_dir_entry_count: root_entry_count,
            data_start_sector,
            root_first_cluster,
            total_sectors,
        })
    }

    // ── Cluster chain ──────────────────────────────────────────────────────

    /// Convert a cluster number to its first logical sector.
    pub fn cluster_to_sector(&self, cluster: u32) -> u64 {
        self.data_start_sector + (cluster as u64 - 2) * self.sectors_per_cluster as u64
    }

    /// Read the FAT entry for `cluster` and return the next cluster in the chain.
    pub fn next_cluster(
        &self,
        dev: &dyn BlockDevice,
        cluster: u32,
    ) -> Option<u32> {
        let bytes_per_entry: u64 = match self.fat_type {
            FatType::Fat16 => 2,
            FatType::Fat32 => 4,
        };
        let fat_byte_offset = cluster as u64 * bytes_per_entry;
        let fat_sector =
            self.reserved_sectors as u64 + fat_byte_offset / self.bytes_per_sector as u64;
        let byte_in_sector = (fat_byte_offset % self.bytes_per_sector as u64) as usize;

        let mut buf = [0u8; 512];
        dev.read_sectors(fat_sector, 1, &mut buf).ok()?;

        let next = match self.fat_type {
            FatType::Fat16 => {
                u16::from_le_bytes([buf[byte_in_sector], buf[byte_in_sector + 1]]) as u32
            }
            FatType::Fat32 => {
                u32::from_le_bytes([
                    buf[byte_in_sector],
                    buf[byte_in_sector + 1],
                    buf[byte_in_sector + 2],
                    buf[byte_in_sector + 3],
                ]) & 0x0FFF_FFFF
            }
        };
        Some(next)
    }

    /// Return `true` if `cluster` is an end-of-chain marker.
    pub fn is_end_of_chain(&self, cluster: u32) -> bool {
        match self.fat_type {
            FatType::Fat16 => cluster >= 0xFFF8,
            FatType::Fat32 => cluster >= 0x0FFF_FFF8,
        }
    }

    // ── Directory listing ──────────────────────────────────────────────────

    /// List the directory identified by `first_cluster`.
    ///
    /// Pass `first_cluster = 0` for the FAT16 fixed root directory.
    /// Skips deleted entries, volume labels, and LFN entries.
    pub fn list_dir(&self, dev: &dyn BlockDevice, first_cluster: u32) -> Vec<FatDirEntry> {
        let mut entries = Vec::new();

        if matches!(self.fat_type, FatType::Fat16) && first_cluster == 0 {
            // FAT16 root directory occupies a fixed region
            let num_sectors =
                (self.root_dir_entry_count * 32 + self.bytes_per_sector - 1)
                    / self.bytes_per_sector;
            for i in 0..num_sectors {
                let sector = self.root_dir_first_sector + i as u64;
                let mut buf = [0u8; 512];
                if dev.read_sectors(sector, 1, &mut buf).is_err() {
                    break;
                }
                if !self.parse_dir_sector(&buf, &mut entries) {
                    break; // End-of-directory sentinel hit
                }
            }
        } else {
            // Cluster-chain directory (all FAT32 dirs, FAT16 sub-dirs)
            let mut cluster = first_cluster;
            loop {
                let sector = self.cluster_to_sector(cluster);
                for i in 0..self.sectors_per_cluster {
                    let mut buf = [0u8; 512];
                    if dev.read_sectors(sector + i as u64, 1, &mut buf).is_err() {
                        return entries;
                    }
                    if !self.parse_dir_sector(&buf, &mut entries) {
                        return entries;
                    }
                }
                match self.next_cluster(dev, cluster) {
                    Some(next) if !self.is_end_of_chain(next) => cluster = next,
                    _ => break,
                }
            }
        }
        entries
    }

    /// Parse one 512-byte directory sector into `out`.
    ///
    /// Returns `false` if the all-zero entry sentinel was found (no more entries).
    fn parse_dir_sector(&self, buf: &[u8], out: &mut Vec<FatDirEntry>) -> bool {
        let count = buf.len() / 32;
        for i in 0..count {
            let e = &buf[i * 32..(i + 1) * 32];
            let first_byte = e[0];
            if first_byte == 0x00 {
                return false; // End-of-directory
            }
            if first_byte == 0xE5 {
                continue; // Deleted entry
            }
            let attr = e[11];
            if attr == 0x0F {
                continue; // Long File Name entry
            }
            if attr & 0x08 != 0 {
                continue; // Volume label
            }
            let is_dir = (attr & 0x10) != 0;

            let Some(name) = parse_short_name(&e[0..11]) else {
                continue;
            };
            if name == "." || name == ".." {
                continue;
            }

            let first_cluster_lo = u16::from_le_bytes([e[26], e[27]]) as u32;
            let first_cluster_hi = if matches!(self.fat_type, FatType::Fat32) {
                u16::from_le_bytes([e[20], e[21]]) as u32
            } else {
                0
            };
            let first_cluster = (first_cluster_hi << 16) | first_cluster_lo;
            let size = u32::from_le_bytes([e[28], e[29], e[30], e[31]]);

            out.push(FatDirEntry { name, first_cluster, size, is_directory: is_dir });
        }
        true
    }

    // ── Path resolution ────────────────────────────────────────────────────

    /// Resolve a path (relative to the volume root) to a [`FatDirEntry`].
    ///
    /// Returns `None` if any component along the path is not found.
    ///
    /// Each path component requires a linear scan of the directory.  This is
    /// inherent to the on-disk FAT directory format; for the small volumes used
    /// in this driver (USB storage for embedded/demo purposes) it is acceptable.
    pub fn lookup_path(&self, dev: &dyn BlockDevice, path: &str) -> Option<FatDirEntry> {
        let path = path.trim_matches('/');
        if path.is_empty() {
            return None; // Root itself; callers handle this case
        }

        let root_cluster = match self.fat_type {
            FatType::Fat16 => 0,
            FatType::Fat32 => self.root_first_cluster,
        };
        let mut current_cluster = root_cluster;

        let mut parts = path.split('/').peekable();
        loop {
            let part = parts.next()?;
            let entries = self.list_dir(dev, current_cluster);
            let entry = entries.into_iter().find(|e| e.name.eq_ignore_ascii_case(part))?;

            if parts.peek().is_none() {
                return Some(entry);
            }
            if !entry.is_directory {
                return None; // Not a directory but more components remain
            }
            current_cluster = entry.first_cluster;
        }
    }

    // ── File reading ───────────────────────────────────────────────────────

    /// Read up to `len` bytes from a file starting at `offset`.
    ///
    /// `first_cluster` is the starting cluster of the file and `size` is its
    /// declared byte count from the directory entry.
    pub fn read_file(
        &self,
        dev: &dyn BlockDevice,
        first_cluster: u32,
        size: u32,
        offset: u64,
        len: usize,
    ) -> Vec<u8> {
        if offset >= size as u64 || len == 0 {
            return Vec::new();
        }
        let actual_len = len.min((size as u64 - offset) as usize);
        let cluster_size =
            (self.sectors_per_cluster as u64) * (self.bytes_per_sector as u64);

        let start_ci = (offset / cluster_size) as u32;
        let end_ci = ((offset + actual_len as u64 - 1) / cluster_size) as u32;

        // Collect the cluster numbers we need to read
        let mut clusters: Vec<u32> = Vec::new();
        let mut cluster = first_cluster;
        let mut ci = 0u32;
        loop {
            if ci >= start_ci {
                clusters.push(cluster);
            }
            if ci == end_ci {
                break;
            }
            ci += 1;
            match self.next_cluster(dev, cluster) {
                Some(next) if !self.is_end_of_chain(next) => cluster = next,
                _ => break,
            }
        }

        let mut result = Vec::with_capacity(actual_len);
        for (idx, &c) in clusters.iter().enumerate() {
            let global_ci = (start_ci + idx as u32) as u64;
            let cluster_byte_start = global_ci * cluster_size;
            let cluster_byte_end = cluster_byte_start + cluster_size;

            // Byte range within this cluster that we want
            let want_start =
                (offset.max(cluster_byte_start) - cluster_byte_start) as usize;
            let want_end =
                ((offset + actual_len as u64).min(cluster_byte_end) - cluster_byte_start)
                    as usize;

            let first_sector = self.cluster_to_sector(c);
            for s in 0..self.sectors_per_cluster as u64 {
                let sec_start = (s * self.bytes_per_sector as u64) as usize;
                let sec_end = sec_start + self.bytes_per_sector as usize;

                if sec_end <= want_start || sec_start >= want_end {
                    continue;
                }

                let mut buf = [0u8; 512];
                if dev.read_sectors(first_sector + s, 1, &mut buf).is_err() {
                    return result;
                }

                let from = want_start.saturating_sub(sec_start);
                let to = want_end.min(sec_end) - sec_start;
                result.extend_from_slice(&buf[from..to]);
            }
        }
        result
    }
}

// ── Short name parsing ────────────────────────────────────────────────────────

/// Parse the raw 8.3 directory entry name bytes (11 bytes, no dot) into a
/// lowercase String.  Returns `None` for deleted or empty entries.
fn parse_short_name(raw: &[u8]) -> Option<String> {
    debug_assert_eq!(raw.len(), 11);
    if raw[0] == 0x00 || raw[0] == 0xE5 {
        return None;
    }

    let name: String = raw[0..8]
        .iter()
        .take_while(|&&c| c != b' ')
        .map(|&c| (c as char).to_ascii_lowercase())
        .collect();

    let ext: String = raw[8..11]
        .iter()
        .take_while(|&&c| c != b' ')
        .map(|&c| (c as char).to_ascii_lowercase())
        .collect();

    if name.is_empty() {
        return None;
    }

    let full = if ext.is_empty() {
        name
    } else {
        let mut s = name;
        s.push('.');
        s.push_str(&ext);
        s
    };
    Some(full)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;
    use stem::block::BlockError;

    struct MockDev {
        data: Vec<u8>,
    }

    impl BlockDevice for MockDev {
        fn sector_size(&self) -> u64 {
            512
        }
        fn read_sectors(&self, lba: u64, count: u64, buf: &mut [u8]) -> Result<(), BlockError> {
            let off = (lba * 512) as usize;
            let len = (count * 512) as usize;
            if off + len > self.data.len() {
                return Err(BlockError::OutOfRange);
            }
            buf[..len].copy_from_slice(&self.data[off..off + len]);
            Ok(())
        }
    }

    #[test]
    fn probe_rejects_invalid_signature() {
        let data = vec![0u8; 1024];
        let dev = MockDev { data };
        assert!(FatFs::probe(&dev).is_none());
    }

    #[test]
    fn parse_short_name_basic() {
        // "README  TXT" → "readme.txt"
        let raw = *b"README  TXT";
        assert_eq!(parse_short_name(&raw), Some("readme.txt".to_string()));
    }

    #[test]
    fn parse_short_name_no_ext() {
        let raw = *b"BOOT       ";
        assert_eq!(parse_short_name(&raw), Some("boot".to_string()));
    }

    #[test]
    fn parse_short_name_deleted() {
        let mut raw = *b"README  TXT";
        raw[0] = 0xE5;
        assert!(parse_short_name(&raw).is_none());
    }

    #[test]
    fn end_of_chain_fat16() {
        let fs = FatFs {
            fat_type: FatType::Fat16,
            bytes_per_sector: 512,
            sectors_per_cluster: 1,
            reserved_sectors: 1,
            num_fats: 1,
            fat_size_sectors: 1,
            root_dir_first_sector: 2,
            root_dir_entry_count: 16,
            data_start_sector: 3,
            root_first_cluster: 0,
            total_sectors: 64,
        };
        assert!(fs.is_end_of_chain(0xFFFF));
        assert!(fs.is_end_of_chain(0xFFF8));
        assert!(!fs.is_end_of_chain(0x0003));
    }

    /// Build a self-contained 64-sector FAT16 volume (no MBR) and verify that
    /// `FatFs::probe`, `list_dir`, and `read_file` all work correctly.
    #[test]
    fn roundtrip_fat16_volume() {
        // Geometry
        const SECTOR_SIZE: usize = 512;
        const SECTORS_PER_CLUSTER: u32 = 1;
        const RESERVED: u32 = 1; // just the boot sector
        const NUM_FATS: u32 = 2;
        const ROOT_ENTRIES: u32 = 16; // 1 sector of root dir
        const FAT_SIZE: u32 = 1; // 1 sector of FAT (enough for small volume)
        const TOTAL_SECTORS: u32 = 32;

        let root_dir_sectors = ROOT_ENTRIES * 32 / SECTOR_SIZE as u32;
        let data_start = (RESERVED + NUM_FATS * FAT_SIZE + root_dir_sectors) as usize;

        let mut img = vec![0u8; TOTAL_SECTORS as usize * SECTOR_SIZE];

        // Boot sector
        {
            let s = &mut img[0..SECTOR_SIZE];
            s[0] = 0xEB; s[1] = 0x58; s[2] = 0x90;
            s[3..11].copy_from_slice(b"MSWIN4.1");
            s[11..13].copy_from_slice(&(SECTOR_SIZE as u16).to_le_bytes());
            s[13] = SECTORS_PER_CLUSTER as u8;
            s[14..16].copy_from_slice(&(RESERVED as u16).to_le_bytes());
            s[16] = NUM_FATS as u8;
            s[17..19].copy_from_slice(&(ROOT_ENTRIES as u16).to_le_bytes());
            s[19..21].copy_from_slice(&(TOTAL_SECTORS as u16).to_le_bytes());
            s[21] = 0xF8; // media byte
            s[22..24].copy_from_slice(&(FAT_SIZE as u16).to_le_bytes());
            s[24..26].copy_from_slice(&63u16.to_le_bytes());
            s[26..28].copy_from_slice(&255u16.to_le_bytes());
            s[38] = 0x29;
            s[39..43].copy_from_slice(&0x1234_5678u32.to_le_bytes());
            s[43..54].copy_from_slice(b"TEST       ");
            s[54..62].copy_from_slice(b"FAT16   ");
            s[510] = 0x55;
            s[511] = 0xAA;
        }

        // FAT1: entries 0+1 = media/EOC, entry 2 = EOC (hello.txt)
        let fat1 = RESERVED as usize * SECTOR_SIZE;
        img[fat1] = 0xF8; img[fat1 + 1] = 0xFF; // entry 0
        img[fat1 + 2] = 0xFF; img[fat1 + 3] = 0xFF; // entry 1
        img[fat1 + 4] = 0xFF; img[fat1 + 5] = 0xFF; // entry 2 = EOC

        // FAT2 mirror
        let fat2 = (RESERVED + FAT_SIZE) as usize * SECTOR_SIZE;
        img.copy_within(fat1..fat1 + SECTOR_SIZE, fat2);

        // Root directory
        let root = (RESERVED + NUM_FATS * FAT_SIZE) as usize * SECTOR_SIZE;
        img[root..root + 8].copy_from_slice(b"HELLO   ");
        img[root + 8..root + 11].copy_from_slice(b"TXT");
        img[root + 11] = 0x20; // archive
        img[root + 26] = 2; // first cluster
        let content = b"hello test\n";
        img[root + 28..root + 32].copy_from_slice(&(content.len() as u32).to_le_bytes());

        // File data at cluster 2
        let data_off = data_start * SECTOR_SIZE;
        img[data_off..data_off + content.len()].copy_from_slice(content);

        let dev = MockDev { data: img };

        let fs = FatFs::probe(&dev).expect("probe should succeed");
        assert_eq!(fs.fat_type, FatType::Fat16);
        assert_eq!(fs.bytes_per_sector, 512);
        assert_eq!(fs.sectors_per_cluster, 1);

        // List root directory
        let entries = fs.list_dir(&dev, 0);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "hello.txt");
        assert!(!entries[0].is_directory);
        assert_eq!(entries[0].first_cluster, 2);
        assert_eq!(entries[0].size, content.len() as u32);

        // Lookup by path
        let entry = fs.lookup_path(&dev, "hello.txt").expect("lookup_path should succeed");
        assert_eq!(entry.name, "hello.txt");

        // Read the file
        let data = fs.read_file(&dev, 2, content.len() as u32, 0, content.len());
        assert_eq!(data, content);

        // Partial read
        let partial = fs.read_file(&dev, 2, content.len() as u32, 6, 4);
        assert_eq!(partial, b"test");
    }
}
