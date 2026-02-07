# Rust Learning Progress

## Completed Lessons

### 1. Hello World (`src/bin/hello_world.rs`)
- Variables with `let`, mutability with `let mut`
- `println!` macro and string formatting with `{}`
- Type inference
- Functions vs macros (macros operate on tokens at compile time)

### 2. Ownership & Borrowing (`src/bin/ownership.rs`)
- Ownership: each value has one owner, dropped when owner goes out of scope
- Move semantics for heap types (`String`), `Copy` for stack types (`i32`, `bool`)
- Borrowing with `&` (shared/immutable) and `&mut` (exclusive/mutable)
- Borrow rules: one `&mut` XOR any number of `&`, never both
- `String` (owned, heap, growable) vs `&str` (borrowed fat pointer: pointer + length)
- Fat pointers vs thin pointers — `&str` and `&[T]` carry length metadata
- `Sized` vs `?Sized` types — `str` and `[T]` can't exist on the stack alone
- Trait objects (`&dyn Trait`) as fat pointers with vtable

## Up Next
- Structs, enums, and pattern matching
- Error handling (`Result`, `Option`, `?` operator)
- Lifetimes
- Traits and generics
- Iterators and closures
