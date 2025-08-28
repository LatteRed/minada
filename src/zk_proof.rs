use crate::{error::Result, crypto::{hash, generate_nonce}};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroKnowledgeProof {
    pub proof_id: String,
    pub transaction_id: String,
    pub proof_data: String,
    pub public_inputs: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub proof_type: ProofType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofType {
    SpendProof,
    OutputProof,
    BalanceProof,
    RangeProof,
}

impl ZeroKnowledgeProof {
    /// Generate a zero-knowledge proof for a transaction
    pub fn generate(transaction_id: &str) -> Result<String> {
        let proof_id = Self::generate_proof_id(transaction_id)?;
        let proof_data = Self::create_proof_data(transaction_id)?;
        
        Ok(format!("{}:{}", proof_id, proof_data))
    }
    
    /// Create a complete ZK proof structure
    pub fn create_spend_proof(
        transaction_id: &str,
        input_commitments: &[String],
        output_commitments: &[String],
        balance_proof: &str,
    ) -> Result<Self> {
        let proof_id = Self::generate_proof_id(transaction_id)?;
        let proof_data = Self::create_spend_proof_data(input_commitments, output_commitments, balance_proof)?;
        
        Ok(Self {
            proof_id,
            transaction_id: transaction_id.to_string(),
            proof_data,
            public_inputs: vec![
                format!("input_count:{}", input_commitments.len()),
                format!("output_count:{}", output_commitments.len()),
            ],
            timestamp: Utc::now(),
            proof_type: ProofType::SpendProof,
        })
    }
    
    /// Verify a zero-knowledge proof
    pub fn verify(&self) -> Result<bool> {
        // In a real implementation, this would verify the actual ZK proof
        // For this demo, we'll simulate verification
        Ok(self.proof_data.len() >= 64 && self.proof_id.len() >= 32)
    }
    
    /// Generate a proof ID based on transaction ID
    fn generate_proof_id(transaction_id: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(transaction_id.as_bytes());
        hasher.update(b"zk_proof");
        hasher.update(generate_nonce());
        
        Ok(hex::encode(&hasher.finalize()[..16]))
    }
    
    /// Create proof data for a transaction
    fn create_proof_data(transaction_id: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(transaction_id.as_bytes());
        hasher.update(b"proof_data");
        hasher.update(generate_nonce());
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Create spend proof data
    fn create_spend_proof_data(
        input_commitments: &[String],
        output_commitments: &[String],
        balance_proof: &str,
    ) -> Result<String> {
        let mut hasher = Sha256::new();
        
        // Hash input commitments
        for commitment in input_commitments {
            hasher.update(commitment.as_bytes());
        }
        
        // Hash output commitments
        for commitment in output_commitments {
            hasher.update(commitment.as_bytes());
        }
        
        // Hash balance proof
        hasher.update(balance_proof.as_bytes());
        hasher.update(b"spend_proof");
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Create a range proof for amount validation
    pub fn create_range_proof(amount: u64, min: u64, max: u64) -> Result<String> {
        if amount < min || amount > max {
            return Err(crate::error::ShieldedError::InvalidAmount(
                format!("Amount {} not in range [{}, {}]", amount, min, max)
            ));
        }
        
        let mut hasher = Sha256::new();
        hasher.update(amount.to_le_bytes());
        hasher.update(min.to_le_bytes());
        hasher.update(max.to_le_bytes());
        hasher.update(b"range_proof");
        hasher.update(generate_nonce());
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Create a balance proof showing total input equals total output
    pub fn create_balance_proof(input_total: u64, output_total: u64, fee: u64) -> Result<String> {
        if input_total != output_total + fee {
            return Err(crate::error::ShieldedError::InvalidTransaction(
                "Input total does not equal output total plus fee".to_string()
            ));
        }
        
        let mut hasher = Sha256::new();
        hasher.update(input_total.to_le_bytes());
        hasher.update(output_total.to_le_bytes());
        hasher.update(fee.to_le_bytes());
        hasher.update(b"balance_proof");
        
        Ok(hex::encode(hasher.finalize()))
    }
}

impl std::fmt::Display for ZeroKnowledgeProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ZKProof({}, type: {:?}, timestamp: {})",
            self.proof_id,
            self.proof_type,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}
