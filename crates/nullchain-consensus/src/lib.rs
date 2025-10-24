//! Consensus rules and proof-of-work for NullChain

pub mod difficulty;
pub mod pow;

pub use difficulty::adjust_difficulty;
pub use pow::mine_block;
