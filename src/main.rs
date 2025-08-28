use clap::{Parser, Subcommand};
use namada_shielded_demo::{
    shielded_transaction::ShieldedTransaction,
    merkle_tree::MerkleTree,
    commitment::CommitmentScheme,
    zk_proof::ZeroKnowledgeProof,
    wallet::Wallet,
    error::ShieldedError,
};
use tracing::{info, error};

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
        #[arg(short, long, default_value = "false")]
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
}

#[tokio::main]
async fn main() -> Result<(), ShieldedError> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting Namada Shielded Transaction Demo");
    
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
            
            println!("Created transaction: {}", transaction.id);
            println!("Type: {}", if shielded { "Shielded" } else { "Public" });
            println!("Amount: {}", amount);
        }
        
        Commands::VerifyTransaction { transaction_id } => {
            let is_valid = ShieldedTransaction::verify(&transaction_id)?;
            println!("Transaction {} is {}", transaction_id, if is_valid { "valid" } else { "invalid" });
        }
        
        Commands::GenerateProof { transaction_id } => {
            let proof = ZeroKnowledgeProof::generate(&transaction_id)?;
            println!("Generated ZK proof for transaction: {}", transaction_id);
            println!("Proof: {}", proof);
        }
        
        Commands::Balance { wallet } => {
            // In a real implementation, this would query the blockchain
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
            let tree = MerkleTree::new();
            println!("Merkle Tree Root: {}", tree.root());
            println!("Tree Height: {}", tree.height());
            println!("Number of leaves: {}", tree.leaf_count());
        }
    }
    
    Ok(())
}
