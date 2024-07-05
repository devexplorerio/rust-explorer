use colored::*;

pub fn loop_equal() {
    println!("\n{}", "loop_example fn:".bold().blue());
    

    let mut a = 0;

    loop {
        if a == 10 {
            break;
        }
        println!("{:?}", a);
        a += 1;
    }
}