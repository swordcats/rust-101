#![allow(unused)] // allow for unused variables; no errors

use std::io;

trait Damage {
    fn damage(&mut self);
}

#[derive(Debug)]
struct HP {
    hp_remaining: i32,
}

impl Damage for HP {
    fn damage(self: &mut Self) {
        self.hp_remaining -= 1;
    }
}
fn main() {
    // Traits - A group of methods that are defined for a particular type.
    let mut hp = HP { hp_remaining: 100 };
    hp.damage();
    println!("You took a hit! HP Remaining: {:?}", hp);

}