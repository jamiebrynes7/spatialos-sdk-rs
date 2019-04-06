use approx;
use spatialos_sdk::worker::{entity::Entity, snapshot::*, EntityId};
use std::env;

use crate::generated::improbable::*;
use spatialos_sdk::worker::entity_builder::EntityBuilder;

#[test]
pub fn writing_invalid_entity_returns_error() {
    let snapshot_path = env::temp_dir().join("test2.snapshot");

    let entity = Entity::new();

    let error = SnapshotOutputStream::new(snapshot_path)
        .expect("Error")
        .write_entity(EntityId::new(1), &entity);

    assert!(error.is_err());
}

#[test]
pub fn create_and_read_snapshot() {
    let snapshot_path = env::temp_dir().join("test.snapshot");

    let entity = get_test_entity().expect("Error");

    {
        let snapshot = SnapshotOutputStream::new(snapshot_path.clone()).expect("Error");
        snapshot
            .write_entity(EntityId::new(1), &entity)
            .expect("Error");
    }

    {
        let mut snapshot = SnapshotInputStream::new(snapshot_path).expect("Error");

        assert!(snapshot.has_next());

        let entity = snapshot.read_entity().expect("Error");

        let position = entity.get::<Position>();
        assert!(position.is_some());
        let coords = &position.unwrap().coords;
        approx::abs_diff_eq!(10.0, coords.x);
        approx::abs_diff_eq!(-10.0, coords.y);
        approx::abs_diff_eq!(0.0, coords.z);

        let persistence = entity.get::<Persistence>();
        assert!(persistence.is_some());

        let acl = entity.get::<EntityAcl>();
        assert!(acl.is_some());
        let read_acl = &acl.unwrap().read_acl;
        assert_eq!(1, read_acl.attribute_set.len());
        assert_eq!("RustWorker", read_acl.attribute_set[0].attribute[0])
    }
}

fn get_test_entity() -> Result<Entity, String> {
    let mut builder = EntityBuilder::new(10.0, -10.0, 0.0, "RustWorker");
    builder.set_persistent("RustWorker");
    builder.build()
}
