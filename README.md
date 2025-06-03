# ECDSA Implementation Using secp256k1 in Rust

![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-stable-blue)

This repository contains an **ECDSA (Elliptic Curve Digital Signature Algorithm) implementation using the [`secp256k1`](https://docs.rs/secp256k1) library** in Rust. It demonstrates how to sign and verify messages using the `secp256k1` curve, which is widely used in blockchain technologies such as Bitcoin.

## ✨ Features

- Uses the battle-tested `secp256k1` library
- Key generation (private/public)
- Message signing
- Signature verification

## 📂 Project Structure

```
ECDSA/
├── src/
│   ├── main.rs        # Example: key generation, signing, verification
├── Cargo.toml         # Project metadata and dependencies
└── README.md 
```

## ⚙️ Quick Start

### Requirements
- Rust (version 1.65 or later)

### Clone the repository

```bash
git clone https://github.com/your-username/ecdsa-secp256k1.git
cd ecdsa-secp256k1
```

### Build the project

```Rust
cargo build --release
```

### Run the example

```Rust
cargo run
```

## 📚 About secp256k1
secp256k1 is a highly optimized C library for elliptic curve cryptography on the secp256k1 curve, used in Bitcoin and other cryptographic systems. This crate provides safe and idiomatic Rust bindings for it.

## 🛡️ Disclaimer
While this project uses a secure cryptographic library, it is provided for educational purposes only. Do not use it in production without a thorough security audit.

## 📃 License
This project is licensed under the MIT License. See the LICENSE file for details.

Made with ❤️ in Rust