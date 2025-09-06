# 📚 Advanced Orderbook

An advanced, modular **Rust-based orderbook engine** with a CLI interface.  
Designed for high-performance trading simulations, decentralized exchanges, and educational exploration of market mechanics.  

---

## 🚀 Features

- 📈 **Core Library (`orderbook-core`)**
  - Modular and reusable
  - Efficient bid/ask matching
  - Order types: limit, market
  - Trade execution & partial fills
  - Extensible for new matching algorithms

- 💻 **CLI Tool (`orderbook-cli`)**
  - Interactive terminal commands
  - Load, submit, and cancel orders
  - Display current orderbook state
  - Simulate trades

- 🛠️ **Workspace Design**
  - `orderbook-core` → reusable library crate
  - `orderbook-cli` → binary frontend depending on the core

---

## 🗂️ Project Structure

