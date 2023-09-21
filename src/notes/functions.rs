#![allow(unused)] // allow for unused variables; no errors

use std::io;
fn main() {
    // Functions = organized blocoks of code
}

fn who_am_i() {
    let name = "Heath";
    let age = 33;
    println!("My name is {} and I am {} yeears old.", name, age);
}

fn add_one_hundred(num: i32) {
    println!("{}", num + 100);
}

fn add(x: i32, y: i32) {
    println!("{}", x + y);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn add_and_multiply(x: i32, y: i32) -> (i32, i32) {
    (x+y, x*y)
}