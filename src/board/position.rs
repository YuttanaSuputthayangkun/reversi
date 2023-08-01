use super::Direction;
use std::ops::AddAssign;

pub type PositionUnit = i64;

pub type Magnitude = u8;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BoardPosition {
    pub x: PositionUnit,
    pub y: PositionUnit,
}

impl BoardPosition {
    pub fn apply_direction(&mut self, direction: &Direction, magnitude: Magnitude) {
        use Direction::*;
        match direction {
            Left => self.x -= magnitude as PositionUnit,
            Right => self.x += magnitude as PositionUnit,
            Up => self.y += magnitude as PositionUnit,
            Down => self.y -= magnitude as PositionUnit,
            UpLeft => {
                self.y += magnitude as PositionUnit;
                self.x -= magnitude as PositionUnit;
            }
            UpRight => {
                self.y += magnitude as PositionUnit;
                self.x += magnitude as PositionUnit;
            }
            DownLeft => {
                self.y -= magnitude as PositionUnit;
                self.x -= magnitude as PositionUnit;
            }
            DownRight => {
                self.y -= magnitude as PositionUnit;
                self.x += magnitude as PositionUnit;
            }
        }
    }
}

impl AddAssign<(&Direction, Magnitude)> for BoardPosition {
    fn add_assign(&mut self, rhs: (&Direction, Magnitude)) {
        self.apply_direction(rhs.0, rhs.1)
    }
}

impl From<(PositionUnit, PositionUnit)> for BoardPosition {
    fn from(value: (PositionUnit, PositionUnit)) -> Self {
        BoardPosition {
            x: value.0,
            y: value.1,
        }
    }
}
