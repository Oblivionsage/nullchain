use crate::hash::Hash256;
use serde::{Deserialize, Serialize};

/// Transaction input (spending a previous output)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    /// Hash of the transaction containing the output to spend
    pub previous_output: Hash256,

    /// Index of the output in the previous transaction
    pub output_index: u32,

    /// Signature proving ownership (Ed25519, 64 bytes)
    pub signature: Vec<u8>,

    /// Public key of the spender (32 bytes)
    pub public_key: Vec<u8>,
}

/// Transaction output (creating new spendable coins)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    /// Amount in nanoNULL (1 NULL = 10^9 nanoNULL)
    pub amount: u64,

    /// Public key hash of the recipient (20 bytes, RIPEMD160(SHA256(pubkey)))
    pub recipient: Vec<u8>,
}

/// Transaction (transfer of value)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction version
    pub version: u32,

    /// Inputs (coins being spent)
    pub inputs: Vec<TransactionInput>,

    /// Outputs (coins being created)
    pub outputs: Vec<TransactionOutput>,

    /// Locktime (block height or timestamp before which tx is invalid)
    pub locktime: u64,
}

impl Transaction {
    /// Create a coinbase transaction (mining reward)
    pub fn coinbase(recipient: Vec<u8>, amount: u64, block_height: u64) -> Self {
        Self {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: Hash256::zero(),
                output_index: 0xFFFFFFFF, // Special value for coinbase
                signature: vec![],        // No signature needed for coinbase
                public_key: block_height.to_le_bytes().to_vec(), // Block height as "pubkey"
            }],
            outputs: vec![TransactionOutput { amount, recipient }],
            locktime: 0,
        }
    }

    /// Check if this is a coinbase transaction
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1
            && self.inputs[0].previous_output == Hash256::zero()
            && self.inputs[0].output_index == 0xFFFFFFFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coinbase_transaction() {
        let recipient = vec![0u8; 20]; // Dummy address
        let tx = Transaction::coinbase(recipient, 100_000_000_000, 0);

        assert!(tx.is_coinbase());
        assert_eq!(tx.inputs.len(), 1);
        assert_eq!(tx.outputs.len(), 1);
        assert_eq!(tx.outputs[0].amount, 100_000_000_000);
    }
}
