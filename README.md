# NullChain

Privacy-first Layer-1 blockchain. Minimal codebase. Zero-knowledge proofs.

    Status: Experimental (testnet)
    Language: Rust
    License: MIT/Apache-2.0

## Install
```bash
git clone https://github.com/Oblivionsage/nullchain
cd nullchain
cargo build --release
```

## Usage
```bash
# Generate keypair
nullchain keygen
nullchain keygen --output ~/.nullchain

# Derive address
nullchain address --pubkey ~/.nullchain/key.pub

# Mine block
nullchain mine --bits 0x1f0fffff

# Genesis block
nullchain genesis

# Block info
nullchain info --json <block.json>
```

## Architecture
```
nullchain/
├── nullchain-types/      Block, Transaction, Merkle
├── nullchain-crypto/     Ed25519, Blake3, Zeroize
├── nullchain-consensus/  PoW, Difficulty
├── nullchain-network/    P2P (libp2p)
├── nullchain-storage/    RocksDB
└── nullchain-node/       CLI
```

## Cryptography

    Hashing:    Blake3
    Signatures: Ed25519 (zeroized on drop)
    ZK Proofs:  Plonky2 (no trusted setup)

## Roadmap

### Phase 0: Foundation [COMPLETE]

    [x] Block structure with Merkle trees
    [x] UTXO transaction model
    [x] Ed25519 signatures
    [x] Blake3 double-hashing for PoW
    [x] Difficulty adjustment algorithm
    [x] CLI wallet (keygen, address)
    [x] Encrypted key storage
    [x] Memory safety (zeroize)

### Phase 1: Core Privacy 

**1.1 Confidential Transactions**

    [ ] Pedersen commitments
    [ ] Range proofs (Bulletproofs)
    [ ] Confidential addresses

**1.2 Ring Signatures**

    [ ] MLSAG/CLSAG implementation
    [ ] Ring size: 11 (configurable)
    [ ] Key images (double-spend prevention)
    [ ] Decoy selection algorithm

**1.3 Stealth Addresses**

    [ ] One-time addresses per transaction
    [ ] Dual-key protocol (view/spend)
    [ ] Address scanning with view key

**1.4 RingCT**

    [ ] Ring signatures + confidential amounts
    [ ] Prunable RingCT
    [ ] Triptych (logarithmic proof size)

### Phase 2: Network Privacy 

**2.1 Dandelion++**

    [ ] Stem phase (anonymous relay)
    [ ] Fluff phase (public broadcast)
    [ ] Timing analysis resistance

**2.2 Tor/I2P**

    [ ] Onion routing for P2P
    [ ] Hidden services
    [ ] I2P garlic routing

**2.3 Decoy Transactions**

    [ ] Automatic dummy transactions
    [ ] Pattern mimicking
    [ ] Configurable noise level

**2.4 Traffic Obfuscation**

    [ ] Random relay delays
    [ ] Uniform packet padding
    [ ] Batched broadcasting

### Phase 3: Advanced ZK 

**3.1 Plonky2**

    [ ] Recursive proof composition
    [ ] Fast proving (<1s)
    [ ] Small proofs (<200KB)

**3.2 Shielded Pool**

    [ ] zkSNARK circuits
    [ ] Nullifier set
    [ ] Note commitment tree

**3.3 Hybrid Model**

    [ ] Transparent pool
    [ ] Shielded pool
    [ ] Cross-pool transactions

**3.4 ZK Set Membership**

    [ ] UTXO existence proofs
    [ ] Aggregated membership
    [ ] Accumulator-based

### Phase 4: zkVM 

**4.1 zkVM Integration**

    [ ] Risc Zero / SP1 / Jolt evaluation
    [ ] Rust → zkVM compiler
    [ ] Standard library
    [ ] Gas metering

**4.2 Private Contracts**

    [ ] Hidden state
    [ ] Private function calls
    [ ] Encrypted storage
    [ ] Selective disclosure

**4.3 Privacy DeFi**

    [ ] Private DEX
    [ ] Confidential lending
    [ ] Private stablecoins
    [ ] Anonymous governance

**4.4 Cross-Contract**

    [ ] Contract composition
    [ ] Encrypted messages
    [ ] Proof of execution

### Phase 5: Quantum Resistance 

**5.1 Post-Quantum Signatures**

    [ ] SPHINCS+ (hash-based)
    [ ] Dilithium (lattice)
    [ ] Falcon (compact lattice)
    [ ] Hybrid (classical + PQ)

**5.2 PQ Key Exchange**

    [ ] Kyber (lattice KEM)
    [ ] NTRU encryption
    [ ] Migration from Ed25519

**5.3 PQ ZK Proofs**

    [ ] STARKs (quantum-resistant)
    [ ] Lattice-based zkSNARKs
    [ ] Hash-based commitments

### Phase 6: Advanced Features 

**6.1 ZK Rollups**

    [ ] Layer 2 scaling
    [ ] Batch transactions
    [ ] 1000x throughput

**6.2 Private Assets**

    [ ] Confidential tokens
    [ ] Colored coins with privacy
    [ ] NFTs with ownership privacy
    [ ] Private atomic swaps

**6.3 Mimblewimble**

    [ ] Confidential transactions
    [ ] Cut-through pruning
    [ ] Compact blockchain

**6.4 Threshold Cryptography**

    [ ] Multi-sig wallets (private)
    [ ] Distributed key generation
    [ ] Threshold decryption
    [ ] MPC key management

**6.5 Time-Lock**

    [ ] VDFs (verifiable delay)
    [ ] Sealed transactions
    [ ] Time-released secrets
    [ ] Fair exchange

**6.6 Homomorphic Encryption**

    [ ] Compute on encrypted data
    [ ] Private state transitions
    [ ] Encrypted analytics
    [ ] ZK machine learning

### Phase 7: Censorship Resistance 

**7.1 P2P Hardening**

    [ ] Kademlia DHT
    [ ] Gossip optimization
    [ ] Eclipse mitigation
    [ ] Sybil resistance

**7.2 PoS + Privacy**

    [ ] Private validator selection
    [ ] Confidential staking
    [ ] Anonymous block producers

**7.3 Cross-Chain**

    [ ] zkBridge (Ethereum)
    [ ] Lightning Network
    [ ] Atomic swaps (Monero)
    [ ] Private IBC (Cosmos)

**7.4 Compliance (Optional)**

    [ ] View keys for auditing
    [ ] Selective disclosure
    [ ] Privacy-preserving KYC

### Phase 8: Research 

**8.1 FHE**

    [ ] Fully homomorphic encryption
    [ ] Arbitrary computation
    [ ] Private analytics
    [ ] Confidential AI

**8.2 Indistinguishability Obfuscation**

    [ ] Hide program logic
    [ ] Private contract code

**8.3 MPC**

    [ ] Distributed key generation
    [ ] Threshold signatures
    [ ] Private set intersection
    [ ] Secure auctions

**8.4 ZK Machine Learning**

    [ ] Prove model accuracy
    [ ] Private training data
    [ ] Verifiable inference
    [ ] On-chain zkML

## Security

**Threat Model:**

- Global passive adversary
- Active network attackers
- Chain analysis firms
- Malicious nodes (33% Byzantine)
- Quantum computers (future)

**Current Mitigations:**

- Constant-time operations
- Memory wiping (zeroize)
- Encrypted key storage
- Minimal codebase (<10k lines)
- Audited dependencies only

**Report vulnerabilities:** GitHub Security Advisories

## Build

```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
```

## License

MIT OR Apache-2.0

---

    github.com/Oblivionsage/nullchain
