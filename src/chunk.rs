use crate::{Error, Result};
use std::fmt;

use crc::{CRC_32_ISO_HDLC, Crc};

use crate::chunk_types::ChunkType;

/// A validated PNG chunk. See the PNG Spec for more details
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(c_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let crc_calc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut to_check = Vec::new();
        to_check.extend_from_slice(&c_type.bytes());
        to_check.extend_from_slice(&data);

        let crc = crc_calc.checksum(&to_check);

        Chunk {
            length: length,
            chunk_type: c_type,
            chunk_data: data,
            crc: crc,
        }
    }

    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone())?)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::<u8>::new();

        let len_bytes: [u8; 4] = self.length.to_be_bytes();
        let c_t_bytes = self.chunk_type.bytes();
        let crc_bytes: [u8; 4] = self.crc.to_be_bytes();

        data.extend_from_slice(&len_bytes);
        data.extend_from_slice(&c_t_bytes);
        data.extend_from_slice(&self.chunk_data);
        data.extend_from_slice(&crc_bytes);

        data
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self> {
        let len_bytes = &bytes[0..4];
        //defines len of chunk_data
        let len = u32::from_be_bytes(len_bytes.try_into().unwrap());

        let chunk_type_bytes = &bytes[4..8];
        let chunk_type_array: [u8; 4] = chunk_type_bytes.try_into()?;
        let chunk_type = ChunkType::new(&chunk_type_array);

        // finds the final location of the chunk_data
        let final_data_loc = (len as usize) + 8;

        let chunk_data_bytes = &bytes[8..final_data_loc];

        let crc_bytes = &bytes[final_data_loc..final_data_loc + 4];
        let crc = u32::from_be_bytes(crc_bytes.try_into().unwrap());

        let crc_calc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut to_check = Vec::new();
        to_check.extend_from_slice(&chunk_type.bytes());
        to_check.extend_from_slice(&chunk_data_bytes);

        let crc_check = crc_calc.checksum(&to_check);

        if !crc.eq(&crc_check) {
            return Err("CRC not correct".into());
        }

        Ok(Chunk {
            length: len,
            chunk_type: chunk_type,
            chunk_data: chunk_data_bytes.to_vec(),
            crc: crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_types::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
