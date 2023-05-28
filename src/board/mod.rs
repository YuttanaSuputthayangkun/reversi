mod board;
mod builder;
mod iterator;
mod position;
mod size;

pub use board::*;
pub use builder::*;
pub use iterator::*;
pub use position::*;
pub use size::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
