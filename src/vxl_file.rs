use crate::Instruction;
use alloc::vec::Vec;

/*
Byte Offset - Value

0x0 - Magic bytes (0x65, 0x58, 0x56, 0x4c)
0x4 - Executable version
0x5 - File size in bytes excluding header (Little endian)
0xd - Starting instruction offset (Little endian)
0x15 - Flags (From LSB to MSB 0 = Hash algorithm (1 = SHA3-224, 0 = SHA2-224))
0x16 - Checksum (SHA3 or SHA2 hash of the expected file. (28 bytes))
0x32 - End header byte (0xaa)
*/
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct VXLHeader {
    version: u8,
    file_size: u64,
    starting_offset: u64,
    flags: u8,
    checksum: [u8; Self::HEADER_CHECKSUM_SIZE],
}

#[derive(Clone, PartialEq, Debug)]
pub struct VXLFile {
    header: VXLHeader,
    instructions: Vec<Instruction>,
}

impl VXLHeader {
    pub const HEADER_CHECKSUM_SIZE: usize = 28;
    pub const SUPPORT_VERSIONS: [u8; 1] = [0x0];
    pub const HEADER_SIZE: usize = 50;
    pub const CHECKSUM_MASK: u8 = 0b0000_0001;
    pub const MAGIC: [u8; 4] = [0x65, 0x58, 0x56, 0x4c];
    pub const END_HEADER_BYTE: u8 = 0xaa;

    pub fn new(
        version: u8,
        file_size: u64,
        starting_offset: u64,
        flags: u8,
        checksum: [u8; Self::HEADER_CHECKSUM_SIZE],
    ) -> Self {
        return Self {
            version,
            file_size,
            starting_offset,
            flags,
            checksum,
        };
    }

    pub fn version(&self) -> u8 {
        return self.version;
    }

    pub fn size(&self) -> u64 {
        return self.file_size;
    }

    pub fn starting_offset(&self) -> u64 {
        return self.starting_offset;
    }

    pub fn flags(&self) -> u8 {
        return self.flags;
    }

    pub fn checksum(&self) -> [u8; Self::HEADER_CHECKSUM_SIZE] {
        return self.checksum;
    }

    pub fn checksum_sha3(&self) -> bool {
        return (self.flags & Self::CHECKSUM_MASK) == 1;
    }

    pub fn checksum_sha2(&self) -> bool {
        return (self.flags & Self::CHECKSUM_MASK) == 0;
    }
}

impl VXLFile {
    pub fn new(header: VXLHeader, instructions: Vec<Instruction>) -> Self {
        return Self {
            header,
            instructions,
        };
    }

    pub fn header(&self) -> VXLHeader {
        return self.header;
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        return &self.instructions;
    }
}

impl Into<Vec<u8>> for VXLHeader {
    fn into(self) -> Vec<u8> {
        let mut bytes = Self::MAGIC.to_vec();

        bytes.push(self.version);
        bytes.extend_from_slice(&self.file_size.to_le_bytes());
        bytes.extend_from_slice(&self.starting_offset.to_le_bytes());
        bytes.push(self.flags);
        bytes.extend_from_slice(&self.checksum);

        bytes.push(Self::END_HEADER_BYTE);

        return bytes;
    }
}

impl Into<Vec<u8>> for VXLFile {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.header.into();

        for instruction in self.instructions {
            let vec: Vec<u8> = instruction.into();

            bytes.extend_from_slice(&vec);
        }

        return bytes;
    }
}
