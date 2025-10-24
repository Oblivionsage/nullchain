use colored::Colorize;

pub fn version() {
    println!();
    println!("{} {}", 
        "NullChain".bright_cyan().bold(),
        "v0.1.0-experimental".yellow());
    println!("{}", "─".repeat(80).bright_black());
    println!();
    println!("  {:<20} {}", "Network:".dimmed(), "Testnet".bright_black());
    println!("  {:<20} {}", "Consensus:".dimmed(), "Proof-of-Work".bright_black());
    println!("  {:<20} {}", "Hash:".dimmed(), "Blake3".bright_black());
    println!("  {:<20} {}", "Signature:".dimmed(), "Ed25519".bright_black());
    println!("  {:<20} {}", "Block Time:".dimmed(), "600s".bright_black());
    println!();
    println!("  {}", "github.com/Oblivionsage/nullchain".dimmed());
    println!();
}
