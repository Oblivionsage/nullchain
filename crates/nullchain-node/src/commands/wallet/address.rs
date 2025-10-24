use colored::Colorize;
use std::fs;

pub fn show_address(pubkey_file: String) {
    let pubkey_hex = match fs::read_to_string(&pubkey_file) {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("{}: {}", "error".red(), e);
            std::process::exit(1);
        }
    };

    let pubkey_bytes = match hex::decode(&pubkey_hex) {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("{}", "error: invalid hex in public key file".red());
            std::process::exit(1);
        }
    };

    let hash = blake3::hash(&pubkey_bytes);
    let address_bytes = &hash.as_bytes()[0..20];
    let address = hex::encode(address_bytes);

    println!("{}", format!("null1{}", address).cyan());
}
