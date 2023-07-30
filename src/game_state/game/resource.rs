use bevy::prelude::{Deref, DerefMut, Resource};

use super::data;

#[derive(Resource, Clone, Deref)]
pub struct BoardSettings(#[deref] pub data::BoardSettings);

#[derive(Default)]
pub struct BoardEntities;

pub type Entities = super::util::Entities<BoardEntities>;

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct GameData(#[deref] pub data::GameData);

impl From<data::GameData> for GameData {
    fn from(value: data::GameData) -> Self {
        GameData(value)
    }
}
