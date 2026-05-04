//! ACPI table discovery via HHDM.
//!
//! Reads raw ACPI table bytes from physical memory using the HHDM (Higher-Half
//! Direct Map) established during early boot.  Used by sysfs to expose table
//! blobs under `/sys/firmware/acpi_tables/<SIG>`.
//!
//! ## Safety
//! All reads go through the HHDM, which Limine establishes before transferring
//! control.  By the time any process queries sysfs the HHDM and ACPI pages are
//! guaranteed to be accessible.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use spin::Mutex;

const RSDP_SIG: &[u8; 8] = b"RSD PTR ";
const SDT_HEADER_LEN: usize = 36;
/// Safety cap: reject any single table larger than 4 MiB.
const MAX_TABLE_LEN: usize = 4 * 1024 * 1024;

// Byte offsets inside the RSDP structure.
const RSDP_OFF_REVISION: usize = 15;
const RSDP_OFF_RSDT_ADDR: usize = 16;
const RSDP_OFF_LENGTH: usize = 20;
const RSDP_OFF_XSDT_ADDR: usize = 24;

static ACPI_ENUM_LOCK: Mutex<()> = Mutex::new(());

/// A discovered ACPI system table.
#[derive(Clone)]
pub struct AcpiEntry {
    /// 4-character ASCII signature (e.g. `APIC`, `FACP`, `DSDT`).
    pub signature: [u8; 4],
    /// Original physical address (for diagnostics).
    pub phys_addr: u64,
    /// Raw table bytes including the 36-byte SDT header.
    pub data: Vec<u8>,
}

impl AcpiEntry {
    /// Sysfs-visible name for this entry, disambiguating duplicates with a
    /// numeric suffix.  Index 0 → bare signature, index 1+ → `<SIG><idx>`.
    pub fn sysfs_name(&self, idx: usize) -> String {
        let sig = core::str::from_utf8(&self.signature).unwrap_or("????");
        let sig = sig.trim_end_matches('\0').trim_end();
        if idx == 0 { sig.to_string() } else { alloc::format!("{}{}", sig, idx) }
    }

    /// OEM table ID from the standard SDT header (bytes 12–19).
    pub fn oem_table_id(&self) -> &[u8] {
        if self.data.len() >= 20 { &self.data[12..20] } else { b"" }
    }
}

/// Enumerate all ACPI tables reachable from the RSDP at `rsdp_phys`.
///
/// Returns an empty `Vec` if ACPI is absent, the RSDP is invalid, or any
/// fatal checksum error is detected.
pub fn enumerate(rsdp_phys: u64, hhdm: u64) -> Vec<AcpiEntry> {
    let _guard = ACPI_ENUM_LOCK.lock();
    // SAFETY: physical table reads are copied through temporary kernel mappings.
    unsafe { do_enumerate(rsdp_phys, hhdm) }
}

