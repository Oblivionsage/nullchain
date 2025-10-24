use colored::Colorize;
use nullchain_crypto::generate_keypair;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn keygen(output_dir: Option<String>) {
    let (signing_key, verifying_key) = generate_keypair();
    let pubkey_hex = hex::encode(verifying_key.to_bytes());

    eprintln!("Generated new keypair");
    println!("{}", pubkey_hex.green());

    if let Some(dir) = output_dir {
        eprint!("Enter passphrase: ");
        io::stderr().flush().unwrap();

        let mut passphrase = String::new();
        io::stdin()
            .read_line(&mut passphrase)
            .expect("Failed to read passphrase");
        let passphrase = passphrase.trim();

        if passphrase.is_empty() {
            eprintln!("{}", "error: empty passphrase".red());
            std::process::exit(1);
        }

        if passphrase.len() < 8 {
            eprintln!(
                "{}",
                "warning: weak passphrase (minimum 8 characters recommended)".yellow()
            );
        }

        let privkey_bytes = signing_key.to_bytes();
        let key_hash = blake3::hash(passphrase.as_bytes());
        let mut encrypted = privkey_bytes.to_vec();
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= key_hash.as_bytes()[i % 32];
        }

        let path = Path::new(&dir);
        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("{}: {}", "error".red(), e);
            std::process::exit(1);
        }

        let pubkey_path = path.join("key.pub");
        let privkey_path = path.join("key.enc");

        if let Err(e) = fs::write(&pubkey_path, &pubkey_hex) {
            eprintln!("{}: failed to write public key: {}", "error".red(), e);
            std::process::exit(1);
        }

        if let Err(e) = fs::write(&privkey_path, hex::encode(&encrypted)) {
            eprintln!("{}: failed to write private key: {}", "error".red(), e);
            std::process::exit(1);
        }

        eprintln!("Saved to {}", path.display().to_string().cyan());
        eprintln!("  public:  {}", pubkey_path.display());
        eprintln!("  private: {} (encrypted)", privkey_path.display());
    }
}
