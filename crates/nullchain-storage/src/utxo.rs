use crate::{Database, Result};
use nullchain_types::{Hash256, TransactionOutput};

/// Outpoint (transaction output reference)
#[derive(Debug, Clone)]
pub struct OutPoint {
    pub txid: Hash256,
    pub index: u32,
}

impl OutPoint {
    pub fn to_key(&self) -> Vec<u8> {
        let mut key = Vec::with_capacity(36);
        key.extend_from_slice(self.txid.as_bytes());
        key.extend_from_slice(&self.index.to_be_bytes());
        key
    }
    
    #[allow(dead_code)]
    pub fn from_key(key: &[u8]) -> Option<Self> {
        if key.len() != 36 {
            return None;
        }
        
        let mut txid_bytes = [0u8; 32];
        txid_bytes.copy_from_slice(&key[0..32]);
        
        let mut index_bytes = [0u8; 4];
        index_bytes.copy_from_slice(&key[32..36]);
        
        Some(Self {
            txid: Hash256::from_bytes(txid_bytes),
            index: u32::from_be_bytes(index_bytes),
        })
    }
}

pub struct UtxoSet<'a> {
    db: &'a Database,
}

impl<'a> UtxoSet<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }
    
    /// Add UTXO
    pub fn add(&self, outpoint: &OutPoint, output: &TransactionOutput) -> Result<()> {
        let key = outpoint.to_key();
        let value = bincode::serialize(output)?;
        self.db.inner().put(&key, &value)?;
        Ok(())
    }
    
    /// Remove UTXO (spent)
    pub fn remove(&self, outpoint: &OutPoint) -> Result<()> {
        let key = outpoint.to_key();
        self.db.inner().delete(&key)?;
        Ok(())
    }
    
    /// Get UTXO
    pub fn get(&self, outpoint: &OutPoint) -> Result<Option<TransactionOutput>> {
        let key = outpoint.to_key();
        match self.db.inner().get(&key)? {
            Some(data) => Ok(Some(bincode::deserialize(&data)?)),
            None => Ok(None),
        }
    }
    
    /// Check if UTXO exists
    pub fn exists(&self, outpoint: &OutPoint) -> Result<bool> {
        let key = outpoint.to_key();
        Ok(self.db.inner().get(&key)?.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_utxo_set() {
        let dir = TempDir::new().unwrap();
        let db = Database::open(dir.path()).unwrap();
        let utxo = UtxoSet::new(&db);
        
        let outpoint = OutPoint {
            txid: Hash256::zero(),
            index: 0,
        };
        
        let output = TransactionOutput {
            amount: 100,
            recipient: vec![0u8; 20],
        };
        
        assert!(!utxo.exists(&outpoint).unwrap());
        
        utxo.add(&outpoint, &output).unwrap();
        assert!(utxo.exists(&outpoint).unwrap());
        
        let retrieved = utxo.get(&outpoint).unwrap().unwrap();
        assert_eq!(retrieved.amount, 100);
        
        utxo.remove(&outpoint).unwrap();
        assert!(!utxo.exists(&outpoint).unwrap());
    }
}
