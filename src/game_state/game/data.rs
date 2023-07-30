use bevy::prelude::{Color, Deref};

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
    pub board_size_x: BoardSize,
    pub board_size_y: BoardSize,
    pub cell_color: Color,
    pub cell_hovered_color: Color,
    pub cell_clickable_color: Color,
    pub background_color: Color,
}

#[derive(Debug)]
pub struct CellData {
    pub position: board::BoardPosition,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
            Turn::Black => Player::White,
            Turn::White => Player::Black,
        }
    }
}

#[derive(Debug)]
pub struct GameData {
    pub turn: Turn,
    pub turn_count: u16,
    pub board: Board,
}

impl GameData {
    pub fn new(first_turn: Turn, board_size_x: BoardSize, board_size_y: BoardSize) -> Self {
        let size =
            board::Size::new(board_size_x.size().into(), board_size_y.size().into()).unwrap();
        let board = Board::new(size);
        GameData {
            turn: first_turn,
            turn_count: 0,
            board: board,
        }
    }
}
