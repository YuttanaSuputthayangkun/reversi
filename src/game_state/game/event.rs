use bevy::prelude::{Deref, Event};

use super::*;

#[derive(Event, Deref)]
pub struct CellClick(#[deref] pub board::BoardPosition);

#[derive(Event, Default)]
pub struct TurnChange;
