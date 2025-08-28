use crate::{error::Result, crypto::{hash, generate_nonce}};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment {
    pub commitment_hash: String,
    pub nonce: String,
    pub amount: Option<u64>, // None for hiding the amount
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeProof {
    pub proof_hash: String,
    pub commitment_hash: String,
    pub amount: u64,
    pub nonce: String,
}

pub struct CommitmentScheme;

impl CommitmentScheme {
    /// Create a commitment to an amount without revealing it
    pub fn commit(amount: u64) -> Result<String> {
        let nonce = generate_nonce();
        let commitment = Self::create_commitment(amount, &nonce)?;
        Ok(commitment.commitment_hash)
    }
    
    /// Create a commitment with a specific nonce
    pub fn create_commitment(amount: u64, nonce: &[u8; 32]) -> Result<Commitment> {
        let mut hasher = Sha256::new();
        hasher.update(amount.to_le_bytes());
        hasher.update(nonce);
        let commitment_hash = hex::encode(hasher.finalize());
        
        Ok(Commitment {
            commitment_hash,
            nonce: hex::encode(nonce),
            amount: None, // Hide the amount
        })
    }
    
    /// Prove knowledge of the amount without revealing it
    pub fn prove_knowledge(amount: u64) -> Result<String> {
        let nonce = generate_nonce();
        let commitment = Self::create_commitment(amount, &nonce)?;
        
        // Create a proof that demonstrates knowledge of the amount
        let mut hasher = Sha256::new();
        hasher.update(amount.to_le_bytes());
        hasher.update(&nonce);
        hasher.update(b"knowledge_proof");
        let proof_hash = hex::encode(hasher.finalize());
        
        Ok(proof_hash)
    }
    
    /// Verify that a proof demonstrates knowledge of the committed amount
    pub fn verify_knowledge(commitment_hash: &str, proof: &str) -> Result<bool> {
        // In a real implementation, this would verify the zero-knowledge proof
        // For this demo, we'll simulate verification
        Ok(commitment_hash.len() == 64 && proof.len() == 64)
    }
    
    /// Open a commitment to reveal the amount
    pub fn open_commitment(commitment: &Commitment, amount: u64, nonce: &str) -> Result<bool> {
        let nonce_bytes = hex::decode(nonce)
            .map_err(|_| crate::error::ShieldedError::CryptoError("Invalid nonce".to_string()))?;
        
        let expected_commitment = Self::create_commitment(amount, &nonce_bytes.try_into().unwrap())?;
        Ok(commitment.commitment_hash == expected_commitment.commitment_hash)
    }
    
    /// Create a range proof (simplified version)
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
        
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Verify a range proof
    pub fn verify_range_proof(proof: &str, commitment_hash: &str, min: u64, max: u64) -> Result<bool> {
        // In a real implementation, this would verify the range proof
        // For this demo, we'll simulate verification
        Ok(proof.len() == 64 && commitment_hash.len() == 64)
    }
}
