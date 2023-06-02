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

#[cfg(test)]
mod test {
    use super::BoardPosition;

    #[test]
    fn apply_up() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Up, 1);
        assert_eq!(pos, BoardPosition { x: 1, y: 2 });
    }

    #[test]
    fn apply_down() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Down, 1);
        assert_eq!(pos, BoardPosition { x: 1, y: 0 });
    }

    #[test]
    fn apply_left() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Left, 1);
        assert_eq!(pos, BoardPosition { x: 0, y: 1 });
    }

    #[test]
    fn apply_right() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Right, 1);
        assert_eq!(pos, BoardPosition { x: 2, y: 1 });
    }

    #[test]
    fn apply_up_left() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&UpLeft, 1);
        assert_eq!(pos, BoardPosition { x: 0, y: 2 });
    }

    #[test]
    fn apply_up_right() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&UpRight, 1);
        assert_eq!(pos, BoardPosition { x: 2, y: 2 });
    }

    #[test]
    fn apply_down_left() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&DownLeft, 1);
        assert_eq!(pos, BoardPosition { x: 0, y: 0 });
    }

    #[test]
    fn apply_down_right() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&DownRight, 1);
        assert_eq!(pos, BoardPosition { x: 2, y: 0 });
    }
}