unsafe fn do_enumerate(rsdp_phys: u64, hhdm: u64) -> Vec<AcpiEntry> {
    let Some(rsdp) = (unsafe { read_phys_bytes(rsdp_phys, 36, hhdm) }) else {
        crate::kwarn!("ACPI: RSDP at phys 0x{:x} is not readable", rsdp_phys);
        return Vec::new();
    };

    // Verify signature.
    if &rsdp[..8] != RSDP_SIG {
        crate::kwarn!("ACPI: invalid RSDP signature at phys 0x{:x}", rsdp_phys);
        return Vec::new();
    }

    // Verify ACPI 1.0 checksum (first 20 bytes).
    if !checksum_bytes_ok(&rsdp[..20]) {
        crate::kwarn!("ACPI: RSDP v1 checksum failed");
        return Vec::new();
    }

    let revision = rsdp[RSDP_OFF_REVISION];
    crate::kdebug!("ACPI: RSDP revision {}", revision);

    if revision >= 2 {
        // ACPI 2.0+: read extended checksum if length is sensible.
        let ext_len =
            u32::from_le_bytes(rsdp[RSDP_OFF_LENGTH..RSDP_OFF_LENGTH + 4].try_into().unwrap())
                as usize;
        if (36..=64).contains(&ext_len) {
            if let Some(ext_rsdp) = unsafe { read_phys_bytes(rsdp_phys, ext_len, hhdm) } {
                if !checksum_bytes_ok(&ext_rsdp) {
                    crate::kwarn!("ACPI: RSDP extended checksum failed (continuing)");
                }
            }
        }
        let xsdt_phys = u64::from_le_bytes(
            rsdp[RSDP_OFF_XSDT_ADDR..RSDP_OFF_XSDT_ADDR + 8].try_into().unwrap(),
        );
        crate::kdebug!("ACPI: using XSDT at phys 0x{:x}", xsdt_phys);
        unsafe { walk_xsdt(xsdt_phys, hhdm) }
    } else {
        let rsdt_phys = u32::from_le_bytes(
            rsdp[RSDP_OFF_RSDT_ADDR..RSDP_OFF_RSDT_ADDR + 4].try_into().unwrap(),
        ) as u64;
        crate::kdebug!("ACPI: using RSDT at phys 0x{:x}", rsdt_phys);
        unsafe { walk_rsdt(rsdt_phys, hhdm) }
    }
}

unsafe fn walk_rsdt(phys: u64, hhdm: u64) -> Vec<AcpiEntry> {
    let Some(hdr) = (unsafe { read_sdt_header(phys, hhdm) }) else { return Vec::new() };
    let total = u32::from_le_bytes(hdr[4..8].try_into().unwrap_or([0; 4])) as usize;
    if total < SDT_HEADER_LEN {
        return Vec::new();
    }
    let entry_count = (total - SDT_HEADER_LEN) / 4;
    let mut out = Vec::new();
    for i in 0..entry_count {
        let entry_phys = phys + SDT_HEADER_LEN as u64 + (i as u64) * 4;
        let Some(addr) = (unsafe { read_phys_u32(entry_phys, hhdm) }) else { continue };
        let addr = addr as u64;
        if addr == 0 {
            continue;
        }
        if let Some(entry) = unsafe { read_full_table(addr, hhdm) } {
            crate::kdebug!(
                "ACPI: table {} at phys 0x{:x} len={}",
                core::str::from_utf8(&entry.signature).unwrap_or("????"),
                addr,
                entry.data.len()
            );
            out.push(entry);
        }
    }
    out
}

unsafe fn walk_xsdt(phys: u64, hhdm: u64) -> Vec<AcpiEntry> {
    let Some(hdr) = (unsafe { read_sdt_header(phys, hhdm) }) else { return Vec::new() };
    let total = u32::from_le_bytes(hdr[4..8].try_into().unwrap_or([0; 4])) as usize;
    if total < SDT_HEADER_LEN {
        return Vec::new();
    }
    let entry_count = (total - SDT_HEADER_LEN) / 8;
    let mut out = Vec::new();
    for i in 0..entry_count {
        let entry_phys = phys + SDT_HEADER_LEN as u64 + (i as u64) * 8;
        let Some(addr) = (unsafe { read_phys_u64(entry_phys, hhdm) }) else { continue };
        if addr == 0 {
            continue;
        }
        if let Some(entry) = unsafe { read_full_table(addr, hhdm) } {
            crate::kdebug!(
                "ACPI: table {} at phys 0x{:x} len={}",
                core::str::from_utf8(&entry.signature).unwrap_or("????"),
                addr,
                entry.data.len()
            );
            out.push(entry);
        }
    }
    out
}

/// Read the 36-byte SDT header from `phys`.  Returns `None` if the address is
/// zero or the length field is out of range.
unsafe fn read_sdt_header(phys: u64, hhdm: u64) -> Option<[u8; SDT_HEADER_LEN]> {
    if phys == 0 {
        return None;
    }
    let bytes = unsafe { read_phys_bytes(phys, SDT_HEADER_LEN, hhdm) }?;
    let mut buf = [0u8; SDT_HEADER_LEN];
    buf.copy_from_slice(&bytes[..SDT_HEADER_LEN]);
    Some(buf)
}

