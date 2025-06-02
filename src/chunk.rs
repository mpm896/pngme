use core::fmt;
use std::string::FromUtf8Error;
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::{ChunkType, CHUNK_SIZE};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunktype: ChunkType,
    data: Vec<u8>,
    crc: u32
}

const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            Err("Chunk does not contain enough information")
        } else {
            let length: u32 = u32::from_be_bytes(
                bytes[0..4]
                .try_into()
                .unwrap()
            );
            let chunktype_bytes: [u8; CHUNK_SIZE] = bytes[4..(4 + CHUNK_SIZE)].try_into().unwrap(); 
            let chunktype: ChunkType = ChunkType::try_from(chunktype_bytes).unwrap();
            let data: Vec<u8> = bytes[8..(8 + length as usize)].to_vec();
            let crc: u32 = u32::from_be_bytes(
                bytes[(8 + length as usize)..(12 + length as usize)]
                .try_into()
                .unwrap()
            );

            // Validate the crc
            let mut input_crc = Vec::new();
            input_crc.extend_from_slice(&chunktype_bytes);
            input_crc.extend_from_slice(&data);
            let computed_crc = PNG_CRC.checksum(&input_crc);

            if crc != computed_crc {
                return Err("CRC mismatch")
            }


            Ok(Self {
                length,
                chunktype,
                data,
                crc
            })
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bind_data = self.data.clone();
        let s: String = match String::from_utf8(bind_data) {
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

impl Chunk {
    pub fn new(chunktype: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;

        // Create a [u8] of chunktype.bytes() + data to calculate the crc
        let prec_bytes: Vec<u8> = chunktype.bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        let checksum = PNG_CRC.checksum(&prec_bytes);

        // Construct the Chunk
        Self {
            length,
            chunktype,
            data,
            crc: checksum
        }

    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunktype
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        Ok(String::from_utf8(self.data.clone()).expect("Data are not valid UTF-8"))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
        .to_be_bytes()
        .iter()
        .chain(self.chunktype.bytes().iter())
        .chain(self.data.iter())
        .chain(self.crc.to_be_bytes().iter())
        .copied()
        .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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