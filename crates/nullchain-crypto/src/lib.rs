//! Cryptographic primitives for NullChain

pub mod hash;
pub mod keys;

pub use hash::{double_hash, hash_block_header, hash_data};
pub use keys::{generate_keypair, sign_message, verify_signature};
