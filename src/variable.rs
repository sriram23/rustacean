use chrono::prelude::*;
pub fn run () {
    let name = "Sriram";
    let mut age = 26;
    println!("Hardcoded age is: {}", age);
    let current_year = Utc::now().year();
    age = current_year - 1996;
    println!("Calculated Age is: {}", age);
    println!("{} is {} years old", name, age);
}