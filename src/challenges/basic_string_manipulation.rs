const BIRTHDAY:i32 = 1;

fn main() {
    let my_name = "Name";
    let my_birthday = "February 30";
    let mut age = 19;
    let new_age = age+BIRTHDAY;
    println!("My name is {} and I am {} years old. I will turn {} on {}.", my_name, age, new_age, my_birthday);
}