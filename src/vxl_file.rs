use alloc::vec::Vec;

/// Defines what the header of a vxl file should contain.
///
/// # Representation when in binary
/// Byte Offset - Value
///
/// 0x0 - Magic bytes (0x65, 0x58, 0x56, 0x4c)
///
/// 0x4 - Executable version
///
/// 0x5 - File size in bytes excluding header (Little endian)
///
/// 0xd - Starting instruction offset (Little endian)
///
/// 0x15 - Flags (From LSB to MSB 0 = Hash algorithm (1 = SHA3-224, 0 = SHA2-224))
///
/// 0x16 - Checksum (SHA3 or SHA2 hash of the expected file. (28 bytes))
///
/// 0x32 - End header byte (0xaa)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct VXLHeader {
    version: u8,
    file_size: u64,
    starting_offset: u64,
    flags: u8,
    checksum: [u8; Self::HEADER_CHECKSUM_SIZE],
}

/// Represents an executable vxl file. A vxl file is simply the header followed by the program bytes.
#[derive(Clone, PartialEq, Debug)]
pub struct VXLFile {
    header: VXLHeader,
    contents: Vec<u8>,
}

impl VXLHeader {
    /// The size in bytes of the checksum
    pub const HEADER_CHECKSUM_SIZE: usize = 28;
    /// The list of supported vxl versions.
    pub const SUPPORTED_VERSIONS: [u8; 1] = [0x0];
    /// The full size fo the header.
    pub const HEADER_SIZE: usize = 50;
    /// The mask when the bitwise 'and' operation is applied to the flag gets the value of the checksum algorithm.
    pub const CHECKSUM_MASK: u8 = 0b0000_0001;
    /// The magic bytes: 0x65, 0x58, 0x56, 0x4c. These are used to verify that the header
    /// is indeed a vxl file.
    pub const MAGIC: [u8; 4] = [0x65, 0x58, 0x56, 0x4c];
    /// A byte that is used to mark the end of the header.
    pub const END_HEADER_BYTE: u8 = 0xaa;

    /// Creates a new instance of the header.
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

    /// The version of this header.
    pub fn version(&self) -> u8 {
        return self.version;
    }

    /// The size of the file that this is the header of.
    pub fn size(&self) -> u64 {
        return self.file_size;
    }

    /// The instruction index at which the vm should start executing from
    pub fn starting_offset(&self) -> u64 {
        return self.starting_offset;
    }

    /// The flags for this header.
    pub fn flags(&self) -> u8 {
        return self.flags;
    }

    /// The stored checksum hash for the file instruction bytes only.
    pub fn checksum(&self) -> [u8; Self::HEADER_CHECKSUM_SIZE] {
        return self.checksum;
    }

    /// True if this header's checksum was computed using SHA3-224.
    pub fn checksum_sha3(&self) -> bool {
        return (self.flags & Self::CHECKSUM_MASK) == 1;
    }

    /// True if this header's checksum was computed using SHA2-224.
    pub fn checksum_sha2(&self) -> bool {
        return (self.flags & Self::CHECKSUM_MASK) == 0;
    }
}

impl VXLFile {
    /// Creates a new instance of a VXLFile.
    pub fn new(header: VXLHeader, contents: Vec<u8>) -> Self {
        return Self { header, contents };
    }

    /// Returns a copy of the header for this file
    pub fn header(&self) -> VXLHeader {
        return self.header;
    }

    /// Returns a reference to the program bytes for this file.
    pub fn contents(&self) -> &Vec<u8> {
        return &self.contents;
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
    fn into(mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.header.into();

        bytes.append(&mut self.contents);

        return bytes;
    }
}
