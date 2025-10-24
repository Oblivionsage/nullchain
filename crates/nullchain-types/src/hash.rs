use serde::{Deserialize, Serialize};
use std::fmt;

/// 256-bit hash output
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash256([u8; 32]);

impl Hash256 {
    /// Create from byte array
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    /// Get byte array
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    
    /// All zeros (used for genesis)
    pub const fn zero() -> Self {
        Self([0u8; 32])
    }
}

impl fmt::Display for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hash256({})", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_display() {
        let hash = Hash256::from_bytes([1u8; 32]);
        let display = format!("{}", hash);
        assert_eq!(display.len(), 64); // 32 bytes * 2 hex chars
        assert!(display.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_zero_hash() {
        let zero = Hash256::zero();
        assert_eq!(zero.as_bytes(), &[0u8; 32]);
    }
}
