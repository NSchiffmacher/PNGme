use crate::chunk_type::ChunkType;
use crate::Result;

use std::convert::TryFrom;
use std::fmt::Display;
use std::error::Error;

use crc::{Crc, CRC_32_ISO_HDLC};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    /// Creates a new chunk based on his type and its data
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { chunk_type, data }
    }

    /// Returns the length of the chunk's data
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    /// Returns the type of the chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Returns the raw data of the chunk
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the CRC of the chunk 
    /// The check includes the type's bytes and the raw data's bytes 
    pub fn crc(&self) -> u32 {
        let data: Vec<_> = self.chunk_type.bytes().iter()
                                        .chain(self.data.iter())
                                        .copied()
                                        .collect();
        Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&data)
    }

    /// Returns the data as a String
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }

    /// Returns the raw bytes of the whole chunk (length + type + data + CRC)
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[derive(Debug)]
pub struct ChunkDecodingError {
    reason: String,
}
impl ChunkDecodingError {
    fn boxed(reason: String) -> Box<Self> {
        Box::new(Self { reason })
    }
}
impl std::fmt::Display for ChunkDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad chunk: {}", self.reason)
    }
}
impl Error for ChunkDecodingError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;
    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let length = u32::from_be_bytes((&value[0..4]).try_into()?);
        let chunk_type: ChunkType = <[u8; 4]>::try_from(&value[4..8])?.try_into()?;
        let chunk_data = value[8..8+(length as usize)].iter().copied().collect();
        let crc = u32::from_be_bytes((&value[8+(length as usize)..]).try_into()?);

        let chunk = Chunk::new(chunk_type, chunk_data);
        if chunk.crc() != crc {
            Err(ChunkDecodingError::boxed(format!("CRC mismatch (received {}, expected {})", crc, chunk.crc())))
        } else {
            Ok(chunk)
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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