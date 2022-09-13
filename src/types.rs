// Read primitve types, about bits in integers/floats, explicitly defining them, etc.
// Rust is statically typed lang, so it must know the types of all vars at compile time,
// however, the compiler can usually infer what type we want to use based on value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 324546353;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_active: bool = true; // also works without explicitly setting it

    // Get Boolean from expression
    let is_greater: bool = 10 < 5;

    // Character
    let a = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a, face));
}
