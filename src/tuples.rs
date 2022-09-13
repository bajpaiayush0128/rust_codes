// Max 12 elements can be there in tuple

pub fn run() {
    let person: (&str, &str, i8) = ("Musk", "Mars", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
