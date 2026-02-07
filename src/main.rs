// Welcome to RustVibing - Learning Rust by Vibing! 🦀
// This is a beginner-friendly Rust project with examples to help you learn.

fn main() {
    println!("🦀 Welcome to RustVibing - Let's Learn Rust! 🦀\n");
    
    // Example 1: Variables and Mutability
    basic_variables();
    
    // Example 2: Functions
    greet_learner("Rust Enthusiast");
    
    // Example 3: Control Flow
    demonstrate_control_flow();
    
    // Example 4: Ownership Basics
    demonstrate_ownership();
    
    println!("\n✨ Keep vibing with Rust! ✨");
}

// Demonstrates variables and mutability
fn basic_variables() {
    println!("\n📝 Example 1: Variables and Mutability");
    
    // Immutable variable (default in Rust)
    let x = 5;
    println!("  Immutable x: {}", x);
    
    // Mutable variable (requires 'mut' keyword)
    let mut y = 10;
    println!("  Mutable y before: {}", y);
    y = 15;
    println!("  Mutable y after: {}", y);
    
    // Constants (always immutable, type must be annotated)
    const MAX_POINTS: u32 = 100_000;
    println!("  Constant MAX_POINTS: {}", MAX_POINTS);
}

// Demonstrates function parameters and return values
fn greet_learner(name: &str) -> String {
    println!("\n👋 Example 2: Functions");
    let greeting = format!("  Hello, {}! Welcome to Rust!", name);
    println!("{}", greeting);
    greeting
}

// Demonstrates if/else and loops
fn demonstrate_control_flow() {
    println!("\n🔀 Example 3: Control Flow");
    
    // If/else
    let number = 7;
    if number < 5 {
        println!("  {} is less than 5", number);
    } else if number == 5 {
        println!("  {} is equal to 5", number);
    } else {
        println!("  {} is greater than 5", number);
    }
    
    // Loop with range
    print!("  Counting: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
}

// Demonstrates basic ownership concepts
fn demonstrate_ownership() {
    println!("\n🔑 Example 4: Ownership Basics");
    
    // String ownership
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // Clone to keep s1 valid
    println!("  s1: {}, s2: {}", s1, s2);
    
    // Borrowing with references
    let message = String::from("Hello");
    let len = calculate_length(&message);
    println!("  The length of '{}' is {}", message, len);
}

// Helper function demonstrating borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}
