use spatialos_sdk::worker::{entity::Entity, snapshot::*, EntityId};
use std::env;

use crate::generated::improbable::*;
use std::collections::BTreeMap;

#[test]
pub fn create_and_read_snapshot() {
    let mut snapshot_path = env::temp_dir();
    snapshot_path.push("test.snapshot");

    let entity = expect_specialized(get_test_entity());

    {
        let snapshot = expect_specialized(SnapshotOutputStream::new(snapshot_path.clone()));
        expect_specialized(snapshot.write_entity(EntityId::new(1), &entity));
    }

    {
        let mut snapshot = expect_specialized(SnapshotInputStream::new(snapshot_path));

        assert!(snapshot.has_next());

        let entity = expect_specialized(snapshot.read_entity());

        let position = entity.get::<Position>();
        assert!(position.is_some());
        let coords = &position.unwrap().coords;
        // TODO: Look into floating point delta libs.
        assert_eq!(10.0, coords.x);
        assert_eq!(-10.0, coords.y);
        assert_eq!(0.0, coords.z);

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
    let mut entity = Entity::new();

    let position = Position {
        coords: Coordinates {
            x: 10.0,
            y: -10.0,
            z: 0.0,
        },
    };

    let acl = EntityAcl {
        read_acl: WorkerRequirementSet {
            attribute_set: vec![WorkerAttributeSet {
                attribute: vec!["RustWorker".to_owned()],
            }],
        },
        component_write_acl: BTreeMap::new(),
    };

    entity.add(position)?;
    entity.add(acl)?;
    entity.add(Persistence {})?;

    Ok(entity)
}

fn expect_specialized<T>(res: Result<T, String>) -> T {
    match res {
        Ok(val) => val,
        Err(e) => panic!(e),
    }
}
