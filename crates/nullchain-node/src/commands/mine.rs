use nullchain_types::Block;
use nullchain_consensus::mine_block;
use nullchain_storage::{Database, BlockStore};
use colored::Colorize;
use std::time::Instant;
use std::path::Path;

pub fn mine(iterations: Option<u64>, bits: u32, datadir: Option<String>) {
    let mut block = Block::genesis();
    
    // Check if we should continue from existing chain
    if let Some(ref dir) = datadir {
        let path = Path::new(dir);
        if path.exists() {
            match Database::open(path) {
                Ok(db) => {
                    let store = BlockStore::new(&db);
                    if let Ok(height) = store.get_height() {
                        if height > 0 {
                            eprintln!("Continuing from height {}", height);
                            if let Ok(last_block) = store.get_by_height(height) {
                                block = Block::genesis();
                                block.header.previous_block = last_block.hash();
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}: failed to open database: {}", "warning".yellow(), e);
                }
            }
        }
    }
    
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
            
            // Save to database
            if let Some(dir) = datadir {
                let path = Path::new(&dir);
                match Database::open(path) {
                    Ok(db) => {
                        let store = BlockStore::new(&db);
                        let height = store.get_height().unwrap_or(0);
                        
                        if let Err(e) = store.put(height, &mined_block) {
                            eprintln!("{}: failed to save block: {}", "error".red(), e);
                        } else if let Err(e) = store.set_best(height, &hash) {
                            eprintln!("{}: failed to update chain tip: {}", "error".red(), e);
                        } else {
                            eprintln!("Saved to database (height {})", height);
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: failed to open database: {}", "error".red(), e);
                    }
                }
            }
            
            let json = serde_json::to_string(&mined_block).unwrap();
            println!("{}", json);
        }
        None => {
            eprintln!("{}", "Mining failed: no solution found".red());
            std::process::exit(1);
        }
    }
}
