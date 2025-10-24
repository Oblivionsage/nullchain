## NULLCHAIN

Minimal privacy-first Layer-1 blockchain. Zero-knowledge proofs. No compromise.

    Status:       [EXPERIMENTAL]
    License:      MIT/Apache-2.0
    Language:     Rust
    Code:         <10k lines 

---

## INSTALL

    git clone https://github.com/Oblivionsage/nullchain
    cd nullchain
    cargo build --release

---

## USAGE

    nullchain keygen                    # Generate keypair
    nullchain keygen --output ~/.null   # Save encrypted keys
    nullchain address --pubkey <file>   # Derive address
    nullchain mine --bits 0x1f0fffff    # Mine block
    nullchain genesis                   # Genesis block

---

## CRYPTOGRAPHY

    Hashing:       Blake3
    Signatures:    Ed25519
    ZK Backend:    Plonky2 (no trusted setup)
    Memory:        Zeroized on drop

---

## ROADMAP

### Phase 0: Foundation [COMPLETE]
**Goal**: Basic blockchain infrastructure

    [x] Block structure with Merkle trees
    [x] UTXO transaction model
    [x] Ed25519 signatures
    [x] Blake3 double-hashing for PoW
    [x] Difficulty adjustment algorithm
    [x] CLI wallet (keygen, address)
    [x] Encrypted key storage with passphrase
    [x] Memory safety (zeroize sensitive data)

---

### Phase 1: Core Privacy Layer 

**Goal**: Transaction-level privacy without trusted setup

#### 1.1: Confidential Transactions

    [ ] Pedersen commitments for amount hiding
    [ ] Range proofs (prevent negative amounts)
    [ ] Bulletproofs for compact proof size
    [ ] Confidential address scheme
    [ ] Balance verification without revealing amounts

#### 1.2: Ring Signatures (Monero-style)

    [ ] MLSAG (Multilayered Linkable Spontaneous Anonymous Group)
    [ ] Ring size: configurable (default 11)
    [ ] Key image for double-spend prevention
    [ ] Decoy selection algorithm (resist chain analysis)
    [ ] Subaddress support for receive privacy

#### 1.3: Stealth Addresses

    [ ] One-time addresses per transaction
    [ ] Dual-key stealth protocol (view/spend keys)
    [ ] Address scanning with view key
    [ ] Encrypted payment IDs
    [ ] Integrated addresses for invoicing

#### 1.4: RingCT Integration

    [ ] Combine ring signatures + confidential amounts
    [ ] CLSAG signatures (more efficient than MLSAG)
    [ ] Prunable RingCT (reduce blockchain bloat)
    [ ] Triptych (future: logarithmic proof size)

---

### Phase 2: Network Privacy

**Goal**: Protect metadata and network-level information

#### 2.1: Dandelion++ Transaction Broadcast

    [ ] Stem phase (anonymous relay)
    [ ] Fluff phase (public broadcast)
    [ ] Resist timing analysis
    [ ] IP address unlinkability

#### 2.2: Tor/I2P Integration

    [ ] Onion routing for P2P connections
    [ ] Hidden service support
    [ ] I2P garlic routing
    [ ] Mixed network selection

#### 2.3: Decoy Transaction Generation

    [ ] Automatic dummy transactions
    [ ] Mimic real transaction patterns
    [ ] Obfuscate transaction graph
    [ ] Configurable noise level

#### 2.4: Network Timing Obfuscation

    [ ] Random delays in transaction relay
    [ ] Padding to uniform packet sizes
    [ ] Traffic analysis resistance
    [ ] Batched transaction broadcasting

---

### Phase 3: Advanced ZK Proofs 

**Goal**: zkSNARKs/zkSTARKs for enhanced privacy

#### 3.1: Plonky2 Integration

    [ ] Recursive proof composition
    [ ] Fast proof generation (<1s)
    [ ] Small proof size (<200KB)
    [ ] No trusted setup required

#### 3.2: Shielded Transaction Pool

    [ ] zkSNARK circuits for private transfers
    [ ] Nullifier set (prevent double-spend)
    [ ] Note commitment tree (Merkle)
    [ ] Spend/output circuits

#### 3.3: Hybrid Privacy Model

    [ ] Transparent pool (like Bitcoin)
    [ ] Shielded pool (like Zcash Sapling)
    [ ] Cross-pool transactions (shield/unshield)
    [ ] User choice: speed vs privacy

#### 3.4: Zero-Knowledge Set Membership

    [ ] Prove UTXO exists without revealing which
    [ ] Aggregated membership proofs
    [ ] Constant-size proofs regardless of set size
    [ ] Accumulator-based approach

---

### Phase 4: zkVM & Private Contracts

**Goal**: Programmable privacy with smart contracts

#### 4.1: zkVM Selection & Integration

    [ ] Evaluate: Risc Zero, SP1, Jolt, zkMIPS
    [ ] Rust → zkVM compiler toolchain
    [ ] Standard library for zkVM contracts
    [ ] Gas metering for execution cost

