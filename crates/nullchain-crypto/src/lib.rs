//! Cryptographic primitives for NullChain

pub mod hash;
pub mod keys;

pub use hash::{hash_data, double_hash, hash_block_header};
pub use keys::{generate_keypair, sign_message, verify_signature};
