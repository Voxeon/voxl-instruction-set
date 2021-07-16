use alloc::vec;
use alloc::vec::Vec;

/// Defines some properties any instruction argument should have.
pub trait InstructionArgument {
    /// The number of bits for this instruction argument type.
    const BIT_SIZE: usize;

    /// The number of bytes for this instruction argument type.
    const BYTES: usize = (Self::BIT_SIZE + Self::BIT_SIZE % 8) / 8;

    /// Should guarantee a conversion to a vector of bytes.
    fn into_bytes(self) -> Vec<u8>;
}

/// This enum defines all the possible registers supported by the voxl-vm and their value as a byte.
///
/// # Registers
/// ### Special Registers
/// 0000 - rsp - Stack pointer
///
/// 0001 - rfp - Frame pointer
///
/// 0010 - rou - Result from instructions/returns
///
/// 0011 - rfl - Flags register (Largely just reserved. LSB 1 - equal, LSB 2 - less than, LSB 3 - greater than)
///
/// ### Reserved registers
/// 0100 - rra - Reserved a
///
/// 0101 - rrb - Reserved b
///
/// ### General purpose registers
/// 0110 - r0
///
/// 0111 - r1
///
/// 1000 - r2
///
/// 1001 - r3
///
/// 1010 - r4
///
/// 1011 - r5
///
/// 1100 - r6
///
/// 1101 - r7
///
/// 1110 - r8
///
/// 1111 - r9
///
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

    /// Reserved A
    RRA = 4,
    /// Reserved B
    RRB = 5,

    /// General purpose register 0
    R0 = 6,
    /// General purpose register 1
    R1 = 7,
    /// General purpose register 2
    R2 = 8,
    /// General purpose register 3
    R3 = 9,
    /// General purpose register 4
    R4 = 10,
    /// General purpose register 5
    R5 = 11,
    /// General purpose register 6
    R6 = 12,
    /// General purpose register 7
    R7 = 13,
    /// General purpose register 8
    R8 = 14,
    /// General purpose register 9
    R9 = 15,
}

/// This struct represents an address in memory, it is an absolute value and used in multiple contexts,
/// it can be used to represent the unique identifier of heap allocated memory or the offset from the
/// start of the program for a specific instruction
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Address {
    absolute_address: u64,
}

/// Represents an immediate value as a 64-bit value. Could be a float, signed or unsigned integer.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Immediate {
    bytes: [u8; Immediate::BYTES],
}

impl Address {
    /// Creates a new address from an absolute address.
    pub fn new(absolute_address: u64) -> Self {
        return Self { absolute_address };
    }
}

impl Register {
    /// Returns the register variant for the specified indicator. Upper 4 bits are ignored.
    pub fn from_bits(indicator: u8) -> Register {
        return indicator.into();
    }
}

impl Immediate {
    /// Creates a new immediate from the constituent bytes.
    pub fn new(bytes: [u8; Immediate::BYTES]) -> Self {
        return Self { bytes };
    }
}

impl InstructionArgument for Address {
    const BIT_SIZE: usize = 64;

    fn into_bytes(self) -> Vec<u8> {
        return Into::<[u8; Self::BYTES]>::into(self).to_vec();
    }
}

impl Into<u64> for Address {
    fn into(self) -> u64 {
        return self.absolute_address;
    }
}

impl From<u64> for Address {
    fn from(n: u64) -> Self {
        return Self {
            absolute_address: n,
        };
    }
}

impl Into<[u8; Address::BYTES]> for Address {
    fn into(self) -> [u8; Address::BYTES] {
        return Into::<u64>::into(self).to_le_bytes();
    }
}

impl From<[u8; Immediate::BYTES]> for Address {
    fn from(bytes: [u8; Immediate::BYTES]) -> Self {
        return Self::new(u64::from_le_bytes(bytes));
    }
}

impl InstructionArgument for Register {
    const BIT_SIZE: usize = 4;

    fn into_bytes(self) -> Vec<u8> {
        return vec![self as u8];
    }
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

impl InstructionArgument for Immediate {
    const BIT_SIZE: usize = 64;

    fn into_bytes(self) -> Vec<u8> {
        return Into::<[u8; Self::BYTES]>::into(self).to_vec();
    }
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

impl From<u64> for Immediate {
    fn from(n: u64) -> Self {
        return Self::new(n.to_le_bytes());
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

impl Into<[u8; Immediate::BYTES]> for Immediate {
    fn into(self) -> [u8; Immediate::BYTES] {
        return Into::<u64>::into(self).to_le_bytes();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Instruction;
    use alloc::vec;
    use alloc::vec::Vec;

    macro_rules! generate_test {
        ($variant:expr, $output:expr, $name:ident) => {
            #[test]
            pub fn $name() {
                let instruction: Instruction = $variant;

                let bytes: Vec<u8> = instruction.into();

                assert_eq!(bytes, $output);
            }
        };
    }

    generate_test!(Instruction::Nop, vec![0x0], test_nop_into_bytes);

    generate_test!(
        Instruction::Syscall(Immediate::from(52u64)),
        vec![0x1, 0x34, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        test_syscall_into_bytes
    );

    generate_test!(
        Instruction::Malloc(Register::R0, Register::R1),
        vec![0x9, 0b0110_0111],
        test_malloc_into_bytes
    );

    generate_test!(
        Instruction::Malloci(Immediate::from(10u64), Register::R0),
        vec![0xa, 0xa, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0b0110_0000],
        test_malloci_into_bytes
    );

    generate_test!(
        Instruction::Copy(
            Register::R5,
            Register::R0,
            Register::R6,
            Register::R1,
            Register::R7
        ),
        vec![0x18, 0b1011_0110, 0b1100_0111, 0b1101_0000],
        test_copy_into_bytes
    );

    generate_test!(
        Instruction::Copyi(
            Immediate::from(0x45u64),
            Immediate::from(0x12u64),
            Immediate::from(0x33u64),
            Register::R0,
            Register::R1
        ),
        vec![
            0x19,
            0x45,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x12,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x33,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0x0,
            0b0110_0111
        ],
        test_copyi_into_bytes
    );

    generate_test!(
        Instruction::Jmp(Address::from(5u64)),
        vec![0x37, 5, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        test_jmp_into_bytes
    );
}
