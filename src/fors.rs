use colored::*;

pub fn for_basic() {
    println!("\n{}", "for_basic fn:".bold().blue());
    
    let message = ['h', 'e', 'l', 'l', 'o'];

    for letter in message {
        println!("{}", letter);
    }
    
}

pub fn for_index() {
    println!("\n{}", "for_index fn:".bold().blue());

    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, letter) in message.iter().enumerate() {
        println!("{} is letter {}", index, letter);
    }
}

pub fn for_break() {
    println!("\n{}", "for_break fn:".bold().blue());

    println!("break if letter == e");

    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &letter) in message.iter().enumerate() {
        println!("{} is letter {}", index, letter);
        if letter == 'e' {
            break;
        }
    }
}