use colored::*;

/* 
    For use cases:
    - Iterate over each item in a collection
    - repeat a block of code N times -> iterate over a range 0..N
*/ 

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
        println!("index {} is letter {}", index, letter);
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

pub fn for_range() {
    println!("\n{}", "for_range fn:".bold().blue());

    for number in 0..5 { // the range includes the starting bound of 0 and excludes the end bound of 5
        println!("number {}", number);
    }
}

pub fn for_nested() {
    println!("\n{}", "for_nested fn:".bold().blue());

    // 2D array
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    for row in matrix.iter() {
        for num in row.iter() {
            print!("{}\t", num); // \t = tab
        }
        println!(); // to insert a new line after each row
    }
}

pub fn for_nested_mut() {
    println!("\n{}", "for_nested_mut fn:".bold().blue());

    let mut matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    for row in matrix.iter_mut() { //  use the iter_mut() method to get a mutable iterator over the numbers
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num); 
        }
        println!(); 
    }
}

// Calculate the max, min, total and mean
pub fn for_calc() {
    println!("\n{}", "for_cals fn:".bold().blue());

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];    
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for num in numbers {

        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }

        /* 
            or with 
            max = num.max(max);
            min = num.min(min);
         */

        mean += num as f64;
    };

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("mean {} max {} min {}. All tests passed.", mean, max, min);

}