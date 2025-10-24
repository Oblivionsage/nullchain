use crate::hash::Hash256;
use crate::transaction::Transaction;
use crate::merkle::MerkleTree;
use serde::{Deserialize, Serialize};

/// Block header (metadata for proof-of-work)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block version
    pub version: u32,
    
    /// Hash of the previous block
    pub previous_block: Hash256,
    
    /// Merkle root of transactions
    pub merkle_root: Hash256,
    
    /// Block timestamp (Unix time)
    pub timestamp: u64,
    
    /// Difficulty target (compact representation)
    pub bits: u32,
    
    /// Nonce for proof-of-work
    pub nonce: u64,
}

impl BlockHeader {
    /// Serialize header for hashing (deterministic)
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).expect("BlockHeader serialization failed")
    }
    
    /// Calculate the hash of this header
    pub fn hash(&self) -> Hash256 {
        use blake3::Hasher;
        
        let serialized = self.serialize();
        let mut hasher = Hasher::new();
        hasher.update(&serialized);
        
        // Double hash for PoW (like Bitcoin)
        let first_hash = hasher.finalize();
        let mut second_hasher = Hasher::new();
        second_hasher.update(first_hash.as_bytes());
        
        Hash256::from_bytes(*second_hasher.finalize().as_bytes())
    }
}

/// Block (collection of transactions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    
    /// Transactions in this block
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Create genesis block (first block in the chain)
    pub fn genesis() -> Self {
        let coinbase = Transaction::coinbase(
            vec![0u8; 20], // Genesis address (burn address)
            crate::constants::INITIAL_BLOCK_REWARD,
            0,
        );
        
        Self {
            header: BlockHeader {
                version: 1,
                previous_block: Hash256::zero(),
                merkle_root: Hash256::zero(), // Will be computed
                timestamp: 1609459200, // 2021-01-01 00:00:00 UTC (symbolic)
                bits: 0x1d00ffff, // Initial difficulty (Bitcoin's)
                nonce: 0,
            },
            transactions: vec![coinbase],
        }
    }
    
    /// Calculate the merkle root of all transactions
    pub fn calculate_merkle_root(&self) -> Hash256 {
        if self.transactions.is_empty() {
            return Hash256::zero();
        }
        
        // Hash each transaction
        let tx_hashes: Vec<Hash256> = self.transactions
            .iter()
            .map(|tx| {
                let serialized = bincode::serialize(tx).expect("tx serialization");
                use blake3::Hasher;
                let mut hasher = Hasher::new();
                hasher.update(&serialized);
                Hash256::from_bytes(*hasher.finalize().as_bytes())
            })
            .collect();
        
        let tree = MerkleTree::new(tx_hashes);
        tree.root()
    }
    
    /// Get the hash of this block
    pub fn hash(&self) -> Hash256 {
        self.header.hash()
    }
    
    /// Check if the block satisfies the difficulty target
    pub fn meets_difficulty_target(&self) -> bool {
        let hash = self.hash();
        let target = Self::bits_to_target(self.header.bits);
        
        // Compare as big-endian (most significant bytes first)
        for i in (0..32).rev() {
            if hash.as_bytes()[i] < target.as_bytes()[i] {
                return true;
            } else if hash.as_bytes()[i] > target.as_bytes()[i] {
                return false;
            }
        }
        true // Equal is also valid
    }
    
    /// Convert compact bits representation to full target
    /// Simplified version for testing
    fn bits_to_target(bits: u32) -> Hash256 {
        let exponent = ((bits >> 24) & 0xff) as usize;
        let mantissa = bits & 0x00ffffff;
        
        let mut target = [0xffu8; 32];
        
        if exponent <= 3 {
            // Very small target
            let bytes = mantissa.to_le_bytes();
            for i in 0..32 {
                target[i] = if i < exponent { bytes[i] } else { 0 };
            }
        } else if exponent < 32 {
            // Normal case
            let start = exponent - 3;
            target[start..32].fill(0xff);
            
            let mantissa_bytes = mantissa.to_be_bytes();
            if start < 32 {
                target[start] = mantissa_bytes[1];
            }
            if start + 1 < 32 {
                target[start + 1] = mantissa_bytes[2];
            }
            if start + 2 < 32 {
                target[start + 2] = mantissa_bytes[3];
            }
            
            // Fill rest with zeros
            for i in (start + 3)..32 {
                target[i] = 0;
            }
        }
        
        Hash256::from_bytes(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        
        assert_eq!(genesis.header.version, 1);
        assert_eq!(genesis.header.previous_block, Hash256::zero());
        assert_eq!(genesis.transactions.len(), 1);
        assert!(genesis.transactions[0].is_coinbase());
    }
    
    #[test]
    fn test_block_hash() {
        let genesis = Block::genesis();
        let hash1 = genesis.hash();
        let hash2 = genesis.hash();
        
        // Hashing should be deterministic
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, Hash256::zero());
    }
    
    #[test]
    fn test_merkle_root_calculation() {
        let mut genesis = Block::genesis();
        let merkle_root = genesis.calculate_merkle_root();
        
        assert_ne!(merkle_root, Hash256::zero());
        
        // Set the merkle root
        genesis.header.merkle_root = merkle_root;
        assert_eq!(genesis.header.merkle_root, merkle_root);
    }
    
    #[test]
    fn test_header_serialization() {
        let genesis = Block::genesis();
        let serialized = genesis.header.serialize();
        
        // Should be deterministic
        assert!(!serialized.is_empty());
        assert_eq!(serialized, genesis.header.serialize());
    }
    
    #[test]
    fn test_easy_difficulty() {
        let mut block = Block::genesis();
        // Very easy: just need first byte < 0x10
        block.header.bits = 0x1f0fffff;
        
        // Should find quickly
        for nonce in 0..100000 {
            block.header.nonce = nonce;
            if block.meets_difficulty_target() {
                println!("Found at nonce: {}", nonce);
                return;
            }
        }
        panic!("Should have found a block");
    }
}
