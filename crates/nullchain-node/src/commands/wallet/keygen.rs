use nullchain_crypto::generate_keypair;
use colored::Colorize;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

pub fn keygen(output_dir: Option<String>) {
    println!("{}", "=".repeat(60).bright_black());
    println!("{}", "  NullChain Keypair Generation".bright_cyan().bold());
    println!("{}", "=".repeat(60).bright_black());
    println!();
    
    let (signing_key, verifying_key) = generate_keypair();
    
    // Display public key only
    let pubkey_hex = hex::encode(verifying_key.to_bytes());
    println!("{}", "Public Key (safe to share):".yellow());
    println!("  {}", pubkey_hex.green());
    println!();
    
    // Warn about private key
    println!("{}", "⚠  PRIVATE KEY GENERATED  ⚠".red().bold());
    println!();
    
    // Save to file if requested
    if let Some(dir) = output_dir {
        let path = Path::new(&dir);
        fs::create_dir_all(path).expect("Failed to create directory");
        
        // Ask for passphrase
        println!("{}", "Enter passphrase to encrypt private key:".yellow());
        print!("  > ");
        io::stdout().flush().unwrap();
        
        let mut passphrase = String::new();
        io::stdin().read_line(&mut passphrase).expect("Failed to read passphrase");
        let passphrase = passphrase.trim();
        
        if passphrase.is_empty() {
            eprintln!();
            eprintln!("{}", "[ERROR] Passphrase cannot be empty".red().bold());
            eprintln!("{}", "Keys NOT saved (too dangerous)".bright_black());
            return;
        }
        
        // Simple encryption: XOR with passphrase hash (NOTE: Use proper encryption in production)
        let privkey_bytes = signing_key.to_bytes();
        let key_hash = blake3::hash(passphrase.as_bytes());
        let mut encrypted = privkey_bytes.to_vec();
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= key_hash.as_bytes()[i % 32];
        }
        
        let pubkey_path = path.join("public.key");
        let privkey_path = path.join("private.key.enc");
        
        fs::write(&pubkey_path, &pubkey_hex)
            .expect("Failed to write public key");
        fs::write(&privkey_path, hex::encode(&encrypted))
            .expect("Failed to write encrypted private key");
        
        println!();
        println!("{}", "Keys saved (private key encrypted):".green());
        println!("  {}: {}", "Public".bright_black(), pubkey_path.display().to_string().dimmed());
        println!("  {}: {}", "Private".bright_black(), privkey_path.display().to_string().dimmed());
        println!();
        println!("{}", "─".repeat(60).bright_black());
        println!("{}", "SECURITY NOTES:".yellow().bold());
        println!("{}", "  • Private key is encrypted with your passphrase".bright_black());
        println!("{}", "  • Store passphrase in secure location (NOT in file)".bright_black());
        println!("{}", "  • Backup both files to offline storage".bright_black());
        println!("{}", "  • Never share private key or passphrase".bright_black());
        println!("{}", "─".repeat(60).bright_black());
    } else {
        println!("{}", "Private key NOT saved to disk (safer)".green());
        println!("{}", "Use --output <dir> to save encrypted keys".bright_black());
        println!();
        println!("{}", "For maximum security:".bright_black());
        println!("  {}", "1. Generate keys on air-gapped machine".dimmed());
        println!("  {}", "2. Write private key on paper (physical backup)".dimmed());
        println!("  {}", "3. Never connect private key to network".dimmed());
    }
    
    println!();
}
