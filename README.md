# NullChain

**A minimal, privacy-first Layer-1 blockchain powered by zero-knowledge proofs.**

## Abstract

NullChain is a ground-up implementation of a Layer-1 blockchain designed around zero-knowledge cryptography. Built in Rust, it provides transaction privacy and computational integrity through zk-SNARKs/zk-STARKs while maintaining a minimal, auditable codebase. Unlike existing privacy chains, NullChain prioritizes simplicity and formal verification over feature bloat.

## Core Principles

- **Minimal Attack Surface**: Every line of code is a liability. Keep it simple.
- **Privacy by Default**: Users shouldn't have to opt-in to privacy.
- **Verifiable Computation**: Zero-knowledge proofs for both privacy and scalability.
- **No Premine, No ICO**: Fair launch, open development.

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

## Technical Architecture

```
┌─────────────────────────────────────────┐
│         Application Layer               │
│  (Wallets, Explorers, dApps)           │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         RPC/API Layer                   │
│  (JSON-RPC, WebSocket)                 │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│      Transaction Pool (Mempool)        │
│  ┌──────────────┐  ┌─────────────┐   │
│  │ Transparent  │  │  Shielded   │   │
│  │      Txs     │  │     Txs     │   │
│  └──────────────┘  └─────────────┘   │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│       Consensus Layer (PoW)            │
│  - Block validation                     │
│  - Chain selection                      │
│  - Difficulty adjustment                │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         State Machine                   │
│  ┌──────────────┐  ┌─────────────┐   │
│  │   UTXO Set   │  │ Note Tree   │   │
│  │  (Unshielded)│  │ (Shielded)  │   │
│  └──────────────┘  └─────────────┘   │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         ZK Proof System                 │
│  - Circuit compilation                  │
│  - Proof generation                     │
│  - Verification                         │
│  - Trusted setup (if needed)           │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         Storage Layer                   │
│  - RocksDB (blockchain data)           │
│  - Proof archive                        │
└─────────────────────────────────────────┘
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

### Phase 0: Project Setup (Week 1-2)
**Goal**: Establish development infrastructure and core data structures.

#### Milestone 0.1: Repository Structure
- [x] Initialize Cargo workspace
- [ ] Set up module structure (blockchain, crypto, network, consensus, storage)
- [ ] Configure CI/CD (GitHub Actions: clippy, test, fmt)
- [ ] Add fuzzing infrastructure (cargo-fuzz)
- [ ] Security audit checklist template

#### Milestone 0.2: Core Data Structures
- [ ] Block structure (header, transactions, proofs)
- [ ] Transaction types (Transparent, Shielded, Shield, Unshield)
- [ ] Merkle tree implementation with caching
- [ ] Serialization (bincode/borsh for efficiency)
- [ ] Unit tests for all structures

#### Milestone 0.3: Cryptographic Foundations
- [ ] Key generation (Ed25519 for transparent, viewing/spending keys for shielded)
- [ ] Address encoding (bech32-style with version prefix)
- [ ] Signature verification
- [ ] Hash functions abstraction layer
- [ ] Constant-time operations where needed

---

### Phase 1: Basic Blockchain (Week 3-6)
**Goal**: Working blockchain without ZK, transparent transactions only.

#### Milestone 1.1: Consensus Engine
- [ ] Proof-of-Work implementation (SHA256d like Bitcoin)
- [ ] Difficulty adjustment algorithm (every 2016 blocks)
- [ ] Block validation rules
- [ ] Orphan block handling
- [ ] Chain reorganization logic

#### Milestone 1.2: Transaction Pool (Mempool)
- [ ] Transaction validation (signatures, double-spend check)
- [ ] Fee market (simple fee-per-byte)
- [ ] Transaction replacement (RBF-like)
- [ ] Mempool eviction policy
- [ ] DoS protection

#### Milestone 1.3: State Management
- [ ] UTXO set database (RocksDB backend)
- [ ] Efficient UTXO lookups (B-tree index)
- [ ] Pruning old blocks (keep UTXO set only)
- [ ] State root commitments
- [ ] Checkpoint system for fast sync

#### Milestone 1.4: P2P Networking
- [ ] libp2p integration (gossipsub for block/tx propagation)
- [ ] Peer discovery (mDNS, bootstrap nodes)
- [ ] Block sync protocol (headers-first)
- [ ] Transaction relay protocol
- [ ] Eclipse attack mitigation

#### Milestone 1.5: Node Implementation
- [ ] Full node (validates everything)
- [ ] Light node (SPV-style verification)
- [ ] Mining node (block production)
- [ ] Configuration system (TOML config file)
- [ ] Logging and metrics (tracing, Prometheus)

---

### Phase 2: ZK Privacy Layer (Week 7-12)
**Goal**: Add shielded transactions with zero-knowledge proofs.

#### Milestone 2.1: ZK Framework Selection
- [ ] Evaluate Plonky2 vs Halo2 vs Noir
- [ ] Benchmark proof generation time
- [ ] Benchmark proof size
- [ ] Decide on trusted setup strategy (or avoid it)
- [ ] Create proving/verification key generation tool

#### Milestone 2.2: Shielded Transaction Circuit
- [ ] Design circuit for: hide sender, receiver, amount
- [ ] Nullifier computation (prevent double-spend)
- [ ] Commitment tree membership proof
- [ ] Range proof (amount is positive, no overflow)
- [ ] Circuit optimization (reduce constraints)

#### Milestone 2.3: Note Commitment Tree
- [ ] Incremental Merkle tree for note commitments
- [ ] Sparse Merkle tree for nullifiers
- [ ] Witness generation for circuit
- [ ] Tree root tracking in block headers
- [ ] Efficient tree updates

#### Milestone 2.4: Shielded Transaction Types
- [ ] **Mint**: Transparent → Shielded (shield coins)
- [ ] **Transfer**: Shielded → Shielded (private transfer)
- [ ] **Burn**: Shielded → Transparent (unshield coins)
- [ ] Multi-input/output shielded transactions
- [ ] Transaction builder library

#### Milestone 2.5: Proving Infrastructure
- [ ] Proof generation (client-side, ~2-10 seconds target)
- [ ] Proof verification (node-side, <10ms target)
- [ ] Proof batching (verify multiple proofs together)
- [ ] Hardware acceleration (GPU proving support)
- [ ] Proof caching system

#### Milestone 2.6: Privacy Guarantees
- [ ] Anonymity set analysis
- [ ] Timing attack mitigation (decoy transactions)
- [ ] Amount privacy (no value leakage)
- [ ] Network-level privacy (Tor/I2P integration)
- [ ] Formal security audit prep

---

### Phase 3: zkVM Integration (Week 13-20)
**Goal**: Enable arbitrary computation with privacy.

#### Milestone 3.1: zkVM Selection & Integration
- [ ] Evaluate: Risc Zero, SP1, zkMIPS, Jolt
- [ ] Integrate chosen zkVM
- [ ] Build toolchain for compiling Rust → zkVM
- [ ] Create standard library for zkVM contracts
- [ ] Gas metering for zkVM execution

#### Milestone 3.2: Smart Contract Layer
- [ ] Contract deployment mechanism
- [ ] Contract state storage (world state tree)
- [ ] Contract call format
- [ ] ABI encoding/decoding
- [ ] Contract-to-contract calls

#### Milestone 3.3: Private Smart Contracts
- [ ] Private state updates (hidden contract state)
- [ ] Private function calls (hide function selector)
- [ ] Selective disclosure (provable public outputs)
- [ ] Private NFTs
- [ ] Private DeFi primitives (DEX, lending)

#### Milestone 3.4: Recursive Proofs
- [ ] Proof composition (prove proof verification)
- [ ] Aggregation layer (combine multiple proofs)
- [ ] Compression (reduce proof size)
- [ ] IVC (Incrementally Verifiable Computation)
- [ ] Application: light clients, bridges

#### Milestone 3.5: Cross-Chain Bridges
- [ ] ZK proof of consensus (prove finality on other chains)
- [ ] Message passing protocol
- [ ] Asset bridge (lock on one chain, mint on another)
- [ ] Bridge security (fraud proofs, watchtowers)
- [ ] Ethereum bridge (for interop)

---

### Phase 4: Optimization & Security (Week 21-26)
**Goal**: Production-ready code with security guarantees.

#### Milestone 4.1: Performance Optimization
- [ ] Profile critical paths (proof gen, verification, sync)
- [ ] Optimize merkle tree operations
- [ ] Parallelize proof verification
- [ ] Database query optimization
- [ ] Memory pool tuning

#### Milestone 4.2: Security Hardening
- [ ] Fuzzing (libFuzzer, AFL++) for all parsers
- [ ] Formal verification of critical modules (Kani, Prusti)
- [ ] Side-channel analysis (constant-time ops)
- [ ] External security audit (Trail of Bits, etc.)
- [ ] Bug bounty program

#### Milestone 4.3: Developer Tooling
- [ ] SDK for wallet developers
- [ ] Block explorer (web interface)
- [ ] Transaction builder GUI
- [ ] Contract development framework
- [ ] Testnet faucet

#### Milestone 4.4: Documentation
- [ ] Technical specification (whitepaper-style)
- [ ] Developer guide (API reference)
- [ ] User guide (wallet, mining)
- [ ] Circuit documentation (constraint system)
- [ ] Security model documentation

#### Milestone 4.5: Testnet Launch
- [ ] Deploy public testnet
- [ ] Faucet for test tokens
- [ ] Community testing program
- [ ] Monitor for issues
- [ ] Iterate based on feedback

---

### Phase 5: Mainnet & Ecosystem (Week 27+)
**Goal**: Launch mainnet and grow ecosystem.

#### Milestone 5.1: Mainnet Preparation
- [ ] Genesis block ceremony
- [ ] Parameter finalization (block time, difficulty, etc.)
- [ ] Security audit completion
- [ ] Launch checklist review
- [ ] Community voting on launch date

#### Milestone 5.2: Mainnet Launch
- [ ] Fair launch (no premine)
- [ ] Bootstrap node deployment (5+ geographically distributed)
- [ ] Mining documentation
- [ ] Wallet release (CLI + GUI)
- [ ] Block explorer live

#### Milestone 5.3: Ecosystem Development
- [ ] Grants program for developers
- [ ] DEX protocol design
- [ ] Stablecoin (private, algorithmic)
- [ ] Privacy-preserving DeFi apps
- [ ] Integration with existing tools (MetaMask snap, etc.)

#### Milestone 5.4: Research & Development
- [ ] Post-quantum cryptography exploration
- [ ] Folding schemes (Nova, SuperNova)
- [ ] Lookup arguments (Plookup, Baloo)
- [ ] zkML (machine learning on-chain)
- [ ] Future protocol upgrades

---

## Non-Goals

- **EVM Compatibility**: We're not cloning Ethereum. Clean slate design.
- **High TPS at Launch**: Correctness > speed. Optimize later.
- **PoS Consensus**: PoW is simpler and more studied for now.
- **Governance Tokens**: The chain's token is the only token. No gimmicks.

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

- GitHub: [https://github.com/Oblivionsage/nullchain]
- Telegram : https://t.me/thankful_for_everyday

---

**"Privacy is not a feature. It's a requirement."**
