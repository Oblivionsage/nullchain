use clap::{Parser, Subcommand};

mod commands;

fn parse_hex_or_decimal(s: &str) -> Result<u32, String> {
    if let Some(hex_str) = s.strip_prefix("0x") {
        u32::from_str_radix(hex_str, 16).map_err(|e| format!("invalid hex: {}", e))
    } else {
        s.parse::<u32>().map_err(|e| format!("invalid number: {}", e))
    }
}

#[derive(Parser)]
#[command(name = "nullchain")]
#[command(version)]
#[command(about = "Privacy-first blockchain node", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate new keypair
    Keygen {
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Derive address from public key
    Address {
        #[arg(short, long)]
        pubkey: String,
    },
    
    /// Mine new block
    Mine {
        #[arg(short, long)]
        iterations: Option<u64>,
        
        #[arg(short, long, default_value = "0x1f0fffff", value_parser = parse_hex_or_decimal)]
        bits: u32,
    },
    
    /// Display genesis block
    Genesis,
    
    /// Show block information
    Info {
        #[arg(short, long)]
        json: String,
    },
    
    /// Show version information
    Version,
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { output } => commands::wallet::keygen(output),
        Commands::Address { pubkey } => commands::wallet::show_address(pubkey),
        Commands::Mine { iterations, bits } => commands::mine(iterations, bits),
        Commands::Genesis => commands::genesis(),
        Commands::Info { json } => commands::info(&json),
        Commands::Version => commands::version(),
    }
}
