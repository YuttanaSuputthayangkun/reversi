mod builder;
mod size;

pub use builder::*;
pub use size::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    size: Size,
}

impl Board {}
