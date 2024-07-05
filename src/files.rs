use colored::*;
use std::fs::{self, OpenOptions};
use std::io::Write;


pub fn reading_file() {
    println!("\n{}", "reading_file fn:".bold().blue());

    let file = fs::read_to_string("my_file.txt")
        .expect("Unable to read file");

    println!("File content:\n{}", file);
}

pub fn writing_file() {
    println!("\n{}", "writing_file fn:".bold().blue());

    let mut file = OpenOptions::new()
        .append(true)
        .open("my_file.txt")
        .expect("Unable to open file");

    let text = "We're making it happen!";

    file.write_all(text.as_bytes())
        .expect("Unable to write to file");
}