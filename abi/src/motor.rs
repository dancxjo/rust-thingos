/// Canonical Motor descriptor ABI version.
pub const MOTOR_DESCRIPTOR_ABI_VERSION: u32 = 1;

/// Canonical symbol exported by Motor-capable binaries.
pub const MOTOR_DESCRIPTOR_SYMBOL: &str = "THINGOS_MOTOR";

/// Interface id for `ProgramV1`.
pub const MOTOR_INTERFACE_PROGRAM_V1: u32 = 1;
/// Interface id for `LifecycleV1`.
pub const MOTOR_INTERFACE_LIFECYCLE_V1: u32 = 2;
/// Interface id for `DriverV1`.
pub const MOTOR_INTERFACE_DRIVER_V1: u32 = 3;

/// Hosting mode bit: binary can be launched as a regular program.
pub const MOTOR_HOST_PROGRAM: u64 = 1 << 0;
/// Hosting mode bit: binary can be hosted as a resident lifecycle service.
pub const MOTOR_HOST_LIFECYCLE: u64 = 1 << 1;
/// Hosting mode bit: binary can be hosted as a driver.
pub const MOTOR_HOST_DRIVER: u64 = 1 << 2;

/// Interface declaration entry inside a [`MotorDescriptor`].
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MotorInterfaceDescriptor {
    pub interface_id: u32,
    pub interface_version: u32,
    pub flags: u32,
    pub reserved: u32,
    /// Optional symbol name for the interface entry surface.
    ///
    /// For `ProgramV1`, a zero length means "use default ELF entry" which
    /// keeps plain `main`/crt-style program flow working.
    pub entry_symbol_ptr: *const u8,
    pub entry_symbol_len: usize,
}

/// Canonical descriptor for image-built loadable binaries ("Motors").
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MotorDescriptor {
    pub abi_version: u32,
    pub interface_count: u32,
    pub hosting_modes: u64,
    pub capabilities: u64,
    pub motor_name_ptr: *const u8,
    pub motor_name_len: usize,
    pub interfaces: [MotorInterfaceDescriptor; 4],
}

unsafe impl Sync for MotorDescriptor {}
unsafe impl Send for MotorDescriptor {}

impl MotorDescriptor {
    #[inline]
    pub fn interface(
        &self,
        interface_id: u32,
        interface_version: u32,
    ) -> Option<MotorInterfaceDescriptor> {
        let mut i = 0usize;
        let limit = core::cmp::min(self.interface_count as usize, self.interfaces.len());
        while i < limit {
            let iface = self.interfaces[i];
            if iface.interface_id == interface_id && iface.interface_version == interface_version {
                return Some(iface);
            }
            i += 1;
        }
        None
    }

    #[inline]
    pub fn implements_driver_v1(&self) -> bool {
        self.interface(MOTOR_INTERFACE_DRIVER_V1, 1).is_some()
    }

    #[inline]
    pub fn implements_program_v1(&self) -> bool {
        self.interface(MOTOR_INTERFACE_PROGRAM_V1, 1).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interface_lookup_finds_declared_interface() {
        let descriptor = MotorDescriptor {
            abi_version: MOTOR_DESCRIPTOR_ABI_VERSION,
            interface_count: 2,
            hosting_modes: MOTOR_HOST_PROGRAM,
            capabilities: 0,
            motor_name_ptr: core::ptr::null(),
            motor_name_len: 0,
            interfaces: [
                MotorInterfaceDescriptor {
                    interface_id: MOTOR_INTERFACE_PROGRAM_V1,
                    interface_version: 1,
                    flags: 0,
                    reserved: 0,
                    entry_symbol_ptr: core::ptr::null(),
                    entry_symbol_len: 0,
                },
                MotorInterfaceDescriptor {
                    interface_id: MOTOR_INTERFACE_DRIVER_V1,
                    interface_version: 1,
                    flags: 0,
                    reserved: 0,
                    entry_symbol_ptr: core::ptr::null(),
                    entry_symbol_len: 0,
                },
                MotorInterfaceDescriptor::default(),
                MotorInterfaceDescriptor::default(),
            ],
        };

        assert!(descriptor.implements_program_v1());
        assert!(descriptor.implements_driver_v1());
        assert!(descriptor.interface(MOTOR_INTERFACE_LIFECYCLE_V1, 1).is_none());
    }
}
