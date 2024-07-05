use colored::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;


pub fn writing_file() {
    println!("\n{}", "writing_file fn:".bold().color("blue"));

    let mut file = OpenOptions::new()
        .append(true) // true to append, false to overwrite file content
        .open("my_file.txt")
        .expect("Unable to open file");

    let text = " We're making it happen!";

    file.write_all(text.as_bytes())
        .expect("Unable to write to file");

    println!("File text writed:\n{}", text);
}

pub fn reading_file() {
    println!("\n{}", "reading_file fn:".bold().color("blue"));

    let text = fs::read_to_string("my_file.txt").expect("Unable to read file");

    println!("File text:\n{}", text);
}