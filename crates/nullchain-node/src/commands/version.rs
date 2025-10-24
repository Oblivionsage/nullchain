pub fn version() {
    println!("nullchain v{}", env!("CARGO_PKG_VERSION"));
    println!("Network: testnet");
    println!("Consensus: proof-of-work");
}
