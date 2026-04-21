//! Typed VFS provider attributes transported via `DeviceCall`.
//!
//! These constants and wire headers define a small, stable protocol for
//! provider-specific metadata (xattr-like values) carried over
//! `DeviceKind::Attr`.

/// Attribute value type tag.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrType {
    Bytes = 1,
    Utf8 = 2,
    Bool = 3,
    I64 = 4,
    U64 = 5,
    F64 = 6,
}

impl AttrType {
    #[inline]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Bytes),
            2 => Some(Self::Utf8),
            3 => Some(Self::Bool),
            4 => Some(Self::I64),
            5 => Some(Self::U64),
            6 => Some(Self::F64),
            _ => None,
        }
    }
}

/// `DeviceCall.op`: get one attribute by name.
pub const ATTR_OP_GET: u32 = 1;
/// `DeviceCall.op`: set one attribute.
pub const ATTR_OP_SET: u32 = 2;
/// `DeviceCall.op`: remove one attribute by name.
pub const ATTR_OP_REMOVE: u32 = 3;
/// `DeviceCall.op`: list attribute names and declared value types.
pub const ATTR_OP_LIST: u32 = 4;

/// Header for GET request payload.
///
/// Payload layout:
/// `[AttrNameHeader][name bytes (UTF-8)]`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AttrNameHeader {
    pub name_len: u16,
    pub _reserved: u16,
}

/// Header for SET request payload.
///
/// Payload layout:
/// `[AttrSetHeader][name bytes][value bytes]`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AttrSetHeader {
    pub name_len: u16,
    pub value_type: u8,
    pub flags: u8,
    pub value_len: u32,
}

/// Header prefixed to GET response payload.
///
/// Response payload:
/// `[AttrValueHeader][value bytes]`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AttrValueHeader {
    pub value_type: u8,
    pub flags: u8,
    pub _reserved: u16,
    pub value_len: u32,
}

/// One LIST entry.
///
/// Entry layout:
/// `[AttrListEntryHeader][name bytes (UTF-8)]`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AttrListEntryHeader {
    pub name_len: u16,
    pub value_type: u8,
    pub flags: u8,
    pub value_len: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abi_sizes_are_stable() {
        assert_eq!(core::mem::size_of::<AttrNameHeader>(), 4);
        assert_eq!(core::mem::size_of::<AttrSetHeader>(), 8);
        assert_eq!(core::mem::size_of::<AttrValueHeader>(), 8);
        assert_eq!(core::mem::size_of::<AttrListEntryHeader>(), 8);
    }
}
