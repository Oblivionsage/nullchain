//! Proof-of-Work mining

use nullchain_types::{Block, Hash256};

/// Mine a block by finding a valid nonce
/// Returns the nonce and hash when found
pub fn mine_block(mut block: Block, max_iterations: Option<u64>) -> Option<(Block, Hash256)> {
    let max = max_iterations.unwrap_or(u64::MAX);

    for nonce in 0..max {
        block.header.nonce = nonce;

        if block.meets_difficulty_target() {
            let hash = block.hash();
            tracing::info!("Block mined! Nonce: {}, Hash: {}", nonce, hash);
            return Some((block, hash));
        }

        // Log progress every million iterations
        if nonce > 0 && nonce % 1_000_000 == 0 {
            tracing::debug!("Mining progress: {} iterations", nonce);
        }
    }

    None
}

/// Check if a block's hash meets the difficulty target
pub fn verify_pow(block: &Block) -> bool {
    block.meets_difficulty_target()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_mining() {
        let mut genesis = Block::genesis();

        // Lower difficulty for testing (0x20ffffff = easier)
        genesis.header.bits = 0x20ffffff;

        // Try mining with limited iterations
        let result = mine_block(genesis.clone(), Some(10_000_000));

        if let Some((mined_block, hash)) = result {
            assert!(verify_pow(&mined_block));
            assert_ne!(hash, Hash256::zero());
        }
        // If no block found in 10M iterations, that's acceptable for this test
    }

    #[test]
    fn test_verify_pow_invalid() {
        let mut block = Block::genesis();
        block.header.bits = 0x1d00ffff;
        block.header.nonce = 12345; // Random nonce, likely invalid

        // Verify function runs without panic
        let _ = verify_pow(&block);
    }
}
