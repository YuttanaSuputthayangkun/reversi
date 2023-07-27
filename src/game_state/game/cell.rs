use super::{board, Board as GameBoard, BoardCell as GameBoardCell, Player};

pub struct PlacableCellIterator<'a> {
    player: Player,
    board: &'a mut GameBoard,
    board_iterator: Box<dyn Iterator<Item = &'a mut GameBoardCell>>,
}

impl<'a> PlacableCellIterator<'a> {
    pub fn new(player: Player, board: &'a mut GameBoard) -> Self {
        let iterator = todo!();
        PlacableCellIterator {
            player: player,
            board: board,
            board_iterator: iterator,
        }
    }
}

impl<'a> Iterator for PlacableCellIterator<'a> {
    type Item = &'a mut GameBoardCell;

    fn next(&mut self) -> Option<Self::Item> {
        self.board_iterator.next()
    }
}
