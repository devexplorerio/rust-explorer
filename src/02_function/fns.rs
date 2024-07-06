use colored::*;

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
