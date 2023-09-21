#![allow(unused)] // allow for unused variables; no errors

use std::io;
fn main() {
    // Enum is a custom data type that allowos you to define a set of named values
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn opposite(&self) -> Direction {
            match *self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }
    }

    let direction = Direction::Up;
    let opposite_direction = direction.opposite();

    enum Shape {
        Circle(f32),
        Rectangle(f32, f32),
    }

    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(30.0, 40.0);
}