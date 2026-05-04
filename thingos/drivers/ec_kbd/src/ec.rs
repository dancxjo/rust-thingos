pub const DATA_PORT: usize = 0x62;
pub const COMMAND_PORT: usize = 0x66;

pub const STATUS_OBF: u8 = 1 << 0;
pub const STATUS_IBF: u8 = 1 << 1;
pub const STATUS_SCI_EVT: u8 = 1 << 5;

pub const CMD_QUERY: u8 = 0x84;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EcStatus(pub u8);

impl EcStatus {
    pub const fn output_full(self) -> bool {
        self.0 & STATUS_OBF != 0
    }

    pub const fn input_full(self) -> bool {
        self.0 & STATUS_IBF != 0
    }

    pub const fn sci_event(self) -> bool {
        self.0 & STATUS_SCI_EVT != 0
    }

    pub const fn looks_absent(self) -> bool {
        self.0 == 0xff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_bits_are_decoded() {
        let status = EcStatus(STATUS_OBF | STATUS_SCI_EVT);
        assert!(status.output_full());
        assert!(status.sci_event());
        assert!(!status.input_full());
        assert!(!status.looks_absent());
        assert!(EcStatus(0xff).looks_absent());
    }
}
