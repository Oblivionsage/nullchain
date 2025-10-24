use nullchain_types::Block;
use colored::Colorize;

pub fn genesis() {
    let genesis = Block::genesis();
    let merkle_root = genesis.calculate_merkle_root();
    let hash = genesis.hash();
    
    println!();
    println!("{}", "GENESIS BLOCK".bright_cyan().bold());
    println!("{}", "─".repeat(80).bright_black());
    println!();
    println!("  {:<20} {}", "Version:".dimmed(), genesis.header.version);
    println!("  {:<20} {}", "Timestamp:".dimmed(), genesis.header.timestamp);
    println!("  {:<20} {}", "Difficulty:".dimmed(), format!("0x{:08x}", genesis.header.bits).cyan());
    println!("  {:<20} {}", "Nonce:".dimmed(), genesis.header.nonce);
    println!("  {:<20} {}", "Transactions:".dimmed(), genesis.transactions.len());
    println!();
    println!("  {}", "Merkle Root:".dimmed());
    println!("    {}", format!("{}", merkle_root).green());
    println!();
    println!("  {}", "Block Hash:".dimmed());
    println!("    {}", format!("{}", hash).bright_green().bold());
    println!();
    println!("{}", "─".repeat(80).bright_black());
    
    // Compact JSON
    let json = serde_json::to_string(&genesis).unwrap();
    println!();
    println!("{}", "[JSON]".bright_black());
    println!("{}", json.dimmed());
    println!();
}
