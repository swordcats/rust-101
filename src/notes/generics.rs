#![allow(unused)] // allow for unused variables; no errors

use std::io;
fn main() {
    // Generics 


    // T is just a type
    // Usees the operator Add feature
    fn sum<T: std::ops::Add<Output=T>> (a: T, b: T) -> T {
        a + b
    }

    let x = sum(1, 2);
    let y = sum(2.3, 3.5);
    println!("The value of x is {} and the value of y is {}.", x, y);
    println!("The sum of 3 + 2 is {}", sum(3, 2));

    struct Items<T> {
        x: T,
        y: T
    }

     let i = Items { x: 1.0, y: 2.0 };
     println!("{}, {}", i.x, i.y);
}