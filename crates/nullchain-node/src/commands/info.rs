use colored::Colorize;
use nullchain_types::Block;

pub fn info(json: &str) {
    let block: Block = match serde_json::from_str(json) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}: invalid block data: {}", "error".red(), e);
            std::process::exit(1);
        }
    };

    let hash = block.hash();
    let valid = block.meets_difficulty_target();

    eprintln!("Block information");
    eprintln!("  version:    {}", block.header.version);
    eprintln!("  timestamp:  {}", block.header.timestamp);
    eprintln!(
        "  difficulty: {}",
        format!("0x{:08x}", block.header.bits).cyan()
    );
    eprintln!("  nonce:      {}", block.header.nonce);
    eprintln!("  txs:        {}", block.transactions.len());
    eprintln!("  hash:       {}", format!("{}", hash).green());
    eprintln!(
        "  valid:      {}",
        if valid { "true".green() } else { "false".red() }
    );
}
