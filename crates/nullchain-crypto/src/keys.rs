//! Ed25519 key operations with security hardening
//!
//! All sensitive key material is zeroized on drop.
//! Uses constant-time operations where applicable.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroizing;

/// Generate Ed25519 keypair using system CSPRNG
///
/// # Security
/// - Uses OS-provided cryptographically secure random number generator
/// - Private key material is zeroized on drop
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;

    // Generate random bytes for private key
    let mut secret_bytes = Zeroizing::new([0u8; SECRET_KEY_LENGTH]);
    csprng.fill_bytes(&mut *secret_bytes);

    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();

    (signing_key, verifying_key)
}

/// Sign message with private key
///
/// # Security
/// - Uses Ed25519 signature scheme (collision-resistant)
/// - Signature is deterministic (no nonce reuse vulnerabilities)
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
    let signature = signing_key.sign(message);
    signature.to_bytes().to_vec()
}

/// Verify signature with constant-time comparison
///
/// # Security
/// - Constant-time verification (no timing side-channels)
/// - Returns bool, not Result (simpler API, less error-prone)
pub fn verify_signature(verifying_key: &VerifyingKey, message: &[u8], signature: &[u8]) -> bool {
    // Reject malformed signatures immediately
    if signature.len() != 64 {
        return false;
    }

    // Parse signature
    let sig = match Signature::from_slice(signature) {
        Ok(s) => s,
        Err(_) => return false,
    };

    // Constant-time verification
    use ed25519_dalek::Verifier;
    verifying_key.verify(message, &sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let (signing_key, verifying_key) = generate_keypair();
        assert_eq!(
            signing_key.verifying_key().as_bytes(),
            verifying_key.as_bytes()
        );
    }

    #[test]
    fn test_sign_and_verify() {
        let (signing_key, verifying_key) = generate_keypair();
        let message = b"nullchain: privacy is mandatory, not optional";

        let signature = sign_message(&signing_key, message);
        assert!(verify_signature(&verifying_key, message, &signature));
    }

    #[test]
    fn test_verify_tampered_message() {
        let (signing_key, verifying_key) = generate_keypair();
        let message = b"original message";
        let signature = sign_message(&signing_key, message);

        let tampered = b"tampered message";
        assert!(!verify_signature(&verifying_key, tampered, &signature));
    }

    #[test]
    fn test_verify_malformed_signature() {
        let (_, verifying_key) = generate_keypair();
        let message = b"test";

        // Too short
        let bad_sig = vec![0u8; 32];
        assert!(!verify_signature(&verifying_key, message, &bad_sig));

        // Too long
        let bad_sig = vec![0u8; 128];
        assert!(!verify_signature(&verifying_key, message, &bad_sig));
    }
}
