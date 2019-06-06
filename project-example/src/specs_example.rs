use specs::prelude::*;

use spatialos_sdk::worker::connection::WorkerConnection;
use spatial_specs::world::*;
use spatial_specs::storage::*;

use crate::generated::improbable::*;

use std::time::Duration;
use std::thread;

use rand::Rng;

// A component contains data which is associated with an entity.

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (SpatialWriteStorage<'a, Position>);

    fn run(&mut self, (mut pos): Self::SystemData) {
        println!("\n");

        let mut rng = rand::thread_rng();

        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        // You could also use `par_join()` to get a rayon `ParallelIterator`.
        for (pos) in (&mut pos).join() {
            println!("{:?}", pos.coords);
            pos.coords.x = rng.gen();
        }
    }
}

pub fn run_game(mut connection: WorkerConnection) {
	let mut world_reader = WorldReader::new();

    world_reader.register_component::<Position>();

    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::new();

    world_reader.setup(&mut world);

    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();

    // setup() must be called before creating any entity, it will register
    // all Components and Resources that Systems depend on
    dispatcher.setup(&mut world.res);

    // An entity may or may not contain some component.

    // world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    // world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    // world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    // // This entity does not have `Vel`, so it won't be dispatched.
    // world.create_entity().with(Pos(2.0)).build();


    loop {
    	world_reader.process(&mut connection, &mut world);

     //    let ops = c.get_op_list(0);

     //    // This dispatches all the systems in parallel (but blocking).
    	dispatcher.dispatch(&world.res);

        world_reader.replicate(&mut connection, &mut world);

        thread::sleep(Duration::from_millis(1000))
    }

    
}
