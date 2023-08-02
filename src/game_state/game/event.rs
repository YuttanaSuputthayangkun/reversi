use bevy::prelude::{Deref, Event};

use super::*;

#[derive(Event, Deref)]
pub struct CellClick(#[deref] pub board::BoardPosition);

#[derive(Event, Default)]
pub struct TurnChange;

#[derive(Event)]
pub struct PlayerCellChanged {
    pub player: data::Player,
    pub board_position: board::BoardPosition,
}

#[derive(Event, Default)]
pub struct TurnStuck;
