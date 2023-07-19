use bevy::prelude::{Event, EventReader, Events, ResMut, Resource, Schedule, World};

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
        res.0 = res.0 + 1;
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