#### 4.2: Private Smart Contract Layer

    [ ] Deploy contracts with hidden state
    [ ] Private function calls (hide selector)
    [ ] Encrypted contract storage
    [ ] Selective disclosure (prove properties)

#### 4.3: Privacy-Preserving DeFi

    [ ] Private DEX (hidden orders, amounts)
    [ ] Confidential lending (rates hidden)
    [ ] Private stablecoins
    [ ] Anonymous voting/governance

#### 4.4: Cross-Contract Privacy

    [ ] Private contract composition
    [ ] Encrypted inter-contract messages
    [ ] Proof of execution correctness
    [ ] Minimal information leakage

---

### Phase 5: Quantum Resistance

**Goal**: Future-proof against quantum computers

#### 5.1: Post-Quantum Signatures

    [ ] SPHINCS+ (stateless hash-based)
    [ ] Dilithium (lattice-based)
    [ ] Falcon (compact lattice signatures)
    [ ] Hybrid scheme (classical + PQ)

#### 5.2: Post-Quantum Key Exchange

    [ ] Kyber (lattice-based KEM)
    [ ] NTRU for encryption
    [ ] Migration path from Ed25519

#### 5.3: Quantum-Resistant ZK Proofs

    [ ] STARKs (already quantum-resistant)
    [ ] Lattice-based zkSNARKs
    [ ] Hash-based commitments

---

### Phase 6: Advanced Privacy Features

**Goal**: State-of-the-art anonymity

#### 6.1: Zero-Knowledge Rollups

    [ ] Layer 2 scaling with privacy
    [ ] Batch thousands of transactions
    [ ] Single ZK proof for entire batch
    [ ] 1000x throughput increase

#### 6.2: Private Asset Issuance

    [ ] Confidential tokens (amounts hidden)
    [ ] Colored coins with privacy
    [ ] NFTs with ownership privacy
    [ ] Private atomic swaps

#### 6.3: Mimblewimble Integration

    [ ] Confidential transactions (different approach)
    [ ] Cut-through (prune transaction history)
    [ ] Compact blockchain (no addresses stored)
    [ ] Dandelion for network privacy

#### 6.4: Threshold Cryptography

    [ ] Multi-signature wallets (privacy-preserving)
    [ ] Distributed key generation
    [ ] Threshold decryption
    [ ] MPC for key management

#### 6.5: Time-Lock Encryption

    [ ] Verifiable delay functions (VDFs)
    [ ] Sealed transactions (revealed later)
    [ ] Time-released secrets
    [ ] Fair exchange protocols

#### 6.6: Homomorphic Encryption

    [ ] Compute on encrypted data
    [ ] Private smart contract state transitions
    [ ] Encrypted blockchain analytics
    [ ] Zero-knowledge machine learning

---

### Phase 7: Censorship Resistance

**Goal**: Unstoppable privacy network

#### 7.1: P2P Network Hardening

    [ ] Kademlia DHT for peer discovery
    [ ] Gossip protocol optimization
    [ ] Eclipse attack mitigation
    [ ] Sybil attack resistance

#### 7.2: Proof-of-Stake + Privacy

    [ ] Private validator selection
    [ ] Confidential staking amounts
    [ ] Anonymous block producers
    [ ] No stake grinding attacks

#### 7.3: Cross-Chain Privacy Bridges

    [ ] zkBridge for Ethereum (private)
    [ ] Lightning Network integration
    [ ] Atomic swaps with Monero
    [ ] Private IBC (Cosmos)

#### 7.4: Regulatory Compliance Layer (Optional)

    [ ] View keys for auditing
    [ ] Selective disclosure proofs
    [ ] Compliance tools (opt-in)
    [ ] Privacy-preserving KYC

---

### Phase 8: Research & Future Tech 

**Goal**: Push boundaries of cryptographic privacy

#### 8.1: Fully Homomorphic Encryption (FHE)

    [ ] Arbitrary computation on encrypted data
    [ ] No decryption needed for operations
    [ ] Private blockchain analytics
    [ ] Confidential AI on-chain

#### 8.2: Indistinguishability Obfuscation

    [ ] Hide program logic completely
    [ ] Private smart contract code
    [ ] Theoretical limit of obfuscation

#### 8.3: Multi-Party Computation (MPC)

    [ ] Distributed private key generation
    [ ] Threshold signatures (no single point of failure)
    [ ] Private set intersection
    [ ] Secure auctions

---

## SECURITY


### Current Mitigations
    [x] Constant-time cryptographic operations
    [x] Memory wiping (zeroize)
    [x] Encrypted key storage
    [x] Minimal codebase (<10k lines)
    [x] Audited dependencies only

### Planned Mitigations

    [ ] Ring signatures (unlinkability)
    [ ] Stealth addresses (untraceability)
    [ ] Network-level privacy (Tor/I2P)
    [ ] Post-quantum cryptography

**Report vulnerabilities:** GitHub Security Advisories

---

## CONTRIBUTING

Read CONTRIBUTING.md. Security PRs prioritized.

---

## LICENSE

MIT OR Apache-2.0

---


    Build: 2025-10-24 | Version: 0.1.0
