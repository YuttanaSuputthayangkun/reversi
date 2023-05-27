mod builder;

pub use builder::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Size {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Size {
    fn from(value: (usize, usize)) -> Self {
        Size {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    size: Size,
}

impl Board {}
