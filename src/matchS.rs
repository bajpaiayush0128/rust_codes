pub fn run() {
    let x = 1;
    match x {
        1 => println!("Value of x is 1"),
        2 => println!("Value of x is 2"),
        _ => println!("Value of x is invalid"),
    }

    let a = true;
    let b = true;

    // if a && b {
    //     println!("X and Y are true.");
    // } else if !a && !b {
    //     println!("X and Y are false.");
    // } else if a && !b {
    //     println!("X is true, Y is false.");
    // } else if !a && b {
    //     println!("X is false, Y is true.");
    // }

    match (a, b) {
        (true, true) => println!("X and Y are true."),
        (false, false) => println!("X and Y are false."),
        (true, false) => println!("X is true, Y is false."),
        (false, true) => println!("X is false, Y is true."),
        // _ => println!("Invalid combination"), // it gives warning because it is not needed, try removing any one of above lines
    }
}
