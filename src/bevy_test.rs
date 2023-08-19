use bevy::prelude::{
    Event, EventReader, Events, In, IntoSystem, ResMut, Resource, Schedule, World,
};

#[derive(Event)]
struct Number(i32);

#[test]
fn world() {
    #[derive(Resource)]
    struct MyResource(i32);
    let mut world = World::new();
    world.insert_resource(MyResource(0));
    let mut schedule = Schedule::new();
    fn increase_number(mut res: ResMut<MyResource>) {
        res.0 += 1;
    }
    schedule.add_systems(increase_number);
    schedule.run(&mut world);
    assert_eq!(1, world.get_resource::<MyResource>().unwrap().0);
    schedule.run(&mut world);
    assert_eq!(2, world.get_resource::<MyResource>().unwrap().0);
}

#[test]
fn read_event_twice() {
    #[derive(Resource)]
    struct Counter(i32);
    #[derive(Event)]
    struct Event;
    let mut world = World::new();
    world.init_resource::<Events<Event>>();
    world.insert_resource(Counter(0));
    let mut schedule = Schedule::new();
    fn read_event(mut event_reader: EventReader<Event>, mut res: ResMut<Counter>) {
        for _ in event_reader.iter() {
            res.0 += 1;
        }
    }
    schedule
        .add_systems(Events::<Event>::update_system)
        .add_systems(read_event);

    // send first
    world.send_event(Event);
    schedule.run(&mut world);
    assert_eq!(1, world.get_resource::<Counter>().unwrap().0);

    // run again without sending
    schedule.run(&mut world);
    assert_eq!(1, world.get_resource::<Counter>().unwrap().0);

    // send second
    world.send_event(Event);
    schedule.run(&mut world);
    assert_eq!(2, world.get_resource::<Counter>().unwrap().0);
}

#[test]
fn test_piping() {
    #[derive(Resource, Default)]
    struct MyNum(Option<i32>);

    let mut world = World::new();
    world.init_resource::<MyNum>();
    let mut schedule = Schedule::new();

    fn base_system() -> i32 {
        1234
    }
    fn update_my_num(In(num): In<i32>, mut my_num: ResMut<MyNum>) {
        *my_num = MyNum(Some(num));
    }

    schedule.add_systems(base_system.pipe(update_my_num));
    schedule.run(&mut world);

    let resource = world.get_resource::<MyNum>();
    matches!(resource, Some(&MyNum(Some(1234))));
}

// #[test]
fn test_pipe_chain() {
    #![allow(dead_code, unused_mut, unused_variables, unreachable_code)]

    use bevy::prelude::*;
    use std::ops::{Deref, DerefMut};

    #[derive(Resource, Default, Deref, DerefMut, PartialEq, Eq)]
    struct PipeResource1(#[deref] i32);

    #[derive(Resource, Default, Deref, DerefMut, PartialEq, Eq)]
    struct PipeResource2(#[deref] i32);

    let mut world = World::new();
    world.init_resource::<PipeResource1>();
    world.init_resource::<PipeResource2>();
    let mut schedule = Schedule::new();

    fn system_base() -> i32 {
        1
    }

    fn piped_in<Resource>(In(num): In<i32>, mut res: ResMut<Resource>)
    where
        Resource: bevy::prelude::Resource + Deref<Target = i32> + DerefMut,
    {
        **res = num;
    }

    matches!(world.get_resource::<PipeResource1>(), None);
    matches!(world.get_resource::<PipeResource2>(), None);

    //find a way to create system set with In<PipeResource>

    // schedule.add_systems(piped_in::<PipeResource1>);
    // schedule.add_systems(system_base.pipe(piped_in::<PipeResource1>));
    // let piped = BoxedSystem::new((piped_in::<PipeResource1>, piped_in::<PipeResource2>).chain());
    // schedule.add_systems(system_base.pipe(piped));
    schedule.run(&mut world);

    matches!(
        world.get_resource::<PipeResource1>(),
        Some(&PipeResource1(1))
    );
    matches!(
        world.get_resource::<PipeResource2>(),
        Some(&PipeResource2(2))
    );
}

#[test]
fn test_added_filter() {
    #![allow(dead_code, unused_mut, unused_variables, unreachable_code)]

    use bevy::prelude::*;
    use std::ops::DerefMut;

    #[derive(Component)]
    struct Message;

    #[derive(Resource, Deref, DerefMut)]
    struct FoundComponent(bool);

    let mut world = World::new();
    world.insert_resource(FoundComponent(false));

    fn add_component(mut commands: Commands) {
        commands.spawn(Message);
    }

    fn detect_new_component(
        mut query: Query<Added<Message>>,
        mut found_component: ResMut<FoundComponent>,
    ) {
        if !query.is_empty() {
            *found_component.as_mut().deref_mut() = true;
        }
    }

    matches!(
        world.get_resource::<FoundComponent>(),
        Some(&FoundComponent(false))
    );

    let mut schedule = Schedule::new();
    schedule.add_systems(detect_new_component);
    schedule.add_systems(add_component);
    schedule.run(&mut world);

    matches!(
        world.get_resource::<FoundComponent>(),
        Some(&FoundComponent(true))
    );
}
