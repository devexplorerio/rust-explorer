use colored::*;


/* 
    Funtion:
    - Function encapsulate functionality
    - Optionally accepts data
    - Optionally returns data
    - Utilized for code organization
    - Makes code easier to read
    - Can be executed by "calling" the function
    - Parameters determine what data a function can work with
 */

fn first()  {
    let first_name = "Jack";
    println!("First name: {}", first_name);
}

fn last() {
    let last_name = "Sparrow";
    println!("Last name: {}", last_name);
}

pub fn calling_fns() {
    println!("\n{}", "calling_fns fn:".bold().blue());

    first();
    last();
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn returning_fn() {
    println!("\n{}", "returning_fn fn:".bold().blue());

    let x = sum(1, 1);
    let y = sum(3, 0);
    let z = sum(x, 1);

    println!("x: {}, y: {}, z: {}", x, y, z);
}

// Scope Example
pub fn scope_fn() {
    println!("\n{}", "scope_fn fn:".bold().blue());

    let var_scope_fn = "Scope Fn";

    if true {
        let var_scope_if = "Scope If";
        
        println!("available only within if block scope: {}", var_scope_if);
        println!("available both within fn and if block scopes: {}", var_scope_fn);
    }

    println!("available within fn scope: {}", var_scope_fn);

} 