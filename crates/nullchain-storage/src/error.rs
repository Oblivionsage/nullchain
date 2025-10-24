use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(#[from] rocksdb::Error),
    
    #[error("serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    
    #[error("block not found: {0}")]
    BlockNotFound(String),
    
    #[error("utxo not found")]
    UtxoNotFound,
    
    #[error("invalid height")]
    InvalidHeight,
}

pub type Result<T> = std::result::Result<T, StorageError>;
