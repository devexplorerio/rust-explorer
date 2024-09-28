use colored::*;

/* 
    References:
    - mutable reference:
        let ref = &mut var;
    - immutable reference:
        let ref = &var;

    Note: You can only create a single mutable reference to a variable within a particular scope.
    If you're only working with regular immutable references, then you can create as many of those as you want pointing to the same variable. 
    The restriction comes in when you try to create references in addition to the one allowed mutable reference. 
    Even if those additional references are immutable, it creates the potential for problems and the Rust compiler will not allow it.
*/


/* 
    Borrowing
    - A borrowed reference allow you to access data without taking ownership of it
    - create references using the borrow operator: &
*/
pub fn borrowing() {
    println!("\n{}", "ownership_borrow fn:".bold().blue());

    let rocket_fuel = String::from("RP-1");

    let length = process_fuel_borrow(&rocket_fuel); // added & to rocket_fuel
    println!("rocket fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel_borrow(rocket_fuel: &String) -> usize { // added & to String type
    println!("processing rocket fuel {}...", rocket_fuel);

    let length = rocket_fuel.len();
    length
}


pub fn slice_string() {
    println!("\n{}", "slice_string fn:".bold().blue());

    let message = String::from("Greeting from Earth!");
    let last_word = &message[14..]; // from index 14 to the end

    println!("last word is {}", last_word);
}


pub fn slice_array() {
    println!("\n{}", "slice_array fn:".bold().blue());

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets = &planets[..4]; // first 4 elements

    println!("inner planets: {:?}", inner_planets);
}


pub fn slice_fn_params() {
    println!("\n{}", "slice_fn_params fn:".bold().blue());

    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);

    println!("first word is: {}", first_word);
}

fn get_first_word(message: &String) -> &str { 
    let bytes = message.as_bytes();  // Note that we're iterating over individual bytes in for loop rather than characteres, because the index for the string is in terms of bytes.
                                            // If the first word contains a UTF-8 character that occupies multiple bytes, then trying to index based on the number of characters we've searched 
                                            // instead of bytes would cause problems.
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &message[..i];
        }
    }
    &message
}

pub fn trim_spaces_all() {
    println!("\n{}", "trim_spaces_all fn:".bold().blue());

    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    
    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀"); 
    println!("trim_spaces_all: All tests passed!");
}

fn trim_spaces(text: &str) -> &str {
    // locate the first non-space character
    let mut start = 0;

    for (i, char) in text.chars().enumerate() {
        if char != ' ' {
            start = i;
            break;
        }
    }

    // locate the last non-space character (search in reverse order)
    let mut end = 0;

    for (i, char) in text.chars().rev().enumerate() {
        if char != ' ' {
            end = text.len() - i;
            break;
        }
    }

    &text[start..end]
}