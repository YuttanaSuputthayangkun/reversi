use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Res};

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
        commands.entity(entity.clone()).despawn_recursive();
    }
    commands.remove_resource::<Resource>();
}