/// Read the full ACPI SDT (header + body) from `phys`, validating its
/// checksum.  Returns `None` on any error.
unsafe fn read_full_table(phys: u64, hhdm: u64) -> Option<AcpiEntry> {
    if phys == 0 {
        return None;
    }
    let hdr = unsafe { read_sdt_header(phys, hhdm) }?;
    let sig: [u8; 4] = hdr[..4].try_into().ok()?;
    let total = u32::from_le_bytes(hdr[4..8].try_into().unwrap_or([0; 4])) as usize;

    if total < SDT_HEADER_LEN || total > MAX_TABLE_LEN {
        crate::kwarn!(
            "ACPI: table {:?} at 0x{:x} has implausible length {}",
            core::str::from_utf8(&sig).unwrap_or("????"),
            phys,
            total
        );
        return None;
    }

    let data = unsafe { read_phys_bytes(phys, total, hhdm) }?;
    if !checksum_bytes_ok(&data) {
        crate::kwarn!(
            "ACPI: checksum failed for table {:?} at 0x{:x}",
            core::str::from_utf8(&sig).unwrap_or("????"),
            phys
        );
        return None;
    }

    Some(AcpiEntry { signature: sig, phys_addr: phys, data })
}

fn checksum_bytes_ok(bytes: &[u8]) -> bool {
    let sum: u8 = bytes.iter().fold(0u8, |a, &b| a.wrapping_add(b));
    sum == 0
}

#[cfg(test)]
unsafe fn checksum_ok(virt: u64, len: usize) -> bool {
    let slice = unsafe { core::slice::from_raw_parts(virt as *const u8, len) };
    checksum_bytes_ok(slice)
}

unsafe fn read_phys_u32(phys: u64, hhdm: u64) -> Option<u32> {
    let bytes = unsafe { read_phys_bytes(phys, 4, hhdm) }?;
    Some(u32::from_le_bytes(bytes[..4].try_into().ok()?))
}

unsafe fn read_phys_u64(phys: u64, hhdm: u64) -> Option<u64> {
    let bytes = unsafe { read_phys_bytes(phys, 8, hhdm) }?;
    Some(u64::from_le_bytes(bytes[..8].try_into().ok()?))
}

unsafe fn read_phys_bytes(phys: u64, len: usize, hhdm: u64) -> Option<Vec<u8>> {
    if phys == 0 || len == 0 {
        return None;
    }
    if crate::is_runtime_initialized() {
        let rt = crate::runtime_base();
        let virt = rt.map_phys_temp(phys, len).ok()?;
        let data = unsafe { core::slice::from_raw_parts(virt as *const u8, len) }.to_vec();
        rt.unmap_phys_temp(virt, len);
        Some(data)
    } else {
        let virt = phys.wrapping_add(hhdm);
        Some(unsafe { core::slice::from_raw_parts(virt as *const u8, len) }.to_vec())
    }
}

