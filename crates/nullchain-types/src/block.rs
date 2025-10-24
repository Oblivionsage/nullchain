use crate::hash::Hash256;
use crate::transaction::Transaction;
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
}
