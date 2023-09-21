#![allow(unused)] // allow for unused variables; no errors

use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::fs;

fn main() {
    // HELPER METHODS - unwrap(), expect()
    // Two Types of Errors: Recoverable & unrecovereable
    // Recoverable - result enum & option enum
    // Unrecoverable - panic! macro

    // Result enum - Result<T, E>

    let result = divide(10, 2);
    //.expect("Divide by zero error");
    /*match result {
        Ok(value) => println!("Result: {}", value),
        Err(msg) => println!("Error: {}", msg)
    }*/

    println!("The show must go on!");

    // ? operator

    let result = read_file("src/files/test.txt");
    match result {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => println!("Error reading file: {}", err),
    }
}

fn divide(x: i32, y:i32) -> Result<i32, String> {
    if y == 0 {
        panic!("Cannot divide by zero.") // Unrecoverable crash
        //return Err(String::from("Cannot divide by zero"))
    }
    Ok(x/y)
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?; 
    /*{
        Ok(file) => file,
        Err(e) => return Err(e),
    };*/

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}