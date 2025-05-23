use crate::{Error, Result};
use std::{fmt::Display, str::FromStr};

/// A validated PNG chunk type. See the PNG spec for more details.
///http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    chunk_bytes: [u8; 4],
}
impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    fn bytes(&self) -> [u8; 4] {
        self.chunk_bytes
    }

    /// Returns the property state of the first byte as described in the PNG spec
    fn is_critical(&self) -> bool {
        self.chunk_bytes[0].is_ascii_uppercase()
    }
    /// Returns the property state of the second byte as described in the PNG spec
    fn is_public(&self) -> bool {
        self.chunk_bytes[1].is_ascii_uppercase()
    }
    /// Returns the property state of the third byte as described in the PNG spec
    fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_bytes[2].is_ascii_uppercase()
    }
    /// Returns the property state of the fourth byte as described in the PNG spec
    fn is_safe_to_copy(&self) -> bool {
        self.chunk_bytes[3].is_ascii_lowercase()
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// NOTE: that this chunk type should always be valid as it is validated during construction.
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && self.is_valid_byte()
    }
    /// Valid bytes are represented by the characters A-Z or a-z
    fn is_valid_byte(&self) -> bool {
        if !self.chunk_bytes.iter().all(|b| b.is_ascii()) {
            return false;
        }
        true
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if !bytes.iter().all(|b| b.is_ascii()) {
            return Err("Bytes need to be in lower or upper-case ASCII".into());
        }

        Ok(ChunkType { chunk_bytes: bytes })
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("Invalid chunk type length".into());
        }
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err("Not alphabetical characters".into());
        }

        let mut s_bytes = [0u8; 4];
        s_bytes.copy_from_slice(s.as_bytes());

        Ok(ChunkType {
            chunk_bytes: s_bytes,
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.chunk_bytes).unwrap();
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

