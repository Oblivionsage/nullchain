//! Core data types for NullChain blockchain.
//!
//! This crate defines the fundamental data structures used throughout
//! the NullChain protocol: blocks, transactions, and cryptographic primitives.

pub mod block;
pub mod transaction;
pub mod hash;
pub mod merkle;

pub use block::{Block, BlockHeader};
pub use transaction::{Transaction, TransactionOutput, TransactionInput};
pub use hash::Hash256;
pub use merkle::MerkleTree;

/// Chain parameters and constants
pub mod constants {
    /// Block time target in seconds (Bitcoin-style: 10 minutes)
    pub const BLOCK_TIME_SECONDS: u64 = 600;
    
    /// Difficulty adjustment interval (blocks)
    pub const DIFFICULTY_ADJUSTMENT_INTERVAL: u64 = 2016;
    
    /// Maximum block size in bytes (1 MB for now)
    pub const MAX_BLOCK_SIZE: usize = 1_000_000;
    
    /// Coinbase maturity (blocks before coinbase can be spent)
    pub const COINBASE_MATURITY: u64 = 100;
    
    /// Initial block reward (100 NULL)
    pub const INITIAL_BLOCK_REWARD: u64 = 100_000_000_000; // 100 * 10^9 (nanoNULL)
    
    /// Halving interval (every 210,000 blocks, ~4 years)
    pub const HALVING_INTERVAL: u64 = 210_000;
}
