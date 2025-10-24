use clap::{Parser, Subcommand};

mod commands;

fn parse_hex_or_decimal(s: &str) -> Result<u32, String> {
    if let Some(hex_str) = s.strip_prefix("0x") {
        u32::from_str_radix(hex_str, 16)
            .map_err(|e| format!("Invalid hex: {}", e))
    } else {
        s.parse::<u32>()
            .map_err(|e| format!("Invalid number: {}", e))
    }
}

#[derive(Parser)]
#[command(name = "nullchain")]
#[command(version = "0.1.0")]
#[command(about = "NullChain - Privacy-first Layer-1 blockchain", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and display the genesis block
    Genesis,
    
    /// Mine a new block
    Mine {
        /// Maximum mining iterations (default: unlimited)
        #[arg(short, long)]
        iterations: Option<u64>,
        
        /// Difficulty bits (hex: 0x21ffffff or decimal: 553648127)
        #[arg(short, long, default_value = "0x24ffffff", value_parser = parse_hex_or_decimal)]
        bits: u32,
    },
    
    /// Display information about a block
    Info {
        /// Block data in JSON format
        #[arg(short, long)]
        json: String,
    },
    
    /// Display version and system information
    Version,
}

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Genesis => commands::genesis(),
        Commands::Mine { iterations, bits } => commands::mine(iterations, bits),
        Commands::Info { json } => commands::info(&json),
        Commands::Version => commands::version(),
    }
}
