mod board;
mod builder;
mod iterator;
mod position;
mod size;

#[cfg(test)]
mod test;

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
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl<Cell> Board<Cell>
where
    Cell: Default,
{
    pub fn iter_mut(
        &mut self,
        pos: BoardPosition,
        direction: Direction,
        step: usize,
    ) -> iterator::IterMut<Cell> {
        iterator::IterMut::<Cell>::new(self, pos, direction, step)
    }
}
