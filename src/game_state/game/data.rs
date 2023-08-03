use bevy::{
    prelude::{Color, Deref},
    utils::HashMap,
};

use super::*;
use std::ops::Rem;

pub type BoardCell = data::Player;
pub type Board = board::Board<BoardCell>;

#[derive(Clone, Copy, Debug, Deref)]
pub struct BoardSize(u16);

impl BoardSize {
    pub fn size(&self) -> u16 {
        self.0
    }
}

impl Into<u16> for BoardSize {
    fn into(self) -> u16 {
        self.0.into()
    }
}

impl TryFrom<u16> for BoardSize {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value.rem(2) {
            0 => Ok(BoardSize(value)),
            _ => Err("BoardSize can only be even."),
        }
    }
}

#[derive(Clone)]
pub struct BoardSettings {
    board_size_x: BoardSize,
    board_size_y: BoardSize,
    cell_color_clickable: Color,
    cell_player_color_map: HashMap<Player, Color>,
    background_color: Color,
}

impl BoardSettings {
    pub fn new(
        board_size_x: BoardSize,
        board_size_y: BoardSize,
        cell_color_clickable: Color,
        cell_player_colors: impl Iterator<Item = (Player, Color)>,
        background_color: Color,
    ) -> Self {
        BoardSettings {
            board_size_x: board_size_x,
            board_size_y: board_size_y,
            cell_color_clickable,
            cell_player_color_map: cell_player_colors.collect::<HashMap<_, _>>(),
            background_color: background_color,
        }
    }

    pub fn board_size_x(&self) -> BoardSize {
        self.board_size_x
    }

    pub fn board_size_y(&self) -> BoardSize {
        self.board_size_y
    }

    pub fn cell_color_clickable(&self) -> Color {
        self.cell_color_clickable
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    pub fn player_cell_color(&self, player: &Player) -> Color {
        self.cell_player_color_map
            .get(player)
            .map(|c| c.clone())
            .ok_or_else(|| format!("Cannot find cell color for player: {:?}", &player))
            .unwrap()
    }
}

#[derive(Debug)]
pub struct CellData {
    pub position: board::BoardPosition,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Player {
    #[default]
    None,
    Black,
    White,
}

impl Player {
    pub fn next(&self) -> Self {
        use Player::*;
        match self {
            Black => White,
            White => Black,
            None => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Turn {
    Black,
    White,
}

impl Turn {
    pub fn next(&self) -> Self {
        use Turn::*;
        match self {
            Black => White,
            White => Black,
        }
    }
}

impl Into<Player> for Turn {
    fn into(self) -> Player {
        match self {
            Turn::Black => Player::Black,
            Turn::White => Player::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameData {
    first_turn: Turn,
    turn: Turn,
    turn_count: u16,
    board: Board,
    turn_stuck: HashMap<Turn, bool>,
}

impl GameData {
    pub fn new(first_turn: Turn, board_size_x: BoardSize, board_size_y: BoardSize) -> Self {
        let size =
            board::Size::new(board_size_x.size().into(), board_size_y.size().into()).unwrap();
        let board = Board::new(size);
        GameData {
            first_turn: first_turn,
            turn: first_turn,
            turn_count: 0,
            board: board,
            turn_stuck: [(first_turn, false), (first_turn.next(), false)]
                .into_iter()
                .collect(),
        }
    }

    pub fn turn(&self) -> &Turn {
        &self.turn
    }

    pub fn current_player(&self) -> Player {
        self.turn.into()
    }

    pub fn opposite_player(&self) -> Player {
        self.current_player().next()
    }

    pub fn update_turn(&mut self) {
        self.turn = self.turn.next();
        self.turn_count += 1;
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn update_turn_stuck(&mut self, turn: Turn, is_stuck: bool) {
        self.turn_stuck.insert(turn, is_stuck);
    }

    pub fn is_turn_stuck(&self) -> bool {
        self.turn_stuck.values().all(|&is_stuck| is_stuck)
    }

    pub fn reset(&mut self) {
        self.turn = self.first_turn.clone();
        self.turn_count = 0;
        self.board = Board::new(self.board.size.clone());
        self.turn_stuck = HashMap::<Turn, bool>::default();
    }
}
