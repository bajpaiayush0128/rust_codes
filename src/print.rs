pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Musk", "Mars");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Musk", "Mars", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Musk",
        activity = "Golf"
    );

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug Trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
