pub fn run() {
    let name = "Musk";
    let mut age = 37; // mut keyword is used to make the variable mutable
    println!("My name is {} and I'm {} years old.", name, age);
    age = 38;
    println!("My name is {} and I'm {} years old.", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Musk", 37);
    println!("{} is {} years old.", my_name, my_age);
}
