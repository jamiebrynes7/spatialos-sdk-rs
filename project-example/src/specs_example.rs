use specs::prelude::*;

use spatialos_sdk::worker::connection::WorkerConnection;
use spatial_specs::spatial_reader::*;
use spatial_specs::spatial_writer::*;
use spatial_specs::storage::*;

use crate::generated::improbable::*;

use std::time::Duration;
use std::thread;

use rand::Rng;

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (SpatialWriteStorage<'a, Position>);

    fn run(&mut self, (mut pos): Self::SystemData) {
        println!("\n");

        let mut rng = rand::thread_rng();

        for (pos) in (&mut pos).join() {
            println!("write: {:?}", pos.coords);
            pos.coords.x = rng.gen();
        }
    }
}

pub fn run_game(mut connection: WorkerConnection) {
    let mut world = World::new();

    world.add_resource(connection);

    let mut dispatcher = DispatcherBuilder::new()
        .with(SpatialReaderSystem, "reader", &[])
        .with_barrier()

        .with(SysA, "sys_a", &[])

        .with_barrier()
        .with(SpatialWriterSystem, "writer", &[])
        .build();

    dispatcher.setup(&mut world.res);

    loop {
    	dispatcher.dispatch(&world.res);

        thread::sleep(Duration::from_millis(1000))
    }
}
