use colored::*;


pub fn logic_operators() { 
    println!("\n{}", "logic_operator fn:".bold().blue());

    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a); 
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b); 
    println!("a XOR b {}", a ^ b); 
    
    /*
        result:
        a is true and b is false
        NOT a is false (flip true to false or false to true)
        a AND b is false (because b is false)
        a OR b is true (because at least one of the values is true)
        a XOR b is true (because they are different)
    */
}

/* Sort-circuiting logical operations
    && and ||
    ex: 
    false && [not evaluated] -> false
    true || [[not evaluated] -> true
*/
