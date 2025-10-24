use nullchain_types::Block;
use nullchain_consensus::mine_block;
use std::time::Instant;
use colored::Colorize;

pub fn mine(iterations: Option<u64>, bits: u32) {
    let mut block = Block::genesis();
    block.header.bits = bits;
    block.header.merkle_root = block.calculate_merkle_root();
    
    println!();
    println!("{}", "MINER".bright_cyan().bold());
    println!("{}", "─".repeat(80).bright_black());
    println!();
    println!("  {:<20} {}", "Difficulty:".dimmed(), format!("0x{:08x}", bits).cyan());
    
    if let Some(max) = iterations {
        println!("  {:<20} {}", "Max Iterations:".dimmed(), format!("{}", max).bright_black());
    } else {
        println!("  {:<20} {}", "Max Iterations:".dimmed(), "unlimited".bright_black());
    }
    
    let starting_hash = block.hash();
    println!();
    println!("  {}", "Starting Hash:".dimmed());
    println!("    {}", format!("{}", starting_hash).dimmed());
    println!();
    println!("  {}", "Mining...".bright_black());
    
    let start = Instant::now();
    
    match mine_block(block, iterations) {
        Some((mined_block, hash)) => {
            let elapsed = start.elapsed();
            let hashrate = mined_block.header.nonce as f64 / elapsed.as_secs_f64();
            
            println!();
            println!("  {} {}", ">>".green().bold(), "Block found".green());
            println!();
            println!("  {:<20} {}", "Nonce:".dimmed(), 
                format!("{}", mined_block.header.nonce).bright_white().bold());
            println!();
            println!("  {}", "Hash:".dimmed());
            println!("    {}", format!("{}", hash).bright_green().bold());
            println!();
            println!("  {:<20} {}", "Time:".dimmed(), 
                format!("{:.3}s", elapsed.as_secs_f64()).bright_black());
            println!("  {:<20} {}", "Hashrate:".dimmed(), 
                format!("{:.0} H/s", hashrate).cyan());
            println!();
            
            if mined_block.meets_difficulty_target() {
                println!("  {}", "[PoW VERIFIED]".green().bold());
                println!();
            }
            
            println!("{}", "─".repeat(80).bright_black());
            println!();
            
            let json = serde_json::to_string_pretty(&mined_block).unwrap();
            println!("{}", "[JSON]".bright_black());
            println!("{}", json.dimmed());
            println!();
        }
        None => {
            let elapsed = start.elapsed();
            println!();
            println!("  {} {}", ">>".red().bold(), "No valid nonce found".red());
            println!();
            println!("  {:<20} {}", "Time:".dimmed(), 
                format!("{:.3}s", elapsed.as_secs_f64()).bright_black());
            println!();
            println!("  {}", "Try: increase iterations or lower difficulty".bright_black());
            println!();
            println!("{}", "─".repeat(80).bright_black());
            println!();
        }
    }
}
