use colored::*;

pub fn variable_immutable() {
    println!("\n{}", "variable_immutable fn:".bold().blue());

    let my_name = "Bill";
    println!("my_name variable is immutable: {}", my_name);
}

pub fn variable_mutable() {
    println!("\n{}", "variable_mutable fn:".bold().blue());
    
    let mut _my_name = "Bill";
    _my_name = "Steve";

    println!("my_name variable is mutable: {}", _my_name);
}