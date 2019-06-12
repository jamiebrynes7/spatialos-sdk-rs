use specs::prelude::*;

use spatialos_sdk::worker::connection::WorkerConnection;
use spatialos_specs::spatial_reader::*;
use spatialos_specs::spatial_writer::*;
use spatialos_specs::storage::*;
use spatialos_sdk::worker::entity::Entity as WorkerEntity;
use spatialos_sdk::worker::entity_builder::EntityBuilder;

use crate::generated::example::*;
use crate::generated::improbable::*;
use spatialos_specs::commands::*;
use spatialos_specs::entities::*;
use spatialos_specs::system_commands::*;
use spatialos_specs::*;

use std::thread;
use std::time::Duration;

use rand::Rng;



fn create_player_entity(has_authority: bool) -> WorkerEntity {
    let mut rng = rand::thread_rng();

    let mut builder =
        EntityBuilder::new(0.0, 0.0, 0.0, if has_authority { "rusty" } else { "other" });

    builder.add_component(
        Example {
            x: 60.0
        },
        "rusty",
    );
    builder.set_metadata("Rotator", "rusty");
    builder.set_entity_acl_write_access("rusty");

    builder.build().unwrap()
}


struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (
        SpatialEntities<'a>,
        SpatialWriteStorage<'a, Position>,
        CommandSender<'a, Example>,
    );

    fn run(&mut self, (entities, mut pos, mut example_command_sender): Self::SystemData) {
        println!("\n");

        let mut rng = rand::thread_rng();

        for (entity, pos) in (&entities, &mut pos).join() {
            println!("write: {:?}", pos.coords);
            pos.coords.x = rng.gen();

            let this_entity = *entity;

            example_command_sender.send_command(
                this_entity,
                ExampleCommandRequest::TestCommand(CommandData { value: rng.gen() }),
                move |res, response| {

                    let mut storage = SpatialWriteStorage::<Position>::fetch(res);
                    storage.get_mut(this_entity).unwrap().coords.x = 5.0;

                    match response {
                        Ok(response) => println!("response {:?}", response),
                        Err(err) => println!("error {:?}", err)
                    };

                    let player_entity = create_player_entity(true);

                    SystemCommandSender::fetch(res).create_entity(player_entity, |res, entity_id| {
                        println!("created entity! {:?}", entity_id);
                        let sender = SystemCommandSender::fetch(res);
                    });
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
                        Some(ExampleCommandResponse::TestCommand(CommandData { value: command_data.value }))
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
