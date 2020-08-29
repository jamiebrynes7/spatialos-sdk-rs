use approx;
use spatialos_sdk::{entity::Entity, snapshot::*, EntityId};
use std::env;

use crate::generated::improbable::*;
use spatialos_sdk::entity_builder::EntityBuilder;

#[test]
pub fn writing_invalid_entity_returns_error() {
    let snapshot_path = env::temp_dir().join("test2.snapshot");

    let entity = Entity::new();

    let error = SnapshotOutputStream::new(snapshot_path)
        .expect("Error")
        .write_entity(EntityId::new(1), entity);

    assert!(error.is_err());
}

#[test]
pub fn create_and_read_snapshot() {
    let snapshot_path = env::temp_dir().join("test.snapshot");

    let entity = get_test_entity();

    {
        SnapshotOutputStream::new(snapshot_path.clone())
            .expect("Failed to create `SnapshotOutputStream`")
            .write_entity(EntityId::new(1), entity)
            .expect("Failed to write entity to snapshot");
    }

    {
        let mut snapshot = SnapshotInputStream::new(snapshot_path)
            .expect("Failed to create `SnapshotInputStream`");

        assert!(snapshot.has_next());

        let entity = snapshot
            .read_entity()
            .expect("Failed to read entity from snapshot");

        let position = entity
            .get::<Position>()
            .expect("No `Position` component on entity")
            .expect("Failed to deserialize `Position`");
        let coords = &position.coords;
        approx::abs_diff_eq!(10.0, coords.x.0);
        approx::abs_diff_eq!(-10.0, coords.y.0);
        approx::abs_diff_eq!(0.0, coords.z.0);

        let persistence = entity.get::<Persistence>();
        assert!(persistence.is_some());

        let acl = entity
            .get::<EntityAcl>()
            .expect("No `EntityAcl` component on entity")
            .expect("Failed to deserialize `EntityAcl`");
        let read_acl = &acl.read_acl;
        assert_eq!(1, read_acl.attribute_set.len());
        assert_eq!("RustWorker", read_acl.attribute_set[0].attribute[0])
    }
}

fn get_test_entity() -> Entity {
    let mut builder = EntityBuilder::new(10.0, -10.0, 0.0, "RustWorker");
    builder.set_persistent("RustWorker");
    builder.build().expect("Failed to build test entity")
}
