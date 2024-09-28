use colored::*;

/* 
    Loop use cases:
    - Repeat a block of code forever;
    - Need the loop to return a value;
    - Infitnite loop until a condition breaks out of the loop;
    - Can exit using "break".
*/ 

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
