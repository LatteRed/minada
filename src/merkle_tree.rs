use crate::{error::Result, crypto::hash};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    pub root: String,
    pub height: usize,
    pub leaf_count: usize,
    pub leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self {
            root: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            height: 0,
            leaf_count: 0,
            leaves: Vec::new(),
        }
    }
    
    pub fn root(&self) -> String {
        self.root.clone()
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
    
    pub fn leaf_count(&self) -> usize {
        self.leaf_count
    }
    
    /// Add a leaf to the Merkle tree
    pub fn add_leaf(&mut self, data: &str) -> Result<()> {
        let leaf_hash = Self::hash_leaf(data);
        self.leaves.push(leaf_hash.clone());
        self.leaf_count += 1;
        
        // Recalculate the root
        self.root = Self::calculate_root(&self.leaves)?;
        self.height = Self::calculate_height(self.leaf_count);
        
        Ok(())
    }
    
    /// Generate a Merkle proof for a leaf
    pub fn generate_proof(&self, leaf_index: usize) -> Result<Vec<String>> {
        if leaf_index >= self.leaf_count {
            return Err(crate::error::ShieldedError::MerkleTreeError(
                "Leaf index out of bounds".to_string()
            ));
        }
        
        let mut proof = Vec::new();
        let mut current_index = leaf_index;
        let mut current_level = self.leaves.clone();
        
        while current_level.len() > 1 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            
            if sibling_index < current_level.len() {
                proof.push(current_level[sibling_index].clone());
            }
            
            // Move to parent level
            current_index /= 2;
            current_level = Self::hash_level(&current_level)?;
        }
        
        Ok(proof)
    }
    
    /// Verify a Merkle proof
    pub fn verify_proof(&self, leaf_data: &str, proof: &[String], leaf_index: usize) -> Result<bool> {
        let leaf_hash = Self::hash_leaf(leaf_data);
        let mut current_hash = leaf_hash;
        let mut current_index = leaf_index;
        
        for sibling_hash in proof {
            let parent_hash = if current_index % 2 == 0 {
                // Current is left child
                Self::hash_pair(&current_hash, sibling_hash)?
            } else {
                // Current is right child
                Self::hash_pair(sibling_hash, &current_hash)?
            };
            
            current_hash = parent_hash;
            current_index /= 2;
        }
        
        Ok(current_hash == self.root)
    }
    
    /// Hash a leaf node
    fn hash_leaf(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"leaf:");
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Hash a pair of nodes
    fn hash_pair(left: &str, right: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(b"node:");
        hasher.update(left.as_bytes());
        hasher.update(right.as_bytes());
        Ok(hex::encode(hasher.finalize()))
    }
    
    /// Hash a level of the tree
    fn hash_level(level: &[String]) -> Result<Vec<String>> {
        let mut next_level = Vec::new();
        
        for i in (0..level.len()).step_by(2) {
            if i + 1 < level.len() {
                next_level.push(Self::hash_pair(&level[i], &level[i + 1])?);
            } else {
                next_level.push(level[i].clone());
            }
        }
        
        Ok(next_level)
    }
    
    /// Calculate the root hash from leaves
    fn calculate_root(leaves: &[String]) -> Result<String> {
        if leaves.is_empty() {
            return Ok("0000000000000000000000000000000000000000000000000000000000000000".to_string());
        }
        
        let mut current_level = leaves.to_vec();
        
        while current_level.len() > 1 {
            current_level = Self::hash_level(&current_level)?;
        }
        
        Ok(current_level[0].clone())
    }
    
    /// Calculate the height of the tree
    fn calculate_height(leaf_count: usize) -> usize {
        if leaf_count == 0 {
            return 0;
        }
        
        let mut height = 0;
        let mut nodes = leaf_count;
        
        while nodes > 1 {
            nodes = (nodes + 1) / 2;
            height += 1;
        }
        
        height
    }
}
