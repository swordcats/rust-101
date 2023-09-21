#![allow(unused)] // allow for unused variables; no errors

use std::io;
fn main() {

    // Get x
    println!("Enter a number:");
    let x = get_num_input();

    // Get y
    println!("Enter a number:");
    let y = get_num_input();

    // Get operator
    println!("Enter an operator (+, -, *, /):");
    let operator = get_input();

    // Perform math
    do_math(x, y, operator.as_str().trim_end());
}

/// This function gets an input from the user.
fn get_input() -> String {
    let mut thing = String::new();
    io::stdin().read_line(&mut thing);

    return thing;
}

/// This function gets an input and converts it to a number.
fn get_num_input() -> i32 {
    let num_str = get_input();
    return num_str.trim().parse::<i32>().unwrap();
}

/// This function takes two numbers and an operator and performs a given calculation.
fn do_math(x: i32, y: i32, operator: &str) {
    // Print the beginning of the operation
    print!("{} {} {} = ", x, operator, y);
    
    match operator {
        "+" => println!("{}", x + y),
        "-" => println!("{}", x - y),
        "*" => println!("{}", x * y),
        "/" => {
            let x_as_float = x as f64;
            let y_as_float = y as f64;
            println!("{}", x_as_float - y_as_float);
        },
        _ => println!("Invalid entry. Exiting the program.")
    };
}