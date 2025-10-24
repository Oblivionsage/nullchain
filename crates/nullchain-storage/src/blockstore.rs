use crate::{Database, Result, StorageError};
use nullchain_types::{Block, Hash256};

const KEY_BEST: &[u8] = b"best";
const KEY_HEIGHT: &[u8] = b"height";

pub struct BlockStore<'a> {
    db: &'a Database,
}

impl<'a> BlockStore<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Store block at height
    pub fn put(&self, height: u64, block: &Block) -> Result<()> {
        let hash = block.hash();
        let block_data = bincode::serialize(block)?;

        // Store block by hash
        self.db.inner().put(hash.as_bytes(), &block_data)?;

        // Index by height
        let height_key = height.to_be_bytes();
        self.db.inner().put(height_key, hash.as_bytes())?;

        Ok(())
    }

    /// Get block by hash
    pub fn get_by_hash(&self, hash: &Hash256) -> Result<Block> {
        match self.db.inner().get(hash.as_bytes())? {
            Some(data) => Ok(bincode::deserialize(&data)?),
            None => Err(StorageError::BlockNotFound(hash.to_string())),
        }
    }

    /// Get block by height
    pub fn get_by_height(&self, height: u64) -> Result<Block> {
        let height_key = height.to_be_bytes();

        let hash_bytes = self
            .db
            .inner()
            .get(height_key)?
            .ok_or(StorageError::InvalidHeight)?;

        let hash = Hash256::from_bytes(
            hash_bytes
                .as_slice()
                .try_into()
                .map_err(|_| StorageError::InvalidHeight)?,
        );

        self.get_by_hash(&hash)
    }

    /// Update best block
    pub fn set_best(&self, height: u64, hash: &Hash256) -> Result<()> {
        self.db.inner().put(KEY_BEST, hash.as_bytes())?;
        self.db.inner().put(KEY_HEIGHT, height.to_be_bytes())?;
        Ok(())
    }

    /// Get best block hash
    pub fn get_best(&self) -> Result<Option<Hash256>> {
        match self.db.inner().get(KEY_BEST)? {
            Some(data) => {
                let hash = Hash256::from_bytes(
                    data.as_slice()
                        .try_into()
                        .map_err(|_| StorageError::BlockNotFound("best".to_string()))?,
                );
                Ok(Some(hash))
            }
            None => Ok(None),
        }
    }

    /// Get current height
    pub fn get_height(&self) -> Result<u64> {
        match self.db.inner().get(KEY_HEIGHT)? {
            Some(data) => {
                let bytes: [u8; 8] = data
                    .as_slice()
                    .try_into()
                    .map_err(|_| StorageError::InvalidHeight)?;
                Ok(u64::from_be_bytes(bytes))
            }
            None => Ok(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_block_storage() {
        let dir = TempDir::new().unwrap();
        let db = Database::open(dir.path()).unwrap();
        let store = BlockStore::new(&db);

        let block = Block::genesis();
        let hash = block.hash();

        store.put(0, &block).unwrap();

        let retrieved = store.get_by_height(0).unwrap();
        assert_eq!(retrieved.hash(), hash);

        let retrieved = store.get_by_hash(&hash).unwrap();
        assert_eq!(retrieved.hash(), hash);
    }

    #[test]
    fn test_best_block() {
        let dir = TempDir::new().unwrap();
        let db = Database::open(dir.path()).unwrap();
        let store = BlockStore::new(&db);

        assert_eq!(store.get_best().unwrap(), None);
        assert_eq!(store.get_height().unwrap(), 0);

        let block = Block::genesis();
        let hash = block.hash();

        store.set_best(0, &hash).unwrap();

        assert_eq!(store.get_best().unwrap(), Some(hash));
        assert_eq!(store.get_height().unwrap(), 0);
    }
}
