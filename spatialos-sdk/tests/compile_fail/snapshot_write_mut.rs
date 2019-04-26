// This test verifies that `SnapshotOutputStream::write_entity` takes `&mut self`.
// This regression tests a fix originally done in PR #98.

extern crate spatialos_sdk;

use spatialos_sdk::worker::{EntityId, entity::Entity, snapshot::SnapshotOutputStream};

fn main() {
    let stream = SnapshotOutputStream::new("output.snapshot").unwrap();
    stream.write_entity(EntityId::from(7), Entity::new());
    //~^ ERROR: cannot borrow immutable local variable `stream` as mutable
}
