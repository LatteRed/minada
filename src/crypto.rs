use crate::error::Result;
use rand::Rng;
use sha2::{Sha256, Digest};
use hex;

pub fn generate_keypair() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let private_key: [u8; 32] = rng.gen();
    let public_key = derive_public_key(&private_key)?;
    
    Ok((hex::encode(public_key), hex::encode(private_key)))
}

pub fn derive_public_key(private_key: &[u8; 32]) -> Result<[u8; 32]> {
    // In a real implementation, this would use proper elliptic curve operations
    let mut hasher = Sha256::new();
    hasher.update(private_key);
    let result = hasher.finalize();
    
    let mut public_key = [0u8; 32];
    public_key.copy_from_slice(&result);
    Ok(public_key)
}

pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

pub fn generate_nonce() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 32];
    rng.fill(&mut nonce);
    nonce
}

pub fn verify_signature(message: &[u8], signature: &str, public_key: &str) -> Result<bool> {
    // In a real implementation, this would verify the signature properly
    let expected_signature = {
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(hex::decode(public_key).unwrap_or_default());
        hex::encode(hasher.finalize())
    };
    
    Ok(signature == expected_signature)
}
