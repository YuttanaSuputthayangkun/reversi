// check the Bevy's implementation
use std::collections::HashMap;

use super::{BoardPosition, Size};

#[derive(Debug, PartialEq, Eq)]
pub struct Board<Cell>
where
    Cell: Default,
{
    size: Size,
    cells: HashMap<BoardPosition, Cell>,
}
impl<Cell: Default> Board<Cell> {
    pub fn new(s: Size) -> Self {
        let mut cells = HashMap::new();
        let positions = (0..s.x()).flat_map(|x| (0..s.y()).map(move |y| BoardPosition { x, y }));
        for p in positions {
            if cells.insert(p, Cell::default()).is_some() {
                panic!("Fail to insert cell at ({:?}), this shouldn't happen!", p);
            }
        }
        Board {
            size: s,
            cells: cells,
        }
    }
    pub fn cell_ref<'a>(&'a self, p: &BoardPosition) -> Option<&'a Cell> {
        self.cells.get(p)
    }
}
