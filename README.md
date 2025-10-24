# NullChain

**A minimal, privacy-first Layer-1 blockchain powered by zero-knowledge proofs.**

## Abstract

NullChain is a ground-up implementation of a Layer-1 blockchain designed around zero-knowledge cryptography. Built in Rust, it provides transaction privacy and computational integrity through zk-SNARKs/zk-STARKs while maintaining a minimal, auditable codebase. Unlike existing privacy chains, NullChain prioritizes simplicity and formal verification over feature bloat.

## Core Principles

- **Minimal Attack Surface**: Every line of code is a liability. Keep it simple.
- **Privacy by Default**: Users shouldn't have to opt-in to privacy.
- **Verifiable Computation**: Zero-knowledge proofs for both privacy and scalability.
- **No Premine, No ICO**: Fair launch, open development.

## Quick Start

### Installation
```bash
git clone https://github.com/Oblivionsage/nullchain.git
cd nullchain
cargo build --release
```

### Usage
```bash
# Display version and system info
./target/release/nullchain version

# Create genesis block
./target/release/nullchain genesis

# Mine a block (easy difficulty for testing)
./target/release/nullchain mine --iterations 10000000 --bits 0x1f0fffff

# Display help
./target/release/nullchain --help
```

### Example Output
```
NullChain Miner

Difficulty: 0x1f0fffff
Max Iterations: 10000000
Starting Hash: 4c5d25bfe560365a3232fd87a843fbdbb1098d02b3dcee4a63a7ce59a10c070b

Mining...

[SUCCESS] Block mined

Nonce: 596
Hash: 722af86bba5573468c1cef6009f582d38501490fd0725834e487b01eb9313e00
Time: 0.00s
Hashrate: 1900704 H/s

[VERIFIED] Proof-of-Work valid
```

## Project Status

**Current Phase:** Foundation (Phase 0 - Week 1-2)

 Completed:
- Project structure with Cargo workspace
- Core data types (Block, Transaction, Hash256, Merkle Tree)
- Cryptographic primitives (Blake3 hashing, Ed25519 signatures)
- Proof-of-Work consensus
- Difficulty adjustment algorithm
- CLI tool for mining and block inspection

 In Progress:
- CI/CD pipeline
- Comprehensive test suite
- Documentation

 Next Steps:
- Storage layer (RocksDB)
- P2P networking (libp2p)
- Transaction validation
- UTXO management

## Features

### Phase 1: Foundation
- Basic blockchain data structures (blocks, transactions, merkle trees)
- Proof-of-Work consensus (Nakamoto-style, simple)
- P2P networking (libp2p)
- CLI wallet and node interface

### Phase 2: ZK Integration
- zk-SNARK circuits for transaction privacy (sender, receiver, amount)
- Integration with Noir (Aztec) or Risc Zero
- Shielded transaction pool
- Proving and verification infrastructure

### Phase 3: ZKVM
- Full zkVM integration for arbitrary computation
- Smart contract layer with privacy guarantees
- Recursive proof composition
- Cross-chain bridges with ZK proof of consensus

## Architecture
```
nullchain/
├── crates/
│   ├── nullchain-types/       # Core data structures
│   ├── nullchain-crypto/      # Cryptographic primitives
│   ├── nullchain-consensus/   # PoW and difficulty adjustment
│   ├── nullchain-network/     # P2P networking (TODO)
│   ├── nullchain-storage/     # Blockchain persistence (TODO)
│   └── nullchain-node/        # CLI and node binary
├── Cargo.toml                 # Workspace configuration
└── README.md
```

## Technical Specifications

- **Hashing**: Blake3 (fast, secure, simple)
- **Signatures**: Ed25519 (transparent), BLS12-381 (shielded, future)
- **Consensus**: Proof-of-Work (SHA256d-style double hashing)
- **Block Time**: 600 seconds (10 minutes)
- **Block Reward**: 100 NULL (halving every 210,000 blocks)
- **Serialization**: bincode (compact binary format)

## Development

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### Project Structure

- `nullchain-types`: Core blockchain types (Block, Transaction, etc.)
- `nullchain-crypto`: Cryptographic functions (hashing, signatures)
- `nullchain-consensus`: Consensus rules (PoW mining, difficulty)
- `nullchain-node`: CLI tool and node implementation

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test --package nullchain-types

# Run specific test
cargo test test_genesis_mining
```

## Cryptographic Primitives

- **Hashing**: Blake3 (fast, secure, simple)
- **Signatures**: Ed25519 (transparent), BLS12-381 (shielded)
- **ZK Backend**: Plonky2 (fast proofs) or Halo2 (no trusted setup)
- **Commitment Scheme**: Pedersen commitments over JubJub curve
- **Merkle Trees**: Binary merkle trees with Poseidon hash

## Why Another L1?

Most existing chains are either:
1. **Bloated**: Ethereum has >1M lines of code. Attack surface is massive.
2. **Centralized**: PoS chains with 20 validators aren't decentralized.
3. **Privacy Afterthought**: Tornado Cash on Ethereum proves privacy should be native.
4. **Legacy Architecture**: Bitcoin's UTXO model is elegant but limited.

NullChain combines:
- Bitcoin's UTXO model (parallel transaction processing)
- Zcash's privacy tech (shielded transactions)
- Modern cryptography (no trusted setup, fast proofs)
- Clean codebase (<10k lines for core)

---

## Development Roadmap

[Full roadmap continues as before...]

## Contributing

This is a research project. Contributions welcome, but expect high standards:
- All code must have unit tests.
- Cryptographic code requires extra scrutiny.
- No dependencies without justification.
- Clippy must pass with zero warnings.
- Format with `cargo fmt`.

## License

MIT + Apache 2.0 (same as Rust). Use at your own risk.

## Contact

- GitHub: https://github.com/Oblivionsage/nullchain
- Author: nullprophet

---

**"Privacy is not a feature. It's a requirement."**
