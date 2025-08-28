use crate::{error::Result, crypto::generate_keypair};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub name: String,
    pub address: String,
    pub public_key: String,
    pub private_key: String, // In production, this would be encrypted
    pub balance: u64,
    pub shielded_balance: u64,
}

impl Wallet {
    pub fn new(name: &str) -> Result<Self> {
        let (public_key, private_key) = generate_keypair()?;
        let address = Self::generate_address(&public_key)?;
        
        Ok(Self {
            name: name.to_string(),
            address,
            public_key,
            private_key,
            balance: 1000, // Starting balance for demo
            shielded_balance: 0,
        })
    }
    
    fn generate_address(public_key: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(public_key.as_bytes());
        let result = hasher.finalize();
        Ok(format!("namada_{}", hex::encode(&result[..20])))
    }
    
    pub fn add_funds(&mut self, amount: u64) {
        self.balance += amount;
    }
    
    pub fn add_shielded_funds(&mut self, amount: u64) {
        self.shielded_balance += amount;
    }
    
    pub fn spend(&mut self, amount: u64) -> Result<()> {
        if self.balance < amount {
            return Err(crate::error::ShieldedError::InsufficientFunds {
                required: amount,
                available: self.balance,
            });
        }
        self.balance -= amount;
        Ok(())
    }
    
    pub fn spend_shielded(&mut self, amount: u64) -> Result<()> {
        if self.shielded_balance < amount {
            return Err(crate::error::ShieldedError::InsufficientFunds {
                required: amount,
                available: self.shielded_balance,
            });
        }
        self.shielded_balance -= amount;
        Ok(())
    }
    
    pub fn get_total_balance(&self) -> u64 {
        self.balance + self.shielded_balance
    }
    
    pub fn sign_message(&self, message: &[u8]) -> Result<String> {
        // In a real implementation, this would use proper cryptographic signing
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(self.private_key.as_bytes());
        let result = hasher.finalize();
        Ok(hex::encode(result))
    }
}
