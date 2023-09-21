#![allow(unused)] // allow for unused variables; no errors


// Main is where the program begins
fn main() {
    hello_world();
    
    scalars();

    variables();

    challenge_a();
}

fn hello_world() {
    // Exclamation point indicates that it's a macro
    println!("Hello, world!");

    // No new line if you just use print
    print!("Hello, world again!");
    print!("Hello, world again!");

    /* Block comments are possible
    Yup */

    add(1, 2);
    
}

/// These types of comments are used for documentation
/// This function adds two numbers 
///
/// # Examples
///
/// 
/// let x = 1;
/// let y = 2;
/// assert_eq!(add(x, y), 3);
/// 

fn add(x: i32, y: i32) -> i32 {
    x + y
} 

fn scalars() {
    // Scalar Types: int, float, boolean, char
    // Unsigned: Never negative - u8, u16, u32, u64, u128, usize
    // Signed: Can be negative and positive - i8, i16, i32, i64, i 128, isize
    // The numbers represeent the number of bits
    // i32 is the standard -- it's usually the fastest
    // usize/isize is how many bytees it's gonna take to referencee any location in memory

    println!("Max size of a u32: {}", u32::MAX);
    println!("Max size of a u64: {}", u64::MAX);
    println!("Max size of a i32: {}", i32::MAX);
    println!("Max size of a i64: {}", i64::MAX);

    // Int Literals: Values that are directly represents in source code (decimals, binary, etc.)
    // There are bitwise operations like and (&), oor (|) and not(!)

    // Floats

    // Only two types - f32 and f64 (these can be called "bits of precision")
    println!("Max size of a f32: {}", f32::MAX);
    println!("Max size of a f64: {}", f64::MAX);   


    // Booleans
    // true or false - bool
    // Useed to create logical conditions

    // Character - char - 4 bytes
    // println!('A');
}

fn variables() {
    // Variables are immutable unless declared mutable
    let hello = "Hello, world!";
    println!("{}", hello);

    let mut mutable_hello = "Hello, world!";
    println!("{}", mutable_hello);

    mutable_hello = "Hello, again!";
    println!("{}", mutable_hello);

    let x = 5;
    let y = 6;
    println!("Math in Rust: {} + {} = {}", x, y, x+y);

    // Constants have to be named in screen-case (all caps)
    // You also have to declare what type they are (type annotation)
    // Constants are global, immutable, and compile quickly
    const NUMBER: i32 = 17;
    println!("{}", NUMBER);

    let x2 = 1;
    {
        // y2 is local to this block of code and cannot be run outside of it
        let y2 = 2;
        println!("{} + {} = {}", x2, y2, x2 + y2)
    }

    let x3 = 1;
    {
        let x3 = 2;
        println!("{}", x3)
    }
    println!("{}", x3);
    // Shadowing is the ability to redeclare a variable within the same scope with the same name
    let x4 = 1;
    let x4 = "Hello";

    println!("{}", x4);

    // Suffixes are used to specify the type of a numeric literal
    let x5 = 42u32; // can also declare it as 42_u32
    let y5 = 1_000_000;
    println!("{}", x5);
    println!("{}", y5)
}

const BIRTHDAY:i32 = 1;

fn challenge_a() {
    let my_name = "Name";
    let my_birthday = "February 30";
    let mut age = 19;
    let new_age = age+BIRTHDAY;
    println!("My name is {} and I am {} years old. I will turn {} on {}.", my_name, age, new_age, my_birthday);
}