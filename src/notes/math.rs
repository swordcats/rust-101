#![allow(unused)] // allow for unused variables; no errors

use std::io;
use rand::Rng;

fn main() {
    let x = rand::thread_rng().gen_range(1..101);
    let y = 4;
    let x_float = x as f64;
    let y_float = y as f64;

    println!("{} + {} = {}", x, y, x+y);
    println!("{} - {} = {}", x, y, x-y);
    println!("{} * {} = {}", x, y, x*y);
    println!("{} / {} = {}", x, y, x_float/y_float);
    println!("{} % {} = {}", x, y, x_float%y_float);

    println!("{}^{} = {}", x, y, i32::pow(x,y.try_into().unwrap()));

}