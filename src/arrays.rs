// Fixed length and same data types

// use std::mem -> this helps remove std:: from code

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get ingle value
    println!("{}", numbers[0]);

    // Get array length
    println!("Array lenght: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}
