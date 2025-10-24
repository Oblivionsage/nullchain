use nullchain_types::Block;
use colored::Colorize;

pub fn genesis() {
    let genesis = Block::genesis();
    let hash = genesis.hash();
    
    eprintln!("Genesis block");
    eprintln!("  version:     {}", genesis.header.version);
    eprintln!("  timestamp:   {}", genesis.header.timestamp);
    eprintln!("  difficulty:  {}", format!("0x{:08x}", genesis.header.bits).cyan());
    eprintln!("  hash:        {}", format!("{}", hash).green());
    
    let json = serde_json::to_string(&genesis).unwrap();
    println!("{}", json);
}
