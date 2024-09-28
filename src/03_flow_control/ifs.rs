use colored::*;

/* 
    If:
    - Controls whether or not to execute a certain block of code, based on conditions the program evaluates during runtime;
    - Codes executes line-by-line;
    - Conditions can change control flow:
        "if"
        "else if"
        "else"

    - Try always include "else", unless there truly is no alternative case.
 */

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
