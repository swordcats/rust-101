#![allow(unused)] // allow for unused variables; no errors

fn main() {
    // Strings - There are several types
    // Likely only use two: String and &str (the third one is str)
    // str = string slice, &str = borrowed string slice - cannot be modified
    // String = data can be modified
    // Borrowed string slices are essentially a subset of a string

    let greeting = "Hello!"; // automatically a borrowed string slice
    let mut name = String::new();
    name.push_str("Heath");
    name.push_str(" test"); // add space and test to the end of string
    println!("{}", name);

    let name1 = String::from("Heath");

    // ESCAPE CHARACTERS
    /* Common Escape Characters 
    \\ = backslash
    \" = double quote
    \' = single quote
    \n = newline
    \r = carriage return
    \t = tab
    \0 = null character
    \xNN = hexadecimal escape sequence where NN is a two-digit hexadecimal number representing a Unicode scalar value*/
    println!("Hello, world!");
    println!("He said \"All is fair in love and war\"");
    println!("{}", concat!("Hello, ", "World!"));
    
    let heart = '\u{2764}';
    println!("{}", heart);

    // Raw string literals are declared with r#" ... "#
    let message = r#"Hello, "world!"\n"#;
    println!("{}", message);

}