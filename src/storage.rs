use crate::{error::Result, shielded_transaction::ShieldedTransaction, merkle_tree::MerkleTree};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const STORAGE_FILE: &str = "transactions.json";
const MERKLE_FILE: &str = "merkle_tree.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageData {
    pub transactions: HashMap<String, ShieldedTransaction>,
    pub merkle_leaves: Vec<String>,
}

impl StorageData {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
            merkle_leaves: Vec::new(),
        }
    }

    /// Load data from storage files
    pub fn load() -> Result<Self> {
        let mut data = Self::new();
        
        // Load transactions
        if Path::new(STORAGE_FILE).exists() {
            let content = fs::read_to_string(STORAGE_FILE)
                .map_err(|e| crate::error::ShieldedError::StorageError(format!("Failed to read transactions file: {}", e)))?;
            data.transactions = serde_json::from_str(&content)
                .map_err(|e| crate::error::ShieldedError::SerializationError(e))?;
        }
        
        // Load Merkle tree leaves
        if Path::new(MERKLE_FILE).exists() {
            let content = fs::read_to_string(MERKLE_FILE)
                .map_err(|e| crate::error::ShieldedError::StorageError(format!("Failed to read Merkle tree file: {}", e)))?;
            data.merkle_leaves = serde_json::from_str(&content)
                .map_err(|e| crate::error::ShieldedError::SerializationError(e))?;
        }
        
        Ok(data)
    }

    /// Save data to storage files
    pub fn save(&self) -> Result<()> {
        // Save transactions
        let transactions_json = serde_json::to_string_pretty(&self.transactions)
            .map_err(|e| crate::error::ShieldedError::SerializationError(e))?;
        fs::write(STORAGE_FILE, transactions_json)
            .map_err(|e| crate::error::ShieldedError::StorageError(format!("Failed to write transactions file: {}", e)))?;
        
        // Save Merkle tree leaves
        let merkle_json = serde_json::to_string_pretty(&self.merkle_leaves)
            .map_err(|e| crate::error::ShieldedError::SerializationError(e))?;
        fs::write(MERKLE_FILE, merkle_json)
            .map_err(|e| crate::error::ShieldedError::StorageError(format!("Failed to write Merkle tree file: {}", e)))?;
        
        Ok(())
    }

    /// Add a transaction to storage
    pub fn add_transaction(&mut self, transaction: ShieldedTransaction) -> Result<()> {
        let id = transaction.id.clone();
        self.transactions.insert(id.clone(), transaction);
        self.merkle_leaves.push(id);
        self.save()
    }

    /// Get a transaction by ID
    pub fn get_transaction(&self, id: &str) -> Option<&ShieldedTransaction> {
        self.transactions.get(id)
    }

    /// Get all transactions
    pub fn get_all_transactions(&self) -> &HashMap<String, ShieldedTransaction> {
        &self.transactions
    }

    /// Get Merkle tree leaves
    pub fn get_merkle_leaves(&self) -> &Vec<String> {
        &self.merkle_leaves
    }

    /// Rebuild Merkle tree from stored leaves
    pub fn rebuild_merkle_tree(&self) -> MerkleTree {
        let mut tree = MerkleTree::new();
        for leaf in &self.merkle_leaves {
            let _ = tree.add_leaf(leaf);
        }
        tree
    }

    /// Clear all stored data
    pub fn clear(&mut self) -> Result<()> {
        self.transactions.clear();
        self.merkle_leaves.clear();
        self.save()
    }
}

