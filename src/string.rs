// Primitive str = immutable
// String = growable, heap-allocated ds

pub fn run() {
    let hello = "Hello"; // primitive
    let mut hello1 = String::from("Hello "); //String

    // Get length
    println!("Lengths are: {} and {}", hello.len(), hello1.len());

    // Below operations are not for primitive types
    hello1.push('W');
    hello1.push_str("orld");

    // Capacity in bytes
    println!("Capacity: {}", hello1.capacity());

    // Check if string is empty
    println!("Is Empty: {}", hello1.is_empty());

    // Contains
    println!("Contains 'World' {}", hello1.contains("World"));

    // Replace
    println!("Replace {}", hello1.replace("World", "Musk"));

    // Loop through string by whitespace
    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing(not just limited to strings)
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    // Nothing happens when it is correct but throws error in case of failure
    println!("{} and {}", hello, hello1);
}
