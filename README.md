# StellarIsko: Pay & Mint 🎓

**Problem:** At universities like the Polytechnic University of the Philippines (PUP), students requesting urgent academic documents face significant delays due to manual payment verification (often taking 1-3 business days) and difficulty in verifying physical documents.

**Solution:** StellarIsko modernizes this process using Soroban smart contracts. It enables instant stablecoin (USDC) settlements for document requests and automatically issues the requested document as a **Soulbound Token (SBT)**—a tamper-proof, instantly verifiable digital credential locked to the student's wallet.

---

## 📁 Project Structure

- **[`stellar_isko/`](./stellar_isko/):** Soroban smart contract (Rust).
- **[`stellar-isko-app/`](./stellar-isko-app/):** Next.js web application (Frontend).
- **[`docs/`](./docs/):** Technical documentation (PRD, Roadmap, Flowcharts).

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

## 🏗 Build & Setup

### Smart Contract (`stellar_isko/`)
1. **Prerequisites:** Rust, WASM target, Stellar CLI.
2. **Build:**
   ```bash
   cd stellar_isko
   cargo build --target wasm32-unknown-unknown --release
   ```
3. **Test:**
   ```bash
   cargo test
   ```

### Frontend (`stellar-isko-app/`)
1. **Install Dependencies:**
   ```bash
   cd stellar-isko-app
   npm install
   ```
2. **Run Development Server:**
   ```bash
   npm run dev
   ```

## 📅 Roadmap
- [x] Phase 1: Smart Contract Implementation
- [x] Phase 2: Frontend Dashboard & Wallet Integration
- [x] Phase 3: Testnet Deployment
- [ ] Phase 4: Mainnet Launch & University Integration

## 📜 License
MIT License