/// Build the per-signature index map used to generate unique sysfs names.
///
/// Returns a `Vec` of `(entry_index, sysfs_name)` pairs in the same order as
/// `entries`.
pub fn sysfs_names(entries: &[AcpiEntry]) -> Vec<(usize, String)> {
    // Track how many times each 4-byte signature has appeared.
    use alloc::collections::BTreeMap;
    let mut seen: BTreeMap<[u8; 4], usize> = BTreeMap::new();
    let mut result = Vec::with_capacity(entries.len());
    for (idx, entry) in entries.iter().enumerate() {
        let occurrence = seen.entry(entry.signature).or_insert(0);
        let name = entry.sysfs_name(*occurrence);
        *occurrence += 1;
        result.push((idx, name));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal valid RSDP (ACPI 1.0) pointing to a minimal RSDT.
    fn make_test_rsdp_rsdt() -> (Vec<u8>, Vec<u8>) {
        // Build a 1-entry RSDT.
        let mut rsdt = Vec::new();
        // Signature "RSDT"
        rsdt.extend_from_slice(b"RSDT");
        // Length = 36 + 4 = 40 (one 32-bit pointer)
        rsdt.extend_from_slice(&40u32.to_le_bytes());
        // Revision, checksum (placeholder), OEM fields, etc.
        rsdt.push(1); // revision
        rsdt.push(0); // checksum placeholder
        rsdt.extend_from_slice(b"TESTOE"); // OEM ID
        rsdt.extend_from_slice(b"TESTTBID"); // OEM Table ID
        rsdt.extend_from_slice(&1u32.to_le_bytes()); // OEM revision
        rsdt.extend_from_slice(&0u32.to_le_bytes()); // creator ID
        rsdt.extend_from_slice(&0u32.to_le_bytes()); // creator revision
        // One pointer to a fake APIC table (address 0 - we'll skip it)
        rsdt.extend_from_slice(&0u32.to_le_bytes());
        // Fix checksum (sum of all bytes mod 256 = 0)
        let sum: u8 = rsdt.iter().fold(0u8, |a, &b| a.wrapping_add(b));
        rsdt[9] = rsdt[9].wrapping_sub(sum);

        // Build RSDP pointing to the RSDT at "physical" offset = 0 (we'll adjust in tests).
        let mut rsdp = Vec::new();
        rsdp.extend_from_slice(b"RSD PTR "); // signature (8 bytes)
        rsdp.push(0); // checksum placeholder
        rsdp.extend_from_slice(b"TESTOE"); // OEM ID (6 bytes)
        rsdp.push(0); // revision (0 = v1)
        rsdp.extend_from_slice(&0u32.to_le_bytes()); // RSDT addr (placeholder)
        // Fix checksum (sum of first 20 bytes = 0)
        let sum: u8 = rsdp.iter().fold(0u8, |a, &b| a.wrapping_add(b));
        rsdp[8] = rsdp[8].wrapping_sub(sum);

        (rsdp, rsdt)
    }

    #[test]
    fn test_checksum_ok_zeros() {
        let data = [0u8; 8];
        // All-zeros passes checksum (sum = 0).
        let ptr = data.as_ptr() as u64;
        assert!(unsafe { checksum_ok(ptr, 8) });
    }

    #[test]
    fn test_checksum_bad() {
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8];
        let ptr = data.as_ptr() as u64;
        assert!(!unsafe { checksum_ok(ptr, 8) });
    }

    #[test]
    fn test_sysfs_names_dedup() {
        let entries = vec![
            AcpiEntry { signature: *b"SSDT", phys_addr: 0x1000, data: vec![] },
            AcpiEntry { signature: *b"FACP", phys_addr: 0x2000, data: vec![] },
            AcpiEntry { signature: *b"SSDT", phys_addr: 0x3000, data: vec![] },
        ];
        let names = sysfs_names(&entries);
        assert_eq!(names[0].1, "SSDT");
        assert_eq!(names[1].1, "FACP");
        assert_eq!(names[2].1, "SSDT1");
    }

    #[test]
    fn test_sysfs_name_index() {
        let e = AcpiEntry { signature: *b"SSDT", phys_addr: 0, data: vec![] };
        assert_eq!(e.sysfs_name(0), "SSDT");
        assert_eq!(e.sysfs_name(1), "SSDT1");
        assert_eq!(e.sysfs_name(2), "SSDT2");
    }

    #[test]
    fn test_rsdp_invalid_signature_rejected() {
        let bad_rsdp = [0u8; 36]; // no "RSD PTR " signature
        let hhdm = bad_rsdp.as_ptr() as u64;
        // We pass the pointer itself as both rsdp_phys and hhdm=0 so
        // rsdp_virt = rsdp_phys + 0 = the pointer value.
        let result = enumerate(bad_rsdp.as_ptr() as u64, 0);
        assert!(result.is_empty(), "should reject invalid RSDP");
    }
}
