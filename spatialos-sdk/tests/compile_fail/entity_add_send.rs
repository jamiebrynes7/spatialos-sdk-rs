extern crate spatialos_sdk;

use spatialos_sdk::dummy_component;
use spatialos_sdk::worker::entity::Entity;
use spatialos_sdk::worker::component::inventory;
use std::{cell::RefCell, rc::Rc};

pub struct TestComponent(Rc<RefCell<bool>>);
dummy_component!(TestComponent, TestComponentUpdate);

fn free_handle_on_drop_entity() {
    let was_dropped = Rc::new(RefCell::new(false));
    let mut entity = Entity::new();
    let _ = entity.add_handle(TestComponent(was_dropped.clone()));
    //~^ ERROR cannot be sent between threads safely
}
