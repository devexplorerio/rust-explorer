use colored::*;

fn subs(a: i32, b: i32) -> i32 {
    a - b
}

pub fn basic_arithmetic() {
    println!("\n{}", "basic_arithmetic fn:".bold().blue());

    let sum = 2 + 2;
    let sub = 10 - 5;
    let div = 10 / 2;
    let mult = 5 * 5;
    let subs = subs(8, 3);

    println!("sum {}, sub {}, div {}, mult {}, subs {}", sum, sub, div, mult, subs);
}
