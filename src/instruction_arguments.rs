pub trait InstructionArgument {
    const BIT_SIZE: usize;
    const BYTES: usize = (Self::BIT_SIZE + Self::BIT_SIZE % 8) / 8;
}

/*
Registers (16)

Special Registers
0000 rsp - Stack pointer
0001 rfp - Frame pointer
0010 rou - Result from instructions/returns
0011 rfl - Flags register (Largely just reserved. LSB 1 - equal, LSB 2 - less than, LSB 3 - greater than)

Reserved registers
0100 ra - Reserved a
0101 rb - Reserved b

General purpose registers
0110 r0
0111 r1
1000 r2
1001 r3
1010 r4
1011 r5
1100 r6
1101 r7
1110 r8
1111 r9
 */
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Register {
    /// Stack pointer
    RSP = 0,
    /// Frame pointer
    RFP = 1,
    /// Output register
    ROU = 2,
    /// Flags register
    RFL = 3,

    /// Reserved
    RRA = 4,
    RRB = 5,

    // General purpose registers
    R0 = 6,
    R1 = 7,
    R2 = 8,
    R3 = 9,
    R4 = 10,
    R5 = 11,
    R6 = 12,
    R7 = 13,
    R8 = 14,
    R9 = 15,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Address {
    absolute_address: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Immediate {
    bytes: [u8; Immediate::BYTES],
}

impl Address {
    pub fn new(absolute_address: u64) -> Self {
        return Self { absolute_address };
    }
}

impl Into<u64> for Address {
    fn into(self) -> u64 {
        return self.absolute_address;
    }
}

impl InstructionArgument for Address {
    const BIT_SIZE: usize = 64;
}

impl From<[u8; Immediate::BYTES]> for Address {
    fn from(bytes: [u8; Immediate::BYTES]) -> Self {
        return Self::new(u64::from_le_bytes(bytes));
    }
}

impl Register {
    /// Returns the register variant for the specified indicator. Upper 4 bits are ignored.
    pub fn from_bits(indicator: u8) -> Register {
        return indicator.into();
    }
}

impl InstructionArgument for Register {
    const BIT_SIZE: usize = 4;
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        return match value {
            0 => Self::RSP,
            1 => Self::RFP,
            2 => Self::ROU,
            3 => Self::RFL,
            4 => Self::RRA,
            5 => Self::RRB,

            6 => Self::R0,
            7 => Self::R1,
            8 => Self::R2,
            9 => Self::R3,
            10 => Self::R4,
            11 => Self::R5,
            12 => Self::R6,
            13 => Self::R7,
            14 => Self::R8,
            15 => Self::R9,
            _ => panic!("Unknown register {}", value),
        };
    }
}

impl Immediate {
    pub fn new(bytes: [u8; Immediate::BYTES]) -> Self {
        return Self { bytes };
    }
}

impl InstructionArgument for Immediate {
    const BIT_SIZE: usize = 64;
}

impl From<[u8; Immediate::BYTES]> for Immediate {
    fn from(bytes: [u8; Immediate::BYTES]) -> Self {
        return Self::new(bytes);
    }
}

impl From<i64> for Immediate {
    fn from(n: i64) -> Self {
        return Self::new(n.to_le_bytes());
    }
}

impl Into<u64> for Immediate {
    fn into(self) -> u64 {
        return u64::from_le_bytes(self.bytes);
    }
}

impl From<f64> for Immediate {
    fn from(n: f64) -> Self {
        return Self::new(n.to_le_bytes());
    }
}

impl From<u8> for Immediate {
    fn from(n: u8) -> Self {
        return Self::new([n, 0, 0, 0, 0, 0, 0, 0]);
    }
}
