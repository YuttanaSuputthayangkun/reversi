use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Res};

pub trait IterEntity {
    fn iter(&self) -> Box<dyn Iterator<Item = Entity> + '_>;
}

#[allow(dead_code)]
pub fn despawn_entities_and_clear_resource<Resource>(
    mut commands: Commands,
    resource: Res<Resource>,
) where
    Resource: bevy::prelude::Resource + IterEntity,
{
    for entity in resource.iter() {
        commands.entity(entity.clone()).despawn_recursive();
    }
    commands.remove_resource::<Resource>();
}
