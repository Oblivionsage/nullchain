use colored::Colorize;

pub fn version() {
    println!("{} {}", "NullChain".bright_cyan().bold(), "v0.1.0".yellow());
    println!();
    println!(
        "{}: {}",
        "Network".bright_black(),
        "Testnet (Development)".dimmed()
    );
    println!(
        "{}: {}",
        "Consensus".bright_black(),
        "Proof-of-Work (SHA256d)".dimmed()
    );
    println!("{}: {}", "Hash Function".bright_black(), "Blake3".dimmed());
    println!(
        "{}: {}",
        "Signature Scheme".bright_black(),
        "Ed25519".dimmed()
    );
    println!(
        "{}: {}",
        "Block Time".bright_black(),
        "600 seconds".dimmed()
    );
    println!("{}: {}", "Block Reward".bright_black(), "100 NULL".dimmed());
    println!();
    println!("{}", "github.com/Oblivionsage/nullchain".bright_black());
}
