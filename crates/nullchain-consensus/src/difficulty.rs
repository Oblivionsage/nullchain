//! Difficulty adjustment algorithm

use nullchain_types::constants;

/// Adjust difficulty based on actual vs target time
/// Returns new bits value
pub fn adjust_difficulty(
    old_bits: u32,
    actual_time: u64,
    target_time: u64,
) -> u32 {
    // Prevent extreme adjustments (max 4x change)
    let actual_clamped = actual_time.clamp(target_time / 4, target_time * 4);
    
    // Calculate adjustment factor
    let adjustment = actual_clamped as f64 / target_time as f64;
    
    // Adjust difficulty (inverse relationship)
    // If blocks too fast (actual < target), increase difficulty (lower bits)
    // If blocks too slow (actual > target), decrease difficulty (higher bits)
    
    let exponent = (old_bits >> 24) as f64;
    let mantissa = (old_bits & 0x00ffffff) as f64;
    
    let new_mantissa = (mantissa * adjustment).min(0x00ffffff as f64) as u32;
    
    ((exponent as u32) << 24) | (new_mantissa & 0x00ffffff)
}

/// Calculate target time for difficulty adjustment period
pub fn calculate_target_time(num_blocks: u64) -> u64 {
    num_blocks * constants::BLOCK_TIME_SECONDS
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_difficulty_increase_when_fast() {
        let old_bits = 0x1d00ffff;
        let target_time = constants::BLOCK_TIME_SECONDS * constants::DIFFICULTY_ADJUSTMENT_INTERVAL;
        let actual_time = target_time / 2; // Blocks too fast
        
        let new_bits = adjust_difficulty(old_bits, actual_time, target_time);
        
        // Difficulty should increase (bits should decrease or stay same)
        assert!(new_bits <= old_bits);
    }
    
    #[test]
    fn test_difficulty_decrease_when_slow() {
        let old_bits = 0x1d00ffff;
        let target_time = constants::BLOCK_TIME_SECONDS * constants::DIFFICULTY_ADJUSTMENT_INTERVAL;
        let actual_time = target_time * 2; // Blocks too slow
        
        let new_bits = adjust_difficulty(old_bits, actual_time, target_time);
        
        // Difficulty should decrease (bits should increase or stay same)
        assert!(new_bits >= old_bits);
    }
    
    #[test]
    fn test_calculate_target_time() {
        let target = calculate_target_time(constants::DIFFICULTY_ADJUSTMENT_INTERVAL);
        
        // Should be 2016 blocks * 600 seconds
        assert_eq!(target, 2016 * 600);
    }
}
