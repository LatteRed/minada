use crate::{
    error::Result,
    crypto::{hash, generate_nonce},
    commitment::CommitmentScheme,
    zk_proof::ZeroKnowledgeProof,
};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldedTransaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub transaction_type: TransactionType,
    pub input_commitments: Vec<String>,
    pub output_commitments: Vec<String>,
    pub zk_proof: Option<String>,
    pub signature: String,
    pub timestamp: DateTime<Utc>,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Public,
    Shielded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

impl ShieldedTransaction {
    /// Create a public transaction (visible amounts)
    pub fn create_public(from: &str, to: &str, amount: u64) -> Result<Self> {
        let id = Self::generate_transaction_id(from, to, amount)?;
        let fee = Self::calculate_fee(amount);
        let signature = Self::generate_signature(&id, from)?;
        
        Ok(Self {
            id,
            from: from.to_string(),
            to: to.to_string(),
            amount,
            fee,
            transaction_type: TransactionType::Public,
            input_commitments: vec![],
            output_commitments: vec![],
            zk_proof: None,
            signature,
            timestamp: Utc::now(),
            status: TransactionStatus::Pending,
        })
    }
    
    /// Create a shielded transaction (hidden amounts)
    pub fn create_shielded(from: &str, to: &str, amount: u64) -> Result<Self> {
        let id = Self::generate_transaction_id(from, to, amount)?;
        let fee = Self::calculate_fee(amount);
        
        // Create input commitment (spending from shielded balance)
        let input_commitment = CommitmentScheme::commit(amount + fee)?;
        
        // Create output commitment (sending to recipient)
        let output_commitment = CommitmentScheme::commit(amount)?;
        
        // Create change commitment (if any)
        let change_commitment = if fee > 0 {
            Some(CommitmentScheme::commit(0)?) // Change goes back to sender
        } else {
            None
        };
        
        let mut input_commitments = vec![input_commitment];
        let mut output_commitments = vec![output_commitment];
        
        if let Some(change) = change_commitment {
            output_commitments.push(change);
        }
        
        // Generate zero-knowledge proof
        let zk_proof = ZeroKnowledgeProof::generate(&id)?;
        
        let signature = Self::generate_signature(&id, from)?;
        
        Ok(Self {
            id,
            from: from.to_string(),
            to: to.to_string(),
            amount,
            fee,
            transaction_type: TransactionType::Shielded,
            input_commitments,
            output_commitments,
            zk_proof: Some(zk_proof),
            signature,
            timestamp: Utc::now(),
            status: TransactionStatus::Pending,
        })
    }
    
    /// Verify a transaction
    pub fn verify(transaction_id: &str) -> Result<bool> {
        // In a real implementation, this would verify the transaction on the blockchain
        // For this demo, we'll simulate verification
        Ok(transaction_id.len() >= 32)
    }
    
    /// Generate a transaction ID
    fn generate_transaction_id(from: &str, to: &str, amount: u64) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(from.as_bytes());
        hasher.update(to.as_bytes());
        hasher.update(amount.to_le_bytes());
        hasher.update(generate_nonce());
        hasher.update(Uuid::new_v4().as_bytes());
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Calculate transaction fee
    fn calculate_fee(amount: u64) -> u64 {
        // Simple fee calculation: 0.1% of amount, minimum 1
        std::cmp::max(1, amount / 1000)
    }
    
    /// Generate a signature for the transaction
    fn generate_signature(transaction_id: &str, signer: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(transaction_id.as_bytes());
        hasher.update(signer.as_bytes());
        hasher.update(generate_nonce());
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Get the total input amount (for shielded transactions)
    pub fn get_input_total(&self) -> u64 {
        self.amount + self.fee
    }
    
    /// Get the total output amount (for shielded transactions)
    pub fn get_output_total(&self) -> u64 {
        self.amount
    }
    
    /// Check if the transaction is balanced (inputs = outputs + fee)
    pub fn is_balanced(&self) -> bool {
        self.get_input_total() == self.get_output_total() + self.fee
    }
    
    /// Convert to JSON for storage/transmission
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| crate::error::ShieldedError::SerializationError(e))
    }
    
    /// Create from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .map_err(|e| crate::error::ShieldedError::SerializationError(e))
    }
}

impl std::fmt::Display for ShieldedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction({}, {} -> {}, amount: {}, type: {:?}, status: {:?})",
            self.id,
            self.from,
            self.to,
            self.amount,
            self.transaction_type,
            self.status
        )
    }
}
