use super::{Board, BoardPosition, Direction};

#[derive(Default)]
enum State {
    #[default]
    NotStarted,
    Started,
    End,
}

pub struct IterMut<'a, Cell>
where
    Cell: Default,
{
    board: &'a mut Board<Cell>,
    position: BoardPosition,
    direction: Direction,
    step: usize,
    state: State,
}

impl<'a, Cell> IterMut<'a, Cell>
where
    Cell: Default,
{
    pub fn new(
        board: &'a mut Board<Cell>,
        pos: BoardPosition,
        direction: Direction,
        step: usize,
    ) -> IterMut<'a, Cell> {
        IterMut {
            board: board,
            position: pos,
            direction,
            step: step,
            state: State::NotStarted,
        }
    }

    fn cell_mut(&mut self, pos: &BoardPosition) -> Option<&mut Cell> {
        self.board.cell_mut(pos)
    }

    fn current_cell_mut(&mut self) -> Option<&mut Cell> {
        self.board.cell_mut(&self.position)
    }
}

impl<'a, Cell> Iterator for IterMut<'a, Cell>
where
    Cell: Default,
{
    type Item = &'a mut Cell;

    fn next(&mut self) -> Option<Self::Item> {
        use State::*;
        match &self.state {
            NotStarted => {
                self.state = Started;
                let result = self.current_cell_mut();
                result.map(|c| unsafe { &mut *(c as *mut Cell) })
            }
            Started => {
                self.position.apply_direction(&self.direction, self.step);
                let result = self.current_cell_mut().map(|c| unsafe {
                    &mut *(c as *mut Cell) // this is for bypassing the lifetime
                });
                if result.is_none() {
                    self.state = End;
                }
                result
            }
            End => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::board::*;
    #[test]
    fn up() {
        let size: Size = (1, 2).try_into().unwrap();
        let mut board = Board::<()>::new(size);
        let mut iter = IterMut::new(&mut board, BoardPosition { x: 0, y: 0 }, Direction::Up, 1);
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn mutate() {
        type Cell = Option<()>;
        let size: Size = (1, 2).try_into().unwrap();
        let mut board = Board::<Cell>::new(size);
        let iter = IterMut::new(&mut board, BoardPosition { x: 0, y: 0 }, Direction::Up, 1);
        iter.for_each(|c| *c = Some(()));
        let board2 = Board {
            size: size.clone(),
            cells: {
                let mut map = HashMap::<BoardPosition, Cell>::new();
                map.insert((0, 0).into(), Some(()));
                map.insert((0, 1).into(), Some(()));
                map
            },
        };
        assert_eq!(board, board2);
    }
}
