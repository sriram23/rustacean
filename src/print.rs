// Public function to be called from the main function
pub fn run () {
    // A simple print statement to the console
    println!("Hello World! This is my first Rust code!");

    // Printing an integer using formatting
    println!("{}", 23);

    // Printing a string with formatting
    println!("My name is {}", "Sriram");

    // Positional arguments
    println!("I live in {0},{1}. {0} is in the state of {2}.","Coimbatore", "India", "Tamilnadu");

    // Named arguments
    println!("I speak {language}, as I'm living in {state}.",language="Tamil", state="Tamilnadu");

    // Placeholder for numbers
    println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}, Decimal: {}",23, 23, 23, 23);

    // Placeholder for debug
    println!("{:?}", ("Sriram", 26, true));

    // Basic calculations
    println!("23 * 7 * 96 = {}", 23*7*96);
}