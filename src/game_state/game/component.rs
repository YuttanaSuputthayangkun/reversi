use bevy::prelude::{Component, Deref, DerefMut};

use super::{board, data};

#[derive(Component, Deref)]
pub struct BoardPosition(pub board::BoardPosition);

#[derive(Component)]
pub struct BoardParent;

#[derive(Component)]
pub struct Cell;

#[derive(Component, Deref, DerefMut)]
pub struct Clickable(pub bool);

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Player(pub data::Player);
