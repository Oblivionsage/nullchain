use nullchain_types::Block;
use colored::Colorize;

pub fn info(json: &str) {
    match serde_json::from_str::<Block>(json) {
        Ok(block) => {
            let hash = block.hash();
            let valid = block.meets_difficulty_target();
            
            println!();
            println!("{}", "BLOCK INFO".bright_cyan().bold());
            println!("{}", "─".repeat(80).bright_black());
            println!();
            println!("  {:<20} {}", "Version:".dimmed(), block.header.version);
            println!("  {:<20} {}", "Timestamp:".dimmed(), block.header.timestamp);
            println!("  {:<20} {}", "Difficulty:".dimmed(), 
                format!("0x{:08x}", block.header.bits).cyan());
            println!("  {:<20} {}", "Nonce:".dimmed(), 
                format!("{}", block.header.nonce).bright_white());
            println!("  {:<20} {}", "Transactions:".dimmed(), block.transactions.len());
            println!();
            println!("  {}", "Previous Block:".dimmed());
            println!("    {}", format!("{}", block.header.previous_block).dimmed());
            println!();
            println!("  {}", "Merkle Root:".dimmed());
            println!("    {}", format!("{}", block.header.merkle_root).green());
            println!();
            println!("  {}", "Block Hash:".dimmed());
            println!("    {}", format!("{}", hash).bright_green().bold());
            println!();
            println!("  {:<20} {}", "PoW Status:".dimmed(), 
                if valid { "[VALID]".green().bold() } else { "[INVALID]".red().bold() });
            println!();
            println!("{}", "─".repeat(80).bright_black());
            println!();
        }
        Err(e) => {
            eprintln!();
            eprintln!("{} {}", "[ERROR]".red().bold(), e);
            eprintln!();
            std::process::exit(1);
        }
    }
}
