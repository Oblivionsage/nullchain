# NullChain

[![CI](https://github.com/Oblivionsage/nullchain/workflows/CI/badge.svg)](https://github.com/Oblivionsage/nullchain/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**A minimal, privacy-first Layer-1 blockchain powered by zero-knowledge proofs.**
```
     ███╗   ██╗██╗   ██╗██╗     ██╗          ██████╗██╗  ██╗ █████╗ ██╗███╗   ██╗
     ████╗  ██║██║   ██║██║     ██║         ██╔════╝██║  ██║██╔══██╗██║████╗  ██║
     ██╔██╗ ██║██║   ██║██║     ██║         ██║     ███████║███████║██║██╔██╗ ██║
     ██║╚██╗██║██║   ██║██║     ██║         ██║     ██╔══██║██╔══██║██║██║╚██╗██║
     ██║ ╚████║╚██████╔╝███████╗███████╗    ╚██████╗██║  ██║██║  ██║██║██║ ╚████║
     ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝╚══════╝     ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝
                                                                                    
            Privacy is not a feature. It's a fundamental human right.
```

NullChain is built on cypherpunk principles:

- **Privacy by default**: No opt-in required
- **Minimal trusted code**: <10k lines of auditable Rust
- **Cryptographic soundness**: Well-studied primitives only
- **Decentralization**: No premine, no ICO, no governance tokens

## Security First

### Cryptographic Guarantees

- **Hashing**: Blake3 (fast, secure, simple)
- **Signatures**: Ed25519 (collision-resistant, deterministic)
- **Key Derivation**: Constant-time operations
- **Memory Safety**: Sensitive data zeroized on drop
- **RNG**: OS-provided CSPRNG (OsRng)

### Privacy Features

- **Shielded Transactions**: Coming with zk-SNARKs integration
- **Encrypted Storage**: Private keys encrypted at rest
- **No Metadata Leaks**: Minimal blockchain analysis surface
- **Decoy Transactions**: Network-level privacy (planned)

## Quick Start

### Installation
```bash
git clone https://github.com/Oblivionsage/nullchain.git
cd nullchain
cargo build --release
```

### Usage

#### Generate Keypair (Secure)
```bash
# Generate keypair (not saved to disk - maximum security)
./target/release/nullchain keygen

# Save encrypted keypair (requires passphrase)
./target/release/nullchain keygen --output ~/.nullchain
```

**Security Notes:**

- Private keys are encrypted with your passphrase
- Passphrase never stored (keep it secure!)
- Use air-gapped machine for maximum security

#### Derive Address

```bash
./target/release/nullchain address --pubkey ~/.nullchain/public.key
```

#### Mine Block

```bash
# Easy difficulty (testing)
./target/release/nullchain mine --iterations 10000000 --bits 0x1f0fffff

# Genesis block
./target/release/nullchain genesis
```

## Architecture

```
nullchain/
├── crates/
│   ├── nullchain-types/       # Core data structures
│   ├── nullchain-crypto/      # Ed25519, Blake3, zeroize
│   ├── nullchain-consensus/   # PoW (transitioning to PoS+ZK)
│   ├── nullchain-network/     # P2P (libp2p)
│   ├── nullchain-storage/     # Blockchain persistence
│   └── nullchain-node/        # CLI and node
```

## Roadmap

**Phase 0: Foundation** 
- Proof-of-Work consensus
- Basic blockchain structures
- CLI wallet

**Phase 1: Privacy Layer** 
- zk-SNARK transaction privacy
- Shielded transaction pool
- Ring signatures (Monero-style)

**Phase 2: zkVM** 
- Smart contracts with privacy
- Recursive proof composition
- Cross-chain bridges

## Comparison

| Feature | NullChain | Monero | Zcash | Bitcoin |
|---------|-----------|--------|-------|---------|
| Privacy | Default | Default | Optional | None |
| Trusted Setup | No | No | Yes | N/A |
| Codebase Size | <10k | ~100k | ~200k | ~100k |
| ZK Proofs | Planned | No | Yes | No |
| Smart Contracts | Planned | No | No | Limited |

## Security

**Threat Model:**
- Global passive adversary
- Active network attackers
- Malicious nodes (up to 33%)
- Quantum computers (future-proofing)

**See [SECURITY.md](SECURITY.md) for vulnerability reporting.**

## Development

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Dual-licensed under MIT + Apache 2.0

## Contact

- **GitHub**: https://github.com/Oblivionsage/nullchain
- **Team**: NullChain Development Team
- **Security**: Use GitHub Security Advisories

---

