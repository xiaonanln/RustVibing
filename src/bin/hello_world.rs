fn main() {
    // `let` creates a variable binding. In Rust, variables are immutable by default!
    let name = "Simon";

    // `{}` inside println! is a placeholder, similar to f-strings in Python
    println!("Hello, {}! Welcome to Rust.", name);

    // To make a variable mutable, use `let mut`
    let mut count = 1;
    println!("This is lesson #{}", count);

    count += 1; // This works because `count` is declared as `mut`
    println!("Next will be lesson #{}", count);
}
