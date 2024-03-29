use super::{inner_board::Board, BoardPosition, Direction, Magnitude};

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
    step: Magnitude,
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
        step: Magnitude,
    ) -> IterMut<'a, Cell> {
        IterMut {
            board,
            position: pos,
            direction,
            step,
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
    type Item = (BoardPosition, &'a mut Cell);

    fn next(&mut self) -> Option<Self::Item> {
        use State::*;
        match &self.state {
            NotStarted => {
                self.state = Started;
                self.position.apply_direction(&self.direction, self.step);
                let position = self.position;
                let result = self.current_cell_mut();
                result.map(|c| unsafe {
                    (
                        position,
                        &mut *(c as *mut Cell), // this is for bypassing the lifetime
                    )
                })
            }
            Started => {
                self.position.apply_direction(&self.direction, self.step);
                let position = self.position;
                let result = self.current_cell_mut().map(|c| unsafe {
                    (
                        position,
                        &mut *(c as *mut Cell), // this is for bypassing the lifetime
                    )
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

pub struct Iter<'a, Cell>
where
    Cell: Default,
{
    board: &'a Board<Cell>,
    position: BoardPosition,
    direction: Direction,
    step: Magnitude,
    state: State,
}

impl<'a, Cell> Iter<'a, Cell>
where
    Cell: Default,
{
    pub fn new(
        board: &'a Board<Cell>,
        pos: BoardPosition,
        direction: Direction,
        step: Magnitude,
    ) -> Iter<'a, Cell> {
        Iter {
            board,
            position: pos,
            direction,
            step,
            state: State::NotStarted,
        }
    }

    fn cell(&mut self, pos: &BoardPosition) -> Option<&Cell> {
        self.board.cell_ref(pos)
    }

    fn current_cell(&mut self) -> Option<&Cell> {
        self.board.cell_ref(&self.position)
    }
}

impl<'a, Cell> Iterator for Iter<'a, Cell>
where
    Cell: Default,
{
    type Item = (BoardPosition, &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        use State::*;
        match &self.state {
            NotStarted => {
                self.state = Started;
                self.position.apply_direction(&self.direction, self.step);
                let position = self.position;
                let result = self.current_cell();
                result.map(|c| unsafe {
                    (
                        position,
                        &*(c as *const Cell), // this is for bypassing the lifetime
                    )
                })
            }
            Started => {
                self.position.apply_direction(&self.direction, self.step);
                let result = self.current_cell().map(|c| unsafe {
                    &*(c as *const Cell) // this is for bypassing the lifetime
                });
                if result.is_none() {
                    self.state = End;
                }
                result.map(|c| (self.position, c))
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
        assert!(iter.next().is_none());
    }

    #[test]
    fn mutate() {
        type Cell = Option<()>;
        let size: Size = (1, 2).try_into().unwrap();
        let mut board = Board::<Cell>::new(size);
        let iter = IterMut::new(&mut board, BoardPosition { x: 0, y: 0 }, Direction::Up, 1);
        iter.for_each(|(_, c)| *c = Some(()));
        let board2 = Board {
            size,
            cells: {
                let mut map = HashMap::<BoardPosition, Cell>::new();
                map.insert((0, 0).into(), None);
                map.insert((0, 1).into(), Some(()));
                map
            },
        };
        assert_eq!(board, board2);
    }
}
