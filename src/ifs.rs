use colored::*;

pub fn if_else() {
    println!("\n{}", "if_else fn:".bold().blue());

    let a = 99;

    if a > 99 {
        println!("Big number");
    } else {
        println!("Small number");
    }
}

pub fn if_elseif_else() {
    println!("\n{}", "if_elseif_else fn:".bold().blue());

    let a = 999;

    if a > 200 {
        println!("Huge number");
    } else if a > 99 {
        println!("Big number");
    } else {
        println!("Small number");
    }
}
