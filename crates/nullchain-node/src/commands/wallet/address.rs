use colored::Colorize;
use std::fs;

pub fn show_address(pubkey_file: String) {
    println!("{}", "Address Information".bright_cyan().bold());
    println!();
    
    match fs::read_to_string(&pubkey_file) {
        Ok(pubkey_hex) => {
            let pubkey_hex = pubkey_hex.trim();
            
            // Derive address (first 20 bytes of hash of pubkey)
            let pubkey_bytes = hex::decode(pubkey_hex)
                .expect("Invalid hex in public key file");
            
            let hash = blake3::hash(&pubkey_bytes);
            let address_bytes = &hash.as_bytes()[0..20];
            let address = hex::encode(address_bytes);
            
            println!("{}: {}", "Public Key".yellow(), pubkey_hex.green());
            println!("{}: {}", "Address".yellow(), 
                format!("null1{}", address).bright_green().bold());
            println!();
            println!("{}", "Share your address to receive NULL tokens".bright_black());
        }
        Err(e) => {
            eprintln!("{} {}", "[ERROR]".red().bold(), e);
            eprintln!("{}", "Use 'nullchain keygen' to create a keypair first".bright_black());
        }
    }
}
