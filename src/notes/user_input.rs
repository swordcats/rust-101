#![allow(unused)] // allow for unused variables; no errors

use std::env;
use std::io::{self, BufRead};

fn main() {
    println!("Enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name); // Takes input and reads it into name

    let enter = "You may now enter.";
    println!("Hello there, {}. {}", name.trim_end(), enter);

    // std::env can be used to read user input from command line
    let args: Vec<String> = env::args().collect();
    println!("The first argument is {}", args[1]);

    // BufRead reads lines of input from the stream

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

}