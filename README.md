# 🦀 Rust Learning Journey

This repository contains my complete Rust learning journey — from basics to advanced projects.

---

## 📘 What is Rust?

Rust is a modern systems programming language focused on **speed**, **safety**, and **concurrency** — without using a garbage collector.

It is used to build fast and reliable software like operating systems, games, web servers, and more.

---

## ⚙️ Rust Installation Guide

### ✅ Install Rust (All Platforms)

Open your terminal or command prompt and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install:
- `rustc` → Rust compiler
- `cargo` → Build tool and dependency manager
- `rustup` → Toolchain manager

After installation, restart your terminal and check versions:

```bash
rustc --version
cargo --version
```

### 🧱 Install GCC (Required for Rust Compilation)
Rust needs a C compiler like GCC to compile some libraries.

- **Windows:**
  - Download and install [MinGW-w64](http://mingw-w64.org/)
  - During installation, choose:
    - Architecture: x86_64
    - Threads: posix
    - Exception: seh
  - Add the `bin` folder (e.g., `C:\Program Files\mingw-w64\...\bin`) to your System PATH

- **Linux (Ubuntu/Debian):**
  ```bash
  sudo apt update
  sudo apt install build-essential
  ```

- **macOS:**
  ```bash
  xcode-select --install
  ```

---

## 📂 Project Structure (Example)

```bash
Rust-Learning/
│
├── basics/                # Individual concept files
│   ├── hello.rs
│   ├── variables.rs
│   └── data_types.rs
│
├── projects/              # Full projects using Cargo
│   └── guess_game/
│       ├── src/
│       │   └── main.rs
│       └── Cargo.toml
│
└── README.md              # This file
```

---

## ▶️ How to Run Code

### 🔸 Run Single `.rs` File (in `basics/` folder)

```bash
cd basics
rustc <filename>.rs
./<filename>       # On Linux/Mac
dir\<filename>.exe # On Windows
```

### 🔸 Run Cargo Project (in `projects/` folder)

```bash
cd projects/<project-folder>
cargo build     # Compile
cargo run       # Build and run
```

To build in release mode:
```bash
cargo build --release
```

---

## 📚 What This Repo Will Cover

As I learn, I'll be covering all key Rust concepts:

- Variables and Data Types
- Strings, Arrays, Tuples
- Functions and Control Flow
- Ownership & Borrowing
- Structs and Enums
- Pattern Matching (`match`)
- Modules and Crates
- Error Handling
- File Handling
- Multithreading
- Real Projects using Cargo

---

## 🙌 Resources

- [Official Rust Website](https://www.rust-lang.org/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Rust Playground](https://play.rust-lang.org/)
