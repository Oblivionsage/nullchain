use nullchain_types::Block;
use colored::Colorize;

pub fn info(json: &str) {
    println!("{}", "Block Information".bright_cyan().bold());
    println!();
    
    match serde_json::from_str::<Block>(json) {
        Ok(block) => {
            println!("{}: {}", "Version".yellow(), block.header.version);
            println!("{}: {}", "Previous Block".yellow(), 
                format!("{}", block.header.previous_block).dimmed());
            println!("{}: {}", "Merkle Root".yellow(), 
                format!("{}", block.header.merkle_root).green());
            println!("{}: {}", "Timestamp".yellow(), block.header.timestamp);
            println!("{}: {}", "Difficulty".yellow(), 
                format!("0x{:08x}", block.header.bits).cyan());
            println!("{}: {}", "Nonce".yellow(), 
                format!("{}", block.header.nonce).bright_white());
            println!("{}: {}", "Transactions".yellow(), block.transactions.len());
            
            let hash = block.hash();
            println!("{}: {}", "Block Hash".yellow(), 
                format!("{}", hash).bright_green().bold());
            
            println!();
            if block.meets_difficulty_target() {
                println!("{}", "[VALID] Proof-of-Work".green().bold());
            } else {
                println!("{}", "[INVALID] Proof-of-Work".red().bold());
            }
        }
        Err(e) => {
            eprintln!("{} {}", "[ERROR]".red().bold(), e);
            std::process::exit(1);
        }
    }
}
