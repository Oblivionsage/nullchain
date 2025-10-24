use colored::Colorize;
use nullchain_storage::{BlockStore, Database};
use std::path::Path;

pub fn chain_info(datadir: String) {
    let path = Path::new(&datadir);

    if !path.exists() {
        eprintln!("{}: database not found at {}", "error".red(), datadir);
        std::process::exit(1);
    }

    let db = match Database::open(path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("{}: failed to open database: {}", "error".red(), e);
            std::process::exit(1);
        }
    };

    let store = BlockStore::new(&db);

    let height = store.get_height().unwrap_or(0);
    let best_hash = store.get_best().unwrap();

    eprintln!("Chain information");
    eprintln!("  height:    {}", height);

    if let Some(hash) = best_hash {
        eprintln!("  best:      {}", format!("{}", hash).green());

        if let Ok(block) = store.get_by_hash(&hash) {
            eprintln!("  timestamp: {}", block.header.timestamp);
            eprintln!("  txs:       {}", block.transactions.len());
        }
    } else {
        eprintln!("  best:      {}", "none (empty chain)".dimmed());
    }
}

pub fn get_block(datadir: String, height: u64) {
    let path = Path::new(&datadir);

    let db = match Database::open(path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("{}: {}", "error".red(), e);
            std::process::exit(1);
        }
    };

    let store = BlockStore::new(&db);

    match store.get_by_height(height) {
        Ok(block) => {
            let json = serde_json::to_string_pretty(&block).unwrap();
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("{}: {}", "error".red(), e);
            std::process::exit(1);
        }
    }
}
