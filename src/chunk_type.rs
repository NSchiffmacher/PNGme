use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    code: [u8; 4],
}

impl ChunkType {
    /// Returns the raw bytes contained in this chunk
    pub fn bytes(&self) -> [u8; 4] {
        self.code
    }
    
    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    /// Returns the property state of the first byte as described in the PNG spec
    pub fn is_critical(&self) -> bool {
        self.code[0].is_ascii_uppercase()
    }

    /// Returns the property state of the second byte as described in the PNG spec
    pub fn is_public(&self) -> bool {
        self.code[1].is_ascii_uppercase()
    }

    /// Returns the property state of the third byte as described in the PNG spec
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.code[2].is_ascii_uppercase()
    }

    /// Returns the property state of the fourth byte as described in the PNG spec
    pub fn is_safe_to_copy(&self) -> bool {
        self.code[3].is_ascii_lowercase()
    }
}

impl FromStr for ChunkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() != 4 {
            return Err(format!(
                "Can't create chunk type with size {} (expected 4)",
                bytes.len()
            ));
        };

        let bytes: [u8; 4] = bytes.try_into().unwrap();
        Ok(ChunkType::try_from(bytes)?)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match value.iter().all(|&elem| elem.is_ascii_alphabetic()) {
            true => Ok(ChunkType { code: value }),
            false => Err("Can't create chunk type with invalid data".to_string()),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.bytes().to_vec()).expect("Crash in impl Display for ChunkType (how tf do I propagate this error??)"))
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
        println!("Chunk: {chunk}");
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
