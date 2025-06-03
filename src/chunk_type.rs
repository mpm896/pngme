use core::str;
use std::fmt::{self, Error};
use std::str::FromStr;

pub const CHUNK_SIZE: usize = 4;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; CHUNK_SIZE]);

impl TryFrom<[u8; CHUNK_SIZE]> for ChunkType {
    type Error = &'static str;

    fn try_from(bytes: [u8; CHUNK_SIZE]) -> Result<Self, Self::Error> {
        if bytes.iter().all(|&c| (c as char).is_ascii_alphabetic()) {
            Ok(Self(bytes))
        } else {
            Err("Chunk type is not ascii alphabetic")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    /// Parse a byte array from a string. If string not valid, return Err
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunk_type = s.trim();

        // Ensure chunk type is valid size and ascii chars
        if chunk_type.len() != CHUNK_SIZE {
            Err("Invalid size of chunk type.")
        } else if !chunk_type.chars().all(|c| c.is_ascii_alphabetic()) {
            Err("Invalid ascii characters.")
        } else {
            let mut bytes = [0; CHUNK_SIZE];
            bytes.clone_from_slice(chunk_type.as_bytes());
            Ok(Self(bytes))
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binding = &self.bytes();
        let s: &str = match str::from_utf8(binding) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8: {}", e)
        };
        write!(
            f,
            "{}",
            s
        )
    }
}

impl ChunkType {

    /// Get the bytes of the Chunk Type
    pub fn bytes(&self) -> [u8; CHUNK_SIZE] {
        self.0
    }

    pub fn at_byte(&self, idx: usize) -> char {
        self.bytes()[idx] as char
    }

    
    pub fn is_valid(&self) -> bool {
        !self.is_public() && self.is_reserved_bit_valid()
    }

    /// Check the ancilliary bit (bit 5 of first byte). Return true if critical.
    pub fn is_critical(&self) -> bool {
        self.at_byte(0).is_uppercase()
    }

    /// Check the private bit (bit 5 of second byte). Return true if private.
    pub fn is_public(&self) -> bool {
        self.at_byte(1).is_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.at_byte(2).is_uppercase()
    }

    /// Check the safe-to-copy bit (bit 5 of fourth byte). Return true if safe to copy.
    pub fn is_safe_to_copy(&self) -> bool {
        self.at_byte(3).is_lowercase()
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