#![allow(unused)] // allow for unused variables; no errors

use std::io;

trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

fn draw_shape<T: Drawable>(shape: &T) {
    shape.draw();
}


fn main() {
    // Traits - A group of methods that are defined for a particular type.
    let circle = Circle { radius: 10.0 };
    circle.draw();
    draw_shape(&circle);
}