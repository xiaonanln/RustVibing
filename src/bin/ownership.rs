fn main() {
    // === OWNERSHIP ===
    // Every value in Rust has exactly ONE owner. When the owner goes out of scope, the value is dropped (freed).
    // No GC, no manual free — deterministic destruction via scope.

    let s1 = String::from("hello"); // s1 owns this heap-allocated String
    let s2 = s1;                    // Ownership MOVES to s2. s1 is now invalid.

    // println!("{}", s1);  // Uncommenting this = compile error: "value used after move"
    println!("s2 = {}", s2);

    // WHY? Rust prevents double-free. If both s1 and s2 owned the same heap data,
    // both would try to free it when they go out of scope. Move semantics solve this.

    // Note: primitive types (i32, bool, f64, char) implement `Copy` — they're
    // stack-allocated and cheap to duplicate, so they copy instead of move:
    let x = 42;
    let y = x; // x is copied, not moved
    println!("x = {}, y = {}", x, y); // both valid

    // === BORROWING (REFERENCES) ===
    // What if you want to use a value without taking ownership? Borrow it with `&`.

    let s3 = String::from("world");
    print_length(&s3); // Pass a reference — s3 is BORROWED, not moved
    println!("s3 is still valid: {}", s3); // s3 still usable

    // === MUTABLE REFERENCES ===
    // `&` is an immutable borrow. `&mut` lets you modify the borrowed value.
    // Key rule: you can have EITHER one &mut OR any number of & — never both at once.
    // This prevents data races at compile time.

    let mut s4 = String::from("hello");
    add_world(&mut s4);
    println!("s4 = {}", s4);
}

fn print_length(s: &String) {
    // `s` borrows the String — it can read but not modify it
    println!("length of '{}' = {}", s, s.len());
} // s goes out of scope, but since it doesn't own the String, nothing is freed

fn add_world(s: &mut String) {
    // `s` is a mutable reference — it can modify the borrowed value
    s.push_str(", world!");
}
