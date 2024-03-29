use bevy::{
    prelude::{Color, Deref},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

use super::*;
use std::{ops::Rem, time::Duration};

pub type BoardCell = data::Player;
pub type Board = board::Board<BoardCell>;

#[derive(Clone, Copy, Debug, Deref, Serialize, Deserialize)]
pub struct BoardSize(u16);

impl BoardSize {
    pub fn size(&self) -> u16 {
        self.0
    }
}

impl From<BoardSize> for u16 {
    fn from(val: BoardSize) -> Self {
        val.0
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

#[derive(Clone, Serialize, Deserialize)]
pub struct BoardSettings {
    board_size_x: BoardSize,
    board_size_y: BoardSize,
    cell_color_clickable: Color,
    cell_player_color_map: HashMap<Player, Color>,
    cell_color_background: Color,
    board_player_color_map: HashMap<Player, Color>,
    board_player_color_change_duration: Duration,
}

impl BoardSettings {
    #[allow(dead_code)]
    pub fn new(
        board_size_x: BoardSize,
        board_size_y: BoardSize,
        cell_color_clickable: Color,
        cell_player_colors: impl Iterator<Item = (Player, Color)>,
        cell_color_background: Color,
        board_player_colors: impl Iterator<Item = (Player, Color)>,
        board_player_color_change_duration: Duration,
    ) -> Self {
        BoardSettings {
            board_size_x,
            board_size_y,
            cell_color_clickable,
            cell_player_color_map: cell_player_colors.collect::<HashMap<_, _>>(),
            cell_color_background,
            board_player_color_map: board_player_colors.collect::<HashMap<_, _>>(),
            board_player_color_change_duration,
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

    pub fn cell_color_background(&self) -> Color {
        self.cell_color_background
    }

    pub fn cell_player_color(&self, player: &Player) -> Color {
        self.cell_player_color_map
            .get(player)
            .copied()
            .ok_or_else(|| format!("Cannot find cell color for player: {:?}", &player))
            .unwrap()
    }

    pub fn board_player_color(&self, player: &Player) -> Color {
        self.board_player_color_map
            .get(player)
            .copied()
            .ok_or_else(|| format!("Cannot find cell color for player: {:?}", &player))
            .unwrap()
    }

    pub fn board_player_color_change_duration(&self) -> Duration {
        self.board_player_color_change_duration
    }
}

#[derive(Debug)]
pub struct CellData {
    pub position: board::BoardPosition,
}

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl From<Turn> for Player {
    fn from(val: Turn) -> Self {
        match val {
            Turn::Black => Player::Black,
            Turn::White => Player::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TurnStuckInfo {
    turn: Turn,
    turn_count: u16,
}

#[derive(Debug, Clone)]
pub struct GameData {
    first_turn: Turn,
    turn: Turn,
    turn_count: u16,
    board: Board,
    turn_stuck_info_list: Vec<TurnStuckInfo>,
}

impl GameData {
    pub fn new(first_turn: Turn, board_size_x: BoardSize, board_size_y: BoardSize) -> Self {
        let size = board::Size::new(board_size_x.size(), board_size_y.size()).unwrap();
        let board = Board::new(size);
        GameData {
            first_turn,
            turn: first_turn,
            turn_count: 0,
            board,
            turn_stuck_info_list: vec![],
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

    pub fn next_turn(&mut self) {
        self.turn = self.turn.next();
        self.turn_count += 1;
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn notify_turn_stuck(&mut self) {
        let new_info = TurnStuckInfo {
            turn: self.turn,
            turn_count: self.turn_count,
        };
        self.turn_stuck_info_list.push(new_info);
    }

    pub fn is_turn_stuck(&self) -> bool {
        let mut last_two_turns = self.turn_stuck_info_list.iter().rev();
        match (last_two_turns.next(), last_two_turns.next()) {
            (Some(last_turn), Some(before_last_turn)) => {
                let is_consecutive = (last_turn.turn_count - before_last_turn.turn_count) == 1;
                let both_stuck = last_turn.turn.next() == before_last_turn.turn;
                is_consecutive && both_stuck
            }
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        self.turn = self.first_turn;
        self.turn_count = 0;
        self.board = Board::new(self.board.size);
        self.turn_stuck_info_list = vec![];
    }
}
