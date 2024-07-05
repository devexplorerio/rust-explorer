use colored::*;

pub fn enum_city () {
    println!("\n{}", "enum_city fn:".bold().blue());

    // const is written with all caps
    const CITY: &str = "New York";
    println!("City is {}", CITY);
}