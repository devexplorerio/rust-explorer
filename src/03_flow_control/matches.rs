use colored::*;

pub fn match_dest() {
    println!("\n{}", "match_dest fn:".bold().blue());

    let dest = "New York";
    
    match dest {
        "New York" => println!("We're heading to New York"),
        "Paris" => println!("We're heading to Paris"),
        "Tokyo" => println!("We're heading to Tokyo"),
        _ => println!("We're heading anywhere else")
    }
}