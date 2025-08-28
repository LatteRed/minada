use clap::{Parser, Subcommand};
use namada_shielded_demo::{
    shielded_transaction::ShieldedTransaction,
    merkle_tree::MerkleTree,
    commitment::CommitmentScheme,
    zk_proof::ZeroKnowledgeProof,
    wallet::Wallet,
    error::ShieldedError,
    storage::StorageData,
};
use tracing::info;

#[derive(Parser)]
#[command(name = "namada-shielded-demo")]
#[command(about = "A demonstration of shielded transactions and zero-knowledge proofs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet
    CreateWallet {
        #[arg(short, long)]
        name: String,
    },
    /// Create a shielded transaction
    CreateTransaction {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long)]
        shielded: bool,
    },
    /// Verify a transaction
    VerifyTransaction {
        #[arg(short, long)]
        transaction_id: String,
    },
    /// Generate a zero-knowledge proof
    GenerateProof {
        #[arg(short, long)]
        transaction_id: String,
    },
    /// Show wallet balance
    Balance {
        #[arg(short, long)]
        wallet: String,
    },
    /// Demonstrate commitment scheme
    DemonstrateCommitment {
        #[arg(short, long)]
        amount: u64,
    },
    /// Show Merkle tree state
    ShowMerkleTree,
    /// List all stored transactions
    ListTransactions,
    /// Clear all stored data
    ClearStorage,
}

#[tokio::main]
async fn main() -> Result<(), ShieldedError> {
    // Set up logging for the demo
    tracing_subscriber::fmt::init();
    
    info!("Starting Namada Shielded Transaction Demo");
    
    // Load existing data from storage
    let mut storage = StorageData::load()?;
    info!("Loaded {} transactions from storage", storage.get_all_transactions().len());
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::CreateWallet { name } => {
            let wallet = Wallet::new(&name)?;
            println!("Created wallet: {}", wallet.address);
            println!("Public key: {}", wallet.public_key);
        }
        
        Commands::CreateTransaction { from, to, amount, shielded } => {
            let transaction = if shielded {
                ShieldedTransaction::create_shielded(&from, &to, amount)?
            } else {
                ShieldedTransaction::create_public(&from, &to, amount)?
            };
            
            // Store the transaction persistently
            storage.add_transaction(transaction.clone())?;
            
            println!("Created transaction: {}", transaction.id);
            println!("Type: {}", if shielded { "Shielded" } else { "Public" });
            println!("Amount: {}", amount);
            println!("Transaction saved to persistent storage!");
        }
        
        Commands::VerifyTransaction { transaction_id } => {
            // Check if transaction exists in persistent storage
            if let Some(transaction) = storage.get_transaction(&transaction_id) {
                println!("Transaction {} found in persistent storage", transaction_id);
                println!("From: {} -> To: {}", transaction.from, transaction.to);
                println!("Amount: {}, Type: {:?}", transaction.amount, transaction.transaction_type);
                println!("Status: {:?}", transaction.status);
                println!("Timestamp: {}", transaction.timestamp);
                
                // Also verify the transaction format
                let is_valid = ShieldedTransaction::verify(&transaction_id)?;
                println!("Transaction format is {}", if is_valid { "valid" } else { "invalid" });
            } else {
                println!("Transaction {} not found in persistent storage", transaction_id);
                println!("Checking transaction format only...");
                let is_valid = ShieldedTransaction::verify(&transaction_id)?;
                println!("Transaction format is {}", if is_valid { "valid" } else { "invalid" });
            }
        }
        
        Commands::GenerateProof { transaction_id } => {
            let proof = ZeroKnowledgeProof::generate(&transaction_id)?;
            println!("Generated ZK proof for transaction: {}", transaction_id);
            println!("Proof: {}", proof);
        }
        
        Commands::Balance { wallet } => {
            // TODO: In production, this would actually query the blockchain state
            println!("Balance for wallet {}: 1000 NAM (estimated)", wallet);
        }
        
        Commands::DemonstrateCommitment { amount } => {
            let commitment = CommitmentScheme::commit(amount)?;
            println!("Commitment for amount {}: {}", amount, commitment);
            
            let proof = CommitmentScheme::prove_knowledge(amount)?;
            println!("Knowledge proof: {}", proof);
            
            let is_valid = CommitmentScheme::verify_knowledge(&commitment, &proof)?;
            println!("Proof verification: {}", if is_valid { "valid" } else { "invalid" });
        }
        
        Commands::ShowMerkleTree => {
            // Rebuild Merkle tree from stored leaves
            let tree = storage.rebuild_merkle_tree();
            let transactions = storage.get_all_transactions();
            
            println!("=== Merkle Tree State ===");
            println!("Merkle Tree Root: {}", tree.root());
            println!("Tree Height: {}", tree.height());
            println!("Number of leaves: {}", tree.leaf_count());
            println!("Total transactions stored: {}", transactions.len());
            
            if !transactions.is_empty() {
                println!("\n=== Stored Transactions ===");
                for (id, transaction) in transactions.iter() {
                    println!("ID: {}", id);
                    println!("  From: {} -> To: {}", transaction.from, transaction.to);
                    println!("  Amount: {}, Type: {:?}", transaction.amount, transaction.transaction_type);
                    println!("  Status: {:?}", transaction.status);
                    println!();
                }
            }
        }
        
        Commands::ListTransactions => {
            let transactions = storage.get_all_transactions();
            
            if transactions.is_empty() {
                println!("No transactions stored yet.");
            } else {
                println!("=== All Stored Transactions ===");
                for (i, (id, transaction)) in transactions.iter().enumerate() {
                    println!("{}. Transaction ID: {}", i + 1, id);
                    println!("   From: {} -> To: {}", transaction.from, transaction.to);
                    println!("   Amount: {}, Type: {:?}", transaction.amount, transaction.transaction_type);
                    println!("   Status: {:?}", transaction.status);
                    println!("   Timestamp: {}", transaction.timestamp);
                    println!();
                }
            }
        }
        
        Commands::ClearStorage => {
            storage.clear()?;
            println!("All stored data has been cleared.");
            println!("Storage files have been reset.");
        }
    }
    
    Ok(())
}

