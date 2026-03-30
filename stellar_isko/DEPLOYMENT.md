# 🚀 StellarIsko: Deployment Guide

Follow these steps to deploy and initialize the **StellarIsko** smart contract on the Stellar Testnet.

### 1. Prerequisites
Ensure you have the Rust WASM target and Stellar CLI installed:
```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

### 2. Set Up Your Testnet Identity
Generate a new keypair and fund it with test XLM:
```bash
stellar keys generate --global my-key --network testnet
stellar keys address my-key
stellar keys fund my-key --network testnet
```

### 3. Build the Contract
Compile the Rust code into a WebAssembly (WASM) file:
```bash
cd stellar_isko
cargo build --target wasm32-unknown-unknown --release
```

### 4. Deploy the Contract
Upload the WASM file to the Stellar Testnet:
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_isko.wasm \
  --source my-key \
  --network testnet
```
**Important:** Copy and save the **Contract ID** (starts with `C...`) returned by this command.

### 5. Initialize the Contract
You must initialize the contract with the Admin, Treasury, and Payment Token (USDC) addresses.

**Example for Testnet Native XLM (as the payment token):**
Native XLM Contract ID: `CDLZFC3SYJYDZT7K67VZ75YJBMKBAV27Z6Y6Z6Z6Z6Z6Z6Z6Z6Z6Z6Z6`

```bash
stellar contract invoke \
  --id <YOUR_CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- \
  initialize \
  --admin <ADMIN_ADDRESS> \
  --treasury <TREASURY_ADDRESS> \
  --payment_token CDLZFC3SYJYDZT7K67VZ75YJBMKBAV27Z6Y6Z6Z6Z6Z6Z6Z6Z6Z6Z6Z6
```

### 6. Verify Deployment
Invoke the `verify_credential` function (should return `false` for a non-existent hash):
```bash
stellar contract invoke \
  --id <YOUR_CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- \
  verify_credential \
  --student <SOME_ADDRESS> \
  --doc_hash 0000000000000000000000000000000000000000000000000000000000000001
```

### 7. Monitor on Stellar Expert
Track your contract's activity on the testnet explorer:
`https://stellar.expert/explorer/testnet/contract/<YOUR_CONTRACT_ID>`
