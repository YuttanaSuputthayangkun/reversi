use super::{Board, BoardPosition, Direction};

enum State {
    NotStarted,
    Started,
    End,
}

struct Test<F>
where
    F: Fn() -> bool,
{
    f: F,
}

impl<F> Test<F>
where
    F: Fn() -> bool,
{
    fn call(&self) {
        (self.f)();
    }
}

struct TestBoxFn {
    pred: Box<dyn Fn() -> bool>,
}

impl TestBoxFn {
    fn call(&self) -> bool {
        (self.pred)()
    }
}

struct BoardIterator<'a, Cell>
where
    Cell: Default,
{
    board: &'a Board<Cell>,
    position: BoardPosition,
    direction: Direction,
    magnitude: usize,
    cell_predicate: Box<dyn Fn(&'a Cell) -> bool>,
    state: State,
}

impl<'a, Cell: Default> BoardIterator<'a, Cell> {
    pub fn new(
        board: &'a Board<Cell>,
        pos: BoardPosition,
        direction: Direction,
        cell_predicate: Box<dyn Fn(&'a Cell) -> bool>,
    ) -> BoardIterator<'a, Cell> {
        BoardIterator {
            board: board,
            position: pos,
            direction,
            magnitude: 1, // add an option to this later
            cell_predicate: cell_predicate,
            state: State::NotStarted,
        }
    }

    fn current_cell(&self) -> Option<&'a Cell> {
        let cell = self.board.cell_ref(&self.position)?;
        (self.cell_predicate)(cell).then_some(cell)
    }
}

impl<'a, Cell: Default> Iterator for BoardIterator<'a, Cell> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        use State::*;
        match &self.state {
            NotStarted => self.current_cell(),
            Started => {
                self.position
                    .apply_direction(&self.direction, self.magnitude);
                self.current_cell()
            }
            End => None,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn up() {}
}
