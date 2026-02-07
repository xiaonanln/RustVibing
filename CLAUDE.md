# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Purpose

This is a learn-Rust-by-doing project. The user is learning Rust interactively with Claude Code as a tutor. Prioritize clear explanations of Rust concepts (ownership, borrowing, lifetimes, traits, error handling, etc.) when writing or modifying code.

## Setup

- Rust toolchain is installed (rustc 1.93.0, edition 2024)
- Cargo project is initialized

## Project Structure

Each lesson is a standalone binary in `src/bin/`:
```
src/bin/hello_world.rs   # Lesson 1: variables, mutability, println! macro
src/bin/ownership.rs     # Lesson 2: ownership, borrowing, references
```

## Common Commands

```bash
cargo run --bin <name>   # Run a specific lesson (e.g., cargo run --bin ownership)
cargo build              # Compile the project
cargo test               # Run all tests
cargo test test_name     # Run a single test by name
cargo clippy             # Lint (requires: rustup component add clippy)
cargo fmt                # Auto-format code
cargo fmt -- --check     # Check formatting without modifying
cargo doc --open         # Generate and open documentation
```

## User Profile

- **Experienced software engineer** — skip basic programming concepts (variables, loops, functions, etc.)
- Focus on what's unique to Rust: ownership, borrowing, lifetimes, traits, the type system, pattern matching, enums, error handling
- Move at a faster pace; no hand-holding needed

## Teaching Approach

When helping the user learn Rust:
- Explain **why** Rust does things (e.g., why the borrow checker rejects something), not just how to fix it
- Focus on Rust-specific concepts; skip general programming explanations
- Use compiler errors as teaching moments; walk through what the error means
- Write idiomatic Rust from the start (use `Result`/`Option` instead of panicking, prefer iterators over manual loops, etc.)
- Add inline comments on new concepts the first time they appear in code
