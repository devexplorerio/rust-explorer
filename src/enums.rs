use colored::*;

#[derive(Debug)]
enum Steepness {
    Easy,
    Moderate,
    Difficult
}

pub fn enum_steepness() {
    println!("\n{}", "enum_example fn:".bold().blue());

    let _calm_trail = Steepness::Easy; // use underscore to ignore unused variable
    let _fun_trail = Steepness::Moderate; // use underscore to ignore unused variable
    let prickly_peak_trail = Steepness::Difficult;
    println!("Steepness is {:?}", prickly_peak_trail);
}
