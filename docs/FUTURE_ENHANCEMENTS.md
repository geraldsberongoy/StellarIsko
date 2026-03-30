# 🚀 StellarIsko: Future Enhancements

This document outlines the long-term vision and planned features for the **StellarIsko** ecosystem beyond the initial MVP. These enhancements focus on scalability, user accessibility, and deeper integration with the Stellar network.

---

### 1. 🤖 AI-Powered Document Processing
- **Automated Validation:** Integrate a lightweight AI agent (using tools like OpenAI or local LLMs) to scan and parse uploaded physical documents from the Registrar.
- **Auto-Hashing:** The AI will automatically extract key metadata (Student ID, Semester, GPA) to generate the 32-byte `doc_hash` for the smart contract, reducing manual data entry for administrative staff.

### 2. ⚓ Local Anchor Integration (PHP/USDC)
- **Direct Fiat On-Ramp:** Partner with local Stellar anchors in the Philippines (e.g., Apper or other PHP-based anchors) to allow students to pay using **GCash, Maya, or Bank Transfer** directly through the dApp.
- **Stablecoin Diversity:** Enable payments in **PHPC** (Philippine Peso Stablecoin) alongside USDC and XLM to minimize currency conversion friction for local students.

### 3. 📦 Decentralized Metadata Storage (IPFS/Arweave)
- **Beyond the Hash:** While the contract stores the "proof" (hash), the actual encrypted PDF of the document will be stored on **IPFS** or **Arweave**.
- **Privacy-First:** Use **Stellar's encryption capabilities** to ensure that only the student (owner) and authorized employers (via a temporary decryption key) can view the actual document content.

### 4. 💸 DeFi Composability (Scholarship Staking)
- **Yield-Bearing Scholarships:** Allow alumni or NGOs to "stake" XLM/USDC into a scholarship pool.
- **Automated Disbursement:** The Soroban contract can automatically release these funds to students who have a verified "Dean's List" SBT minted to their wallet for the current semester.

### 5. 📱 Mobile-First Experience & Low-Connectivity Support
- **Soroban-Powered Mobile App:** A dedicated Flutter or React Native app with a built-in "lite" wallet.
- **Offline Verification:** Enable QR-code based verification that works even with intermittent internet, allowing employers in remote areas to verify a student's SBT offline via a signed message from the student's wallet.

### 6. 🏛️ DAO Governance for Universities
- **Registry of Registrars:** Move the `Admin` role from a single wallet to a **Multisig or DAO**.
- **Collaborative Issuance:** Multiple university departments (Registrar, Accounting, Dean's Office) must sign off on a "Graduation" SBT before it is issued to the student, ensuring zero administrative errors.

### 🌉 Multi-Chain Interoperability
- **Bridge to Ethereum/Solana:** Use **Stellar's bridge technology** to allow students to showcase their verified academic achievements on other chains (e.g., as part of a Web3 Resume or LinkedIn profile integration).

---

### 📅 Roadmap Phase 2 & 3
| Phase | Focus | Key Feature |
| :--- | :--- | :--- |
| **Phase 2** | Accessibility | GCash/PHP Anchor Integration |
| **Phase 2** | Security | Multisig Admin Controls |
| **Phase 3** | Ecosystem | Scholarship Staking & DeFi |
| **Phase 3** | AI | Automated Document Parsing |
