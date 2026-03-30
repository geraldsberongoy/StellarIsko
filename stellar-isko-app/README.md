# StellarIsko Frontend 🌐

The frontend for **StellarIsko**, a decentralized university document request platform. This dashboard allows students to connect their Stellar wallet (Freighter), request academic documents with instant USDC payments, and view their Soulbound Token (SBT) credentials.

## 🚀 Features
- **Wallet Connectivity:** Integrated with Freighter for secure authentication and transaction signing.
- **Document Catalog:** Browse and select academic documents to request.
- **Credential Dashboard:** A dedicated space for students to view their permanent, verifiable academic credentials (SBTs).
- **Admin Portal:** Interface for the University Registrar to verify payments and mint credentials.

## 🛠 Tech Stack
- **Framework:** [Next.js](https://nextjs.org/) (App Router)
- **Styling:** [Tailwind CSS](https://tailwindcss.com/)
- **Wallet:** [@stellar/freighter-api](https://www.npmjs.com/package/@stellar/freighter-api)
- **Blockchain API:** [stellar-sdk](https://www.npmjs.com/package/stellar-sdk)

## 🏗 Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Freighter Wallet](https://freighter.app/) installed as a browser extension (set to Testnet)

### Installation
```bash
npm install
```

### Development
Run the development server:
```bash
npm run dev
```
Open [http://localhost:3000](http://localhost:3000) to view the app.

## 📜 Deployment
This application can be deployed to Vercel or any other Next.js-compatible hosting provider.

```bash
npm run build
```

## 📜 License
MIT License
