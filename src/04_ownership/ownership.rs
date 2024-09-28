
use colored::*;


/* 
    Ownership

    - variables are responsible for freeing their own resources
    - rules:
        - every value is "owned" by one, and only one, variable at a time
        - when the owing variable goes out of scope, the value is dropped
    - difference between stack (ex: interger) and heap (ex: string) data:
        - for stack data: rust make a copy to the new variable  
        - for heap data: rust move the ownership to the new variable 

    - when a function takes ownership of a value, it becomes responsible for that value and its associated data. 
    - the function can then do whatever it wants with the data, including modify or delete it. If the function doesn't need the data anymore, it will be dropped.
*/


pub fn ownership_copy() {
    println!("\n{}", "ownership_copy fn:".bold().blue());

    let rocket_fuel = 1;

    process_fuel_copy(rocket_fuel);

    println!("rocket fuel: {}", rocket_fuel);
}

fn process_fuel_copy(mut rocket_fuel: i32) {
    rocket_fuel += 1;

    println!("processing rocket fuel {}...", rocket_fuel);
}



pub fn ownership_move() {
    println!("\n{}", "ownership_move fn:".bold().blue());

    let rocket_fuel = String::from("RP-1");

    /* 
        if we implement like this, the compiler will give us an error (uncomment to see the error)
        process_fuel_move(rocket_fuel);
        println!("new rocket fuel: {}", rocket_fuel); // error: "value borrowed here after move"
    */

    let new_rocket_fuel = process_fuel_move(rocket_fuel); // we have to return the new value because rocket_fuel was dropped
    println!("new rocket fuel: {}", new_rocket_fuel);
}

fn process_fuel_move(rocket_fuel: String) -> String {
    println!("processing rocket fuel {}...", rocket_fuel);

    let new_rocket_fuel = String::from("LNG");
    new_rocket_fuel
}

