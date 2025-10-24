use nullchain_types::Block;
use colored::Colorize;

pub fn genesis() {
    println!("{}", "NullChain Genesis Block".bright_cyan().bold());
    println!();
    
    let genesis = Block::genesis();
    
    println!("{}: {}", "Version".yellow(), genesis.header.version);
    println!("{}: {}", "Previous Block".yellow(), 
        format!("{}", genesis.header.previous_block).dimmed());
    println!("{}: {}", "Timestamp".yellow(), genesis.header.timestamp);
    println!("{}: {}", "Difficulty".yellow(), 
        format!("0x{:08x}", genesis.header.bits).cyan());
    println!("{}: {}", "Nonce".yellow(), genesis.header.nonce);
    println!("{}: {}", "Transactions".yellow(), genesis.transactions.len());
    
    // Calculate merkle root
    let merkle_root = genesis.calculate_merkle_root();
    println!("{}: {}", "Merkle Root".yellow(), 
        format!("{}", merkle_root).green());
    
    // Calculate hash
    let hash = genesis.hash();
    println!("{}: {}", "Block Hash".yellow(), 
        format!("{}", hash).bright_green().bold());
    
    // Serialize to JSON for saving
    let json = serde_json::to_string_pretty(&genesis).unwrap();
    println!();
    println!("{}", "JSON representation:".bright_black());
    println!("{}", json.dimmed());
}
