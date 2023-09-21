#![allow(unused)] // allow for unused variables; no errors

use std::io;

/// A simple calculator that takes two user inputs
/// then calculates the addition, subtraction, multiplication,
/// and division of those inputs.
fn main() {

    // Declare the variables
    let mut x = String::new();
    let mut y = String::new();

    // Read values into both variables
    println!("Enter a number: ");
    io::stdin().read_line(&mut x);
    println!("Enter a number: ");
    io::stdin().read_line(&mut y);

    // Convert strings into i32
    // The parse method does the actual conversion, then the unwrap method makes it
    // so that it is i32 instead of a Result
    let mut x_int = x.trim().parse::<i32>().unwrap();
    let mut y_int = y.trim().parse::<i32>().unwrap();
    let x_float = x_int as f64;
    let y_float = y_int as f64;

    println!("RESULTS:");
    println!("{} + {} = {}", x_int, y_int, x_int + y_int);
    println!("{} - {} = {}", x_int, y_int, x_int - y_int);
    println!("{} * {} = {}", x_int, y_int, x_int * y_int);
    println!("{} / {} = {}", x_int, y_int, x_float / y_float);

}