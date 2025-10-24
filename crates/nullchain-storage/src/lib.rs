//! Blockchain storage using RocksDB
//!
//! Provides persistent storage for:
//! - Blocks (by height and hash)
//! - UTXO set (unspent transaction outputs)
//! - Chain state (best block, height)

mod blockstore;
mod error;
mod utxo;

pub use blockstore::BlockStore;
pub use error::{Result, StorageError};
pub use utxo::UtxoSet;

use rocksdb::{Options, DB};
use std::path::Path;

/// Database handle
pub struct Database {
    db: DB,
}

impl Database {
    /// Open database at path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let db = DB::open(&opts, path)?;
        Ok(Self { db })
    }

    /// Get reference to RocksDB
    pub fn inner(&self) -> &DB {
        &self.db
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_open_database() {
        let dir = TempDir::new().unwrap();
        let db = Database::open(dir.path()).unwrap();
        assert!(db.inner().path().exists());
    }
}
