use std::marker::PhantomData;

use bevy::prelude::{
    Commands, Deref, DerefMut, DespawnRecursiveExt, Entity, Event as BevyEvent, EventWriter, Res,
    Resource,
};

#[derive(Resource, Clone, Default, Deref, DerefMut)]
pub struct Entities<Marker>(PhantomData<Marker>, #[deref] Vec<Entity>);

impl<Marker> IterEntity for Entities<Marker> {
    fn iter_entity(&self) -> Box<dyn Iterator<Item = Entity> + '_> {
        let iter = self.1.iter().copied();
        Box::new(iter)
    }
}

pub trait IterEntity {
    fn iter_entity(&self) -> Box<dyn Iterator<Item = Entity> + '_>;
}

pub fn despawn_entities_and_clear_resource<Resource>(
    mut commands: Commands,
    resource: Res<Resource>,
) where
    Resource: bevy::prelude::Resource + IterEntity,
{
    for entity in resource.iter_entity() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
    commands.remove_resource::<Resource>();
}

pub fn send_default_event<Event>(mut event_writer: EventWriter<Event>)
where
    Event: BevyEvent + Default,
{
    event_writer.send_default();
}

pub fn init_resource<Resource: bevy::prelude::Resource + Default>(mut commands: Commands) {
    commands.init_resource::<Resource>();
}

pub fn remove_resource<Resource: bevy::prelude::Resource>(mut commands: Commands) {
    commands.remove_resource::<Resource>();
}

pub mod system_adapter {
    use bevy::prelude::{Event as BevyEvent, EventWriter, In};

    pub fn send_event<Event, Input>(In(input): In<Input>, mut event_writer: EventWriter<Event>)
    where
        Event: BevyEvent,
        Input: Into<Event>,
    {
        event_writer.send(input.into());
    }

    pub fn info_pipe<Data>(In(data): In<Data>) -> Data
    where
        Data: std::fmt::Debug,
    {
        bevy::prelude::info!("{:?}", &data);
        data
    }
}
