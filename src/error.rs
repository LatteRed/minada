use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShieldedError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },
    
    #[error("Invalid wallet address: {0}")]
    InvalidWalletAddress(String),
    
    #[error("Zero-knowledge proof generation failed: {0}")]
    ZKProofError(String),
    
    #[error("Commitment verification failed: {0}")]
    CommitmentError(String),
    
    #[error("Merkle tree operation failed: {0}")]
    MerkleTreeError(String),
    
    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),
    
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),
}

pub type Result<T> = std::result::Result<T, ShieldedError>;
