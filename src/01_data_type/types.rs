use colored::*;

pub fn primitive_types() {
    println!("\n{}", "primitive_types fn:".bold().blue());

    let bool: bool = true;                                   // bool: true, false
    let inte = 10;                                      // integer (i8, u8, i16, u16, i32, u32, i64, u64...): 1, 2, 50, 99, -2... 
    let floa = 1.5;                                     // double/float (f32, f64): 1.1., 5.5, 200.0001, 2.0...
    let char: char = 'a';                                    // char: 'A', 'B', 'c', '6', '$'
    let str = "This is a string";                      // str: "Any string" -> String Literal (&str): - hard-coded into the executable; - immutable; - must be known before compilation;
    let tup: (&str, i32, char) = ("Eve", 38, 'P');           // tuple (&str, i32, char): ("Eve", 38, 'P')
    let vect = vec!["glasses", "gloves", "hat"];  // vector (with macro vec! or Vec::new()): ["glasses", "gloves", "hat"]
    let arra = ['a', 'r', 'r', 'a', 'y'];         // array [char; 5]: ['a', 'r', 'r', 'a', 'y']
    
    println!("boolean type ex: {}", bool);
    println!("integer type ex: {}", inte);
    println!("double/float type ex: {}", floa);
    println!("character type ex: {}", char);
    println!("string type ex: {}", str);
    println!("tuple type ex: {:?}", tup);
    println!("vector type ex: {:?}", vect);
    println!("array type ex: {:?}", arra);
}

// https://doc.rust-lang.org/std/index.html#primitives

/* 
    String Type: 
    - allocated on the heap
    - mutable
    - dynamically generated at runtime
 */ 
pub fn string_type() {
    println!("\n{}", "string_type fn:".bold().blue());

    let mut message = String::from("Earth");
    println!("message is: {}",message);
    message.push_str(" is home.");
    println!("now message is: {}",message);
}

// https://doc.rust-lang.org/std/string/struct.String.html


fn sentence_builder(person_info: (&str, i32, char)) {
    println!("{} is {}, and her last initial is {}.", person_info.0, person_info.1, person_info.2);
}

pub fn tuple_type() {
    println!("\n{}", "tuple_type fn:".bold().blue());

    let person_info = ("Eve", 38, 'P');
    sentence_builder(person_info);
}