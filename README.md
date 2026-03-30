# 🎓 StellarIsko: Instant Academic Credentials

**StellarIsko** is a decentralized document issuance and payment system for universities in Southeast Asia, starting with the Polytechnic University of the Philippines (PUP).

### 🔴 PROBLEM
A graduating student at the Polytechnic University of the Philippines (PUP) faces 1-3 day delays and significant manual friction when requesting urgent academic documents, often missing immediate job opportunities or scholarship deadlines due to slow payment verification and paper-based processing.

### 🔵 SOLUTION
StellarIsko solves this by using Soroban smart contracts to enable instant XLM/USDC settlements for document fees and automatically issuing requested records as tamper-proof **Soulbound Tokens (SBTs)** directly to the student's wallet, making academic credentials instantly verifiable by any employer globally.

---

### 🛠 STELLAR FEATURES USED
- **Soroban Smart Contracts:** Core logic for automated fee collection and permanent credential registry.
- **XLM / USDC Transfers:** Instant peer-to-university payment settlement.
- **Contract Events:** Real-time indexing of payment and minting actions for the dashboard.
- **Permanent Storage:** Persistent ledger entries for "Soulbound" document verification.

### 🎯 TARGET USERS
- **Iskolar ng Bayan (Students):** 70,000+ students at PUP seeking fast, digital-first academic services.
- **University Registrars:** Administrative staff needing to automate payment reconciliation and document issuance.
- **Employers/DAOs:** Third parties requiring instant, zero-trust verification of an applicant's educational background.

### 🏗 CORE FEATURE (MVP)
**Student Payment → Registrar Issuance → Permanent Verification**
1.  **User Action:** Student selects "Transcript of Records" (TOR) and pays 10 XLM via Freighter.
2.  **On-Chain Action:** Soroban contract transfers XLM to the Treasury and emits a `pay` event.
3.  **Result:** Registrar sees the paid request, clicks "Issue," and the contract anchors a 32-byte document hash to the student's wallet permanently.

---

### 🛠 Tech Stack                                     
**Smart Contract:** Soroban (Rust)
**Frontend:** Next.js (App Router), Tailwind CSS
**Wallet Integration:** Stellar Freighter API
**Blockchain:** Stellar Testnet                   

---    

### 📋 CONSTRAINTS
| Dimension | Selection |
| :--- | :--- |
| **Region** | SEA (Philippines) |
| **User Type** | Students, Registrars, Employers |
| **Complexity** | Soroban required, Web App |
| **Theme** | Education → Credential Verification & Payments |

---

### 🌐 DEPLOYED CONTRACTS (TESTNET)
| Component | Identifier |
| :--- | :--- |
| **Contract ID** | `CBW23F5HWT5HHDLGMA4U4NGPGOS6B6K5HCLWLLH5BZOXFOKU4VPV35OK` |
| **Admin Key** | `GD67NPG7TKJDE5HEHSPWS3YAWYNHWTLWRSQMTO4NQOVSZAEFPICO3HYG` |
| **Treasury Key** | `GBRLGRWUJXJSHJDZQ4OH2SDH7ROF7EWAHI4ZIQM2E6TMONH7IG4P7QKL` |
| **Payment Token (XLM Native)** | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` |

🔍 **Explorer:** [Stellar Expert - Testnet](https://stellar.expert/explorer/testnet/contract/CBW23F5HWT5HHDLGMA4U4NGPGOS6B6K5HCLWLLH5BZOXFOKU4VPV35OK)

---

### 🧪 AUTOMATED TESTS
The contract includes exactly **3 comprehensive tests** to ensure security and reliability:

1.  **`test_end_to_end_flow` (Happy Path):** Validates the full user journey—Student payment, Registrar issuance, and final Credential verification.
2.  **`test_unauthorized_minting` (Edge Case):** Ensures that only the authorized Admin can issue credentials. Any attempt by a non-admin wallet triggers a secure `Auth` failure.
3.  **`test_state_verification` (Integrity Check):** Confirm that the contract storage correctly reflects the "Soulbound" status before and after issuance.

Run them with: `cd stellar_isko && cargo test`

---

### 🚀 BUILD & TEST GUIDE

#### Prerequisites
- Rust toolchain (`wasm32-unknown-unknown`)
- Stellar CLI (v25.2.0 or higher)

#### 1. Smart Contract (`stellar_isko/`)
```bash
cd stellar_isko
# Build optimized WASM
stellar contract build --optimize
# Run local tests
cargo test
```

#### 2. Sample CLI Invocations
**Register/Initialize Contract:**
```bash
stellar contract invoke --id CBW23F5HWT5HHDLGMA4U4NGPGOS6B6K5HCLWLLH5BZOXFOKU4VPV35OK --network testnet --source my-key -- initialize --admin <ADMIN_ADDR> --treasury <TREASURY_ADDR> --payment_token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
```

**Verify a Student Credential:**
```bash
stellar contract invoke --id CBW23F5HWT5HHDLGMA4U4NGPGOS6B6K5HCLWLLH5BZOXFOKU4VPV35OK --network testnet --source any-key -- verify_credential --student <STUDENT_ADDR> --doc_hash 0000000000000000000000000000000000000000000000000000000000000001
```

---

### 🚀 BUILD & TEST GUIDE
...

**Future Vision:** For a detailed look at where we're headed (AI, DeFi, and Local Anchors), see [docs/FUTURE_ENHANCEMENTS.md](./docs/FUTURE_ENHANCEMENTS.md).

---

### 📜 LICENSE
MIT License
