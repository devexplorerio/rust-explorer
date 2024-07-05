use colored::*;


/* 
    While use cases:
    - Continue repeating a block of code as long as a condition is true.
*/ 

pub fn while_diff() {
    println!("\n{}", "while_diff fn:".bold().blue());

    let mut a = 0;

    while a != 5 {
        println!("{:?}", a);
        a += 1;
    }
}

pub fn while_countdown() {
    println!("\n{}", "while_countdown fn:".bold().blue());

    let mut new_year_countdown = 10;

    while new_year_countdown > 0 {
        println!("{new_year_countdown}");
        new_year_countdown -= 1;
    }
}