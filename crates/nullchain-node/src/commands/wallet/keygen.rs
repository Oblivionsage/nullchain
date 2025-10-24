use nullchain_crypto::generate_keypair;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn keygen(output_dir: Option<String>) {
    println!("{}", "Generating new keypair...".bright_cyan().bold());
    println!();
    
    let (signing_key, verifying_key) = generate_keypair();
    
    // Display keys
    println!("{}: {}", "Public Key".yellow(), 
        hex::encode(verifying_key.to_bytes()).green());
    println!("{}: {}", "Private Key".yellow(), 
        hex::encode(signing_key.to_bytes()).red());
    
    println!();
    println!("{}", "[WARNING] Keep your private key secure!".red().bold());
    println!();
    
    // Save to file if requested
    if let Some(dir) = output_dir {
        let path = Path::new(&dir);
        fs::create_dir_all(path).expect("Failed to create directory");
        
        let pubkey_path = path.join("public.key");
        let privkey_path = path.join("private.key");
        
        fs::write(&pubkey_path, hex::encode(verifying_key.to_bytes()))
            .expect("Failed to write public key");
        fs::write(&privkey_path, hex::encode(signing_key.to_bytes()))
            .expect("Failed to write private key");
        
        println!("{}", "Keys saved:".bright_black());
        println!("  Public:  {}", pubkey_path.display().to_string().dimmed());
        println!("  Private: {}", privkey_path.display().to_string().dimmed());
        println!();
        println!("{}", "[NOTE] Store private.key in a secure location".bright_black());
    } else {
        println!("{}", "Use --output <dir> to save keys to file".bright_black());
    }
}
