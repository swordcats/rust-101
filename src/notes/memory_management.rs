#![allow(unused)] // allow for unused variables; no errors

use std::io;


fn main() {
    let name = String::from("heath");
    let new_name = name; // "name" owns the String, but now new_man owns the String conatained in name.  String does not have a copy feature so this 

    // Rule #1: Each value has an owner
    // Rule #2: There can only be one owner at a time
    // When an owner goes out of scope, the memory is automatoically freed

    // Can borrow via &
    // Mutability has to match in order to borrow
    let borrowed_name = &new_name;

    let mut x = 10;
    let y = &mut x;
    println!("{}", *y);
    println!("{}", x);
    *y += 1;
    
}