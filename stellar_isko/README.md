# StellarIsko: Pay & Mint 🎓

**Problem:** At universities like the Polytechnic University of the Philippines (PUP), students requesting urgent academic documents face significant delays due to manual payment verification (often taking 1-3 business days) and difficulty in verifying physical documents.

**Solution:** StellarIsko modernizes this process using Soroban smart contracts. It enables instant stablecoin (USDC) settlements for document requests and automatically issues the requested document as a **Soulbound Token (SBT)**—a tamper-proof, instantly verifiable digital credential locked to the student's wallet.

## 🚀 Features
- **Instant Payments:** Use USDC to pay for document requests with immediate on-chain settlement.
- **Soulbound Credentials:** Securely mint non-transferable academic records directly to student wallets.
- **Tamper-Proof Verification:** Read-only verification for employers or third parties to confirm document authenticity.
- **Admin Dashboard:** Automated queue for Registrars to process paid requests and issue credentials.

## 🛠 Tech Stack
- **Smart Contract:** Soroban (Rust)
- **Frontend:** Next.js (App Router), Tailwind CSS
- **Wallet Integration:** Stellar Freighter API
- **Blockchain:** Stellar Testnet

## 🏗 Build & Test

### Prerequisites
- [Rust & Cargo](https://rustup.rs/)
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)

### Build the Contract
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests
```bash
cargo test
```

### Deploy to Testnet
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_isko.wasm \
  --source my-key \
  --network testnet
```

### Sample CLI Invocation

**Initialize the Contract:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- \
  initialize \
  --admin <ADMIN_ADDRESS> \
  --treasury <TREASURY_ADDRESS> \
  --payment_token <USDC_TOKEN_ID>
```

**Request and Pay:**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source student-key \
  --network testnet \
  -- \
  request_and_pay \
  --student <STUDENT_ADDRESS> \
  --amount 150000000
```

**Issue Soulbound Credential (Admin only):**
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source admin-key \
  --network testnet \
  -- \
  issue_soulbound_credential \
  --admin <ADMIN_ADDRESS> \
  --student <STUDENT_ADDRESS> \
  --doc_hash <32_BYTE_HASH>
```

## 📜 License
MIT License
