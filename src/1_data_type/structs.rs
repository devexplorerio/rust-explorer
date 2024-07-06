use colored::*;

struct Person {
    name: String,
    age: u8
}

pub fn object_person() {
    println!("\n{}", "object fn:".bold().blue());

    let person = Person {
        name: String::from("Jennifer"),
        age: 35
    };

    println!("{} has {} years old", person.name, person.age);
}