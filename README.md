# 💸 Payment Splitter Factory

A **smart contract factory** that creates **payment splitter contracts** to automatically distribute token balances between multiple recipients based on predefined percentages.

This solution is already live across multiple chains:

- 🟡 **Stellar (Soroban)** – `payment_splitter`
- 🌀 **Polkadot (Ink!)** – `split_contract`
- 🟣 **zkSync** – `zk-splitter`
- 🔵 **Mantle** – `mantle-splitter`

Use cases include:
- Splitting crypto grants or revenue
- Royalties for music, NFTs, or content
- Real estate income sharing (RWA)
- Revenue sharing among co-founders, investors, or DAOs

---

## 🏭 How It Works

### ✅ Factory Pattern
- A **Factory Contract** deploys multiple **Splitter Contracts** with unique configurations.
- Each Splitter Contract is **immutable** and **fully on-chain**.

### 🔁 Splitter Logic
Each deployed splitter contract:
1. Accepts token transfers directly.
2. Stores a list of recipients and their share (in basis points).
3. When `pay()` is called:
   - Reads the current balance of a specific token.
   - Splits the full balance.
   - Sends funds to each recipient accordingly.

No manual calculations, no spreadsheets, no custodians.

---

## ✨ Features

- 🔐 Trustless and transparent
- 🚀 Works with any standard token (USDC, wrapped assets, etc.)
- 📊 View functions to read:
  - Current token balance
  - Recipient list and allocations
- 🧩 Chain-agnostic architecture
- 🔁 Portable across Stellar, zkSync, Mantle, and Polkadot

---

## 🌐 Multi-Chain Implementation

| Chain        | Contract Name     | Language       |
|--------------|-------------------|----------------|
| Stellar      | `payment_splitter`| Rust (Soroban) |
| Polkadot     | `split_contract`  | Ink!           |
| zkSync       | `zk-splitter`     | Solidity       |
| Mantle       | `mantle-splitter` | Solidity       |

All implementations follow the same factory/deployer model, enabling a shared frontend and common user experience across ecosystems.

---

## 📦 Use Cases

| Use Case                   | Example                                                                 |
|----------------------------|-------------------------------------------------------------------------|
| Hackathon Teams            | Split grant or prize winnings among members                            |
| Real Estate / RWA          | Distribute rental income to multiple owners                            |
| Royalties for Creators     | Stream earnings to artists, producers, and collaborators automatically |
| Startup Revenue Sharing    | Pay investors and co-founders based on pre-agreed splits               |
| DAOs / Collectives         | Reward contributors based on contribution levels                       |

---

## 📄 Interface (Stellar Version)

```rust
pub fn pay(env: Env, token: Address);
pub fn get_balance(env: Env, token: Address) -> i128;
pub fn get_recipients(env: Env) -> Vec<(Address, u32)>;
