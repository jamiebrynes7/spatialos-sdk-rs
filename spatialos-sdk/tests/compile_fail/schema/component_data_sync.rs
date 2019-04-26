extern crate spatialos_sdk;

use spatialos_sdk::worker::{component::*, schema::{self, *}};
use std::thread;

pub struct CustomComponent;

impl Component for CustomComponent {
    const ID: ComponentId = 7777;
    type Update = CustomComponentUpdate;
}

impl SchemaObjectType for CustomComponent {
    fn from_object(object: &Object) -> Self {
        unimplemented!()
    }

    fn into_object(&self, object: &mut Object) {
        unimplemented!();
    }
}

pub struct CustomComponentUpdate;

impl Update for CustomComponentUpdate {
    type Component = CustomComponent;
}

fn main() {
    let component_data = ComponentData::new(&CustomComponent);

    thread::spawn(|| { //~ ERROR cannot be shared between threads safely
        let _ = component_data.deserialize::<CustomComponent>();
    });

    let _ = component_data.deserialize::<CustomComponent>();
}