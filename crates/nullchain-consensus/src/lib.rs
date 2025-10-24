//! Consensus rules and proof-of-work for NullChain

pub mod pow;
pub mod difficulty;

pub use pow::mine_block;
pub use difficulty::adjust_difficulty;
