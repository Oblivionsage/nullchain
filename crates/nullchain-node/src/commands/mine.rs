use nullchain_types::Block;
use nullchain_consensus::mine_block;
use std::time::Instant;
use colored::Colorize;

pub fn mine(iterations: Option<u64>, bits: u32) {
    println!("{}", "NullChain Miner".bright_cyan().bold());
    println!();
    
    let mut block = Block::genesis();
    block.header.bits = bits;
    
    // Update merkle root
    block.header.merkle_root = block.calculate_merkle_root();
    
    println!("{}: {}", "Difficulty".yellow(), 
        format!("0x{:08x}", bits).cyan());
    
    if let Some(max) = iterations {
        println!("{}: {}", "Max Iterations".yellow(), 
            format!("{}", max).bright_black());
    } else {
        println!("{}: {}", "Max Iterations".yellow(), "unlimited".bright_black());
    }
    
    println!("{}: {}", "Starting Hash".yellow(), 
        format!("{}", block.hash()).dimmed());
    println!();
    println!("{}", "Mining...".bright_black());
    
    let start = Instant::now();
    
    match mine_block(block, iterations) {
        Some((mined_block, hash)) => {
            let elapsed = start.elapsed();
            
            println!();
            println!("{}", "[SUCCESS] Block mined".bright_green().bold());
            println!();
            println!("{}: {}", "Nonce".yellow(), 
                format!("{}", mined_block.header.nonce).bright_white().bold());
            println!("{}: {}", "Hash".yellow(), 
                format!("{}", hash).bright_green().bold());
            println!("{}: {}", "Time".yellow(), 
                format!("{:.2}s", elapsed.as_secs_f64()).bright_black());
            
            let hashrate = mined_block.header.nonce as f64 / elapsed.as_secs_f64();
            println!("{}: {}", "Hashrate".yellow(), 
                format!("{:.0} H/s", hashrate).cyan());
            
            // Verify
            if mined_block.meets_difficulty_target() {
                println!();
                println!("{}", "[VERIFIED] Proof-of-Work valid".green());
            }
            
            // JSON output
            let json = serde_json::to_string_pretty(&mined_block).unwrap();
            println!();
            println!("{}", "Block JSON:".bright_black());
            println!("{}", json.dimmed());
        }
        None => {
            let elapsed = start.elapsed();
            println!();
            println!("{}", "[FAILED] No valid nonce found".red().bold());
            println!("{}: {}", "Time".yellow(), 
                format!("{:.2}s", elapsed.as_secs_f64()).bright_black());
            println!("{}", "Try increasing iterations or decreasing difficulty".bright_black());
        }
    }
}
