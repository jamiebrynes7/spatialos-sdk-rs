use specs::prelude::*;

use spatialos_sdk::worker::connection::WorkerConnection;
use spatialos_specs::spatial_reader::*;
use spatialos_specs::spatial_writer::*;
use spatialos_specs::storage::*;

use crate::generated::example::*;
use crate::generated::improbable::*;
use spatialos_specs::commands::*;
use spatialos_specs::*;

use std::thread;
use std::time::Duration;

use rand::Rng;

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (
        ReadStorage<'a, EntityId>,
        SpatialWriteStorage<'a, Position>,
        CommandSender<'a, Example>,
    );

    fn run(&mut self, (entity_id, mut pos, mut example_command_sender): Self::SystemData) {
        println!("\n");

        let mut rng = rand::thread_rng();

        for (entity_id, pos) in (&entity_id, &mut pos).join() {
            println!("write: {:?}", pos.coords);
            pos.coords.x = rng.gen();

            // TODO: Make closure accept SystemData
            example_command_sender.send_command(
                *entity_id,
                ExampleCommandRequest::TestCommand(CommandData { value: 17 }),
                |res, response| {
                    println!("response {:?}", response);
                },
                |status_code| {
                    println!("failure {:?}", status_code);
                }
            );
        }
    }
}

struct SysB;

impl<'a> System<'a> for SysB {
    type SystemData = CommandRequests<'a, Example>;

    fn run(&mut self, mut requests: Self::SystemData) {
        for request in (&mut requests).join() {
            request.respond(|request| {
                println!("got request {:?}", request);

                match request {
                    ExampleCommandRequest::TestCommand(command_data) => {
                        Some(ExampleCommandResponse::TestCommand(command_data.clone()))
                    }
                }
            });
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
        .with(SysB, "sys_b", &[])
        .with_barrier()
        .with(SpatialWriterSystem, "writer", &[])
        .build();

    dispatcher.setup(&mut world.res);

    loop {
        dispatcher.dispatch(&world.res);

        thread::sleep(Duration::from_millis(1000))
    }
}
