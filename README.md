# Shielded Transaction Demo

A comprehensive demonstration of shielded transactions and zero-knowledge proofs inspired by Namada's privacy features. 

### 1. Shielded Transactions
- Hide transaction amounts from public view
- Conceal sender and recipient identities
- Maintain transaction validity through cryptographic proofs

### 2. Zero-Knowledge Proofs
- Prove transaction validity without revealing amounts
- Demonstrate balance conservation (inputs = outputs + fees)
- Range proofs for amount validation

### 3. Commitment Schemes
- Commit to amounts without revealing them
- Prove knowledge of committed values
- Support for range proofs

### 4. Merkle Trees
- Efficient transaction inclusion proofs
- Compact representation of transaction history
- Fast verification of transaction existence

## Installation

1. Ensure you have Rust installed (version 1.70 or later)
2. Clone this repository
3. Navigate to the project directory
4. Build the project:

```bash
cargo build --release
```

## Usage

The application provides a comprehensive CLI interface for demonstrating shielded transaction concepts:

### Create a Wallet
```bash
cargo run -- create-wallet --name "Alice"
```

### Create a Public Transaction
```bash
cargo run -- create-transaction --from "Alice" --to "Bob" --amount 100 --shielded false
```

### Create a Shielded Transaction
```bash
cargo run -- create-transaction --from "Alice" --to "Bob" --amount 100 --shielded true
```

### Verify a Transaction
```bash
cargo run -- verify-transaction --transaction-id "your_transaction_id"
```

### Generate Zero-Knowledge Proof
```bash
cargo run -- generate-proof --transaction-id "your_transaction_id"
```

### Demonstrate Commitment Scheme
```bash
cargo run -- demonstrate-commitment --amount 500
```

### Show Merkle Tree State
```bash
cargo run -- show-merkle-tree
```

### Check Wallet Balance
```bash
cargo run -- balance --wallet "Alice"
```
