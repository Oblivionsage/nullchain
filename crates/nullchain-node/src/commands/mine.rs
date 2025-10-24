use nullchain_types::Block;
use nullchain_consensus::mine_block;
use colored::Colorize;
use std::time::Instant;

pub fn mine(iterations: Option<u64>, bits: u32) {
    let mut block = Block::genesis();
    block.header.bits = bits;
    block.header.merkle_root = block.calculate_merkle_root();
    
    eprintln!("Mining block");
    eprintln!("  difficulty: {}", format!("0x{:08x}", bits).cyan());
    if let Some(max) = iterations {
        eprintln!("  max_iter:   {}", max);
    }
    
    let start = Instant::now();
    
    match mine_block(block, iterations) {
        Some((mined_block, hash)) => {
            let elapsed = start.elapsed();
            let hashrate = mined_block.header.nonce as f64 / elapsed.as_secs_f64();
            
            eprintln!("{}", "Block mined".green());
            eprintln!("  nonce:    {}", mined_block.header.nonce);
            eprintln!("  hash:     {}", format!("{}", hash).green());
            eprintln!("  time:     {:.3}s", elapsed.as_secs_f64());
            eprintln!("  hashrate: {:.0} H/s", hashrate);
            
            let json = serde_json::to_string(&mined_block).unwrap();
            println!("{}", json);
        }
        None => {
            eprintln!("{}", "Mining failed: no solution found".red());
            std::process::exit(1);
        }
    }
}
