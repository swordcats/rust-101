#![allow(unused)] // allow for unused variables; no errors

use std::io;
use std::cmp::Ordering;
fn main() {
    let money1 = 10;
    let can_purchase_drink = if money1 >= 10 {
        true
    } else {
        false
    };
    println!("Can purchase a drink? {}", can_purchase_drink);

    println!("How much money do you have?");
    let mut input_money = String::new();
    io::stdin().read_line(&mut input_money);

    let money: i32 = input_money.trim().parse().expect("Entry was not an integer");

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    if (age >= 21) && (money >= 5) {
        println!("We're getting a drink");
    } else if (age >= 21) && (money < 5) {
        println!("You don't have enough money for a drink")
    } else if (age < 21) && (money >= 5) {
        println!("You are not old enough to drink legally")
    } else {
        println!("You're too young and too broke.")
    };

    // Match does pattern matching; kinda like switch case. 
    // All possible values must be covereed

    let candidacy_age = age;

    match candidacy_age {
        1..=24 => println!("Cannot hold office."),
        25..=29 => println!("Can run for the House."),
        30..=34 => println!("Can run for the Senate."),
        35..=i32::MAX => println!("Can run for President."),
        _ => println!("Are you an infant?")
    };

    let my_age = 19;
    let drinking_age = 21;

    match my_age.cmp(&drinking_age) {
        Ordering::Less => println!("Cannot drink!"),
        Ordering::Equal => println!("Woo, you can drink!"),
        Ordering::Greater => println!("Can drink!")
    };

    // LOOPS - While, for, infinite

    // FOR LOOPS - Start to finish of an iterate

    let mut vegetables = ["Cucumber", "Spinach", "Tomato", "Jalapeno", "Broccoli"];

    for x in vegetables.iter() {
        println!("{}", x)
    }

    // WHILE LOOPS

    let mut i = 1;

    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    // INFINITE LOOPS
    let mut y = 0;
    println!("Counting!");
    loop {
        y += 1;

        if y == 10 {
            println!("We've reached 10.");
            break;
        }
    }

}