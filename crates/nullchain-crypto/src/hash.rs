//! Hashing functions using Blake3

use nullchain_types::Hash256;

/// Hash arbitrary data using Blake3
pub fn hash_data(data: &[u8]) -> Hash256 {
    let hash = blake3::hash(data);
    Hash256::from_bytes(*hash.as_bytes())
}

/// Double hash (hash of hash) - used for block hashing
pub fn double_hash(data: &[u8]) -> Hash256 {
    let first = blake3::hash(data);
    let second = blake3::hash(first.as_bytes());
    Hash256::from_bytes(*second.as_bytes())
}

/// Hash a block header
pub fn hash_block_header(header: &[u8]) -> Hash256 {
    double_hash(header)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_deterministic() {
        let data = b"nullchain";
        let hash1 = hash_data(data);
        let hash2 = hash_data(data);
        assert_eq!(hash1, hash2);
    }
    
    #[test]
    fn test_double_hash_different() {
        let data = b"test";
        let single = hash_data(data);
        let double = double_hash(data);
        assert_ne!(single, double);
    }
}
