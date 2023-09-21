#![allow(unused)] // allow for unused variables; no errors

use std::io;
fn main() {
    // Vectors - Similar ot arrays
    // Slower than arrays, but more flexible

    let mut vec1 = Vec::new(); // creates an empty veector
    let vec2 = vec![1,2,3]; // creates a vector with three elements

    vec1.push(1);

    let second_element = vec2[1];
    println!("The second element is {}", second_element);
    println!("The length of the vector is {}", vec2.len());

    for element in vec2.iter() {
        println!("Element: {}", element)
    }
}