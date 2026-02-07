# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Purpose

This is a learn-Rust-by-doing project. The user is learning Rust interactively with Claude Code as a tutor. Prioritize clear explanations of Rust concepts (ownership, borrowing, lifetimes, traits, error handling, etc.) when writing or modifying code.

## Setup

The Rust toolchain is not yet installed. To get started:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

The repo does not yet have a Cargo project. Initialize one with:
```
cargo init
```

## Common Commands

```bash
cargo build              # Compile the project
cargo run                # Build and run
cargo test               # Run all tests
cargo test test_name     # Run a single test by name
cargo clippy             # Lint (requires: rustup component add clippy)
cargo fmt                # Auto-format code
cargo fmt -- --check     # Check formatting without modifying
cargo doc --open         # Generate and open documentation
```

## Teaching Approach

When helping the user learn Rust:
- Explain **why** Rust does things (e.g., why the borrow checker rejects something), not just how to fix it
- Introduce concepts incrementally — don't overwhelm with advanced features too early
- Use compiler errors as teaching moments; walk through what the error means
- Write idiomatic Rust from the start (use `Result`/`Option` instead of panicking, prefer iterators over manual loops, etc.)
- Add inline comments on new concepts the first time they appear in code
