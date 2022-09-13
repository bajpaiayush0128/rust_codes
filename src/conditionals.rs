pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let knows_age_of_person = true;

    // If-else
    if age >= 21 && check_id || knows_age_of_person {
        println!("Bartender: What would you like to have?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave!");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shrthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age: {}", is_of_age);
}
