# Photographer Portfolio Website 📸✨

A decentralized portfolio website for photographers built using the Stellar blockchain and Soroban smart contracts. Submitted as part of MP's biggest Web 3.0 Build-a-thon.

## 🚀 Overview

This platform allows photographers to:
- 📤 Upload their best shots
- 💰 Receive tips from visitors in a decentralized way
- 🔍 Let users view and appreciate their work on-chain

## 🔗 Built With

- 🌐 **Stellar Blockchain** + **Soroban Smart Contracts**
- 🦀 **Rust** for writing smart contracts
- 📁 **IPFS (optional)** for decentralized image hosting
- 🧰 **VS Code**, **Git**, and **CLI tools**

## 📂 Project Structure

Photographer-Portfolio-Website/ ├── contracts/ │ └── hello world/ │ ├── Cargo.toml │ ├── Makefile │ └── src/ │ ├── lib.rs # Main smart contract logic │ └── test.rs # Unit tests ├── Cargo.toml ├── Cargo.lock └── README.md


## 📜 Smart Contract Functions

- `upload_photo()`: Store a new photo entry
- `view_photo()`: Retrieve photo information
- `tip_photo()`: Allow users to tip the photographer

## 🛠️ How to Build

```bash
# Navigate to the contract directory
cd contracts/hello\ world/

# Build using Soroban CLI
stellar contract build

