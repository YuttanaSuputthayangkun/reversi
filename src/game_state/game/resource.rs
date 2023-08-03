use bevy::{
    prelude::{Deref, DerefMut, Entity, Resource},
    utils::HashMap,
};

use super::{board, data, util::IterEntity};

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

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct BoardCellEntities(HashMap<board::BoardPosition, Entity>);

impl Default for BoardCellEntities {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl IterEntity for BoardCellEntities {
    fn iter_entity(&self) -> Box<dyn Iterator<Item = Entity> + '_> {
        let iter = self.values().map(|x| x.clone());
        Box::new(iter)
    }
}
