//! Key generation and signature verification using Ed25519

use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use rand::RngCore;

/// Generate a new Ed25519 keypair
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let mut secret_bytes = [0u8; SECRET_KEY_LENGTH];
    csprng.fill_bytes(&mut secret_bytes);
    
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Sign a message with a private key
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
    let signature = signing_key.sign(message);
    signature.to_bytes().to_vec()
}

/// Verify a signature
pub fn verify_signature(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &[u8],
) -> bool {
    if signature.len() != 64 {
        return false;
    }
    
    let sig = match Signature::from_slice(signature) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    verifying_key.verify(message, &sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keypair_generation() {
        let (signing_key, verifying_key) = generate_keypair();
        assert_eq!(signing_key.verifying_key().as_bytes(), verifying_key.as_bytes());
    }
    
    #[test]
    fn test_sign_and_verify() {
        let (signing_key, verifying_key) = generate_keypair();
        let message = b"nullchain transaction";
        
        let signature = sign_message(&signing_key, message);
        assert!(verify_signature(&verifying_key, message, &signature));
    }
    
    #[test]
    fn test_verify_wrong_message() {
        let (signing_key, verifying_key) = generate_keypair();
        let message = b"original";
        let signature = sign_message(&signing_key, message);
        
        let wrong_message = b"tampered";
        assert!(!verify_signature(&verifying_key, wrong_message, &signature));
    }
    
    #[test]
    fn test_invalid_signature_length() {
        let (_, verifying_key) = generate_keypair();
        let message = b"test";
        let bad_sig = vec![0u8; 32]; // Wrong length
        
        assert!(!verify_signature(&verifying_key, message, &bad_sig));
    }
}
