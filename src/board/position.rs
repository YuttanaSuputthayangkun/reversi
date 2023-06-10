use super::Direction;
use std::ops::AddAssign;

type Position = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BoardPosition {
    pub x: Position,
    pub y: Position,
}

impl BoardPosition {
    pub fn apply_direction(&mut self, direction: &Direction, magnitude: usize) {
        use Direction::*;
        match direction {
            Left => self.x -= magnitude,
            Right => self.x += magnitude,
            Up => self.y += magnitude,
            Down => self.y -= magnitude,
            UpLeft => {
                self.y += magnitude;
                self.x -= magnitude;
            }
            UpRight => {
                self.y += magnitude;
                self.x += magnitude;
            }
            DownLeft => {
                self.y -= magnitude;
                self.x -= magnitude;
            }
            DownRight => {
                self.y -= magnitude;
                self.x += magnitude;
            }
        }
    }
}

impl AddAssign<(&Direction, Position)> for BoardPosition {
    fn add_assign(&mut self, rhs: (&Direction, Position)) {
        self.apply_direction(rhs.0, rhs.1)
    }
}

impl From<(usize, usize)> for BoardPosition {
    fn from(value: (usize, usize)) -> Self {
        BoardPosition {
            x: value.0,
            y: value.1,
        }
    }
}
