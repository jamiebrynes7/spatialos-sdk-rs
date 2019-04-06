use crate::generated::improbable::*;
use approx;
use spatialos_sdk::worker::component::Component;
use spatialos_sdk::worker::entity_builder::EntityBuilder;

#[test]
fn position_is_serialized_correctly() {
    let entity = EntityBuilder::new(10.0, -10.0, 7.5, "rusty")
        .build()
        .unwrap();

    let maybe_position = entity.get::<Position>();
    assert!(maybe_position.is_some());

    let position = maybe_position.unwrap();

    approx::abs_diff_eq!(10.0, position.coords.x);
    approx::abs_diff_eq!(-10.0, position.coords.y);
    approx::abs_diff_eq!(7.5, position.coords.z);
}

#[test]
fn entity_acl_is_serialized_correctly() {
    let entity = EntityBuilder::new(0.0, 0.0, 0.0, "position_acl")
        .add_component(
            Metadata {
                entity_type: "test".to_owned(),
            },
            "metadata_acl",
        )
        .set_entity_acl_write_access(EntityAcl::ID, "entity_acl_acl")
        .add_read_access(&["client", "server"])
        .build()
        .unwrap();

    let maybe_acl = entity.get::<EntityAcl>();
    assert!(maybe_acl.is_some());

    let acl = maybe_acl.unwrap();

    // First check that we insert each layer into a different set.
    assert_eq!(2, acl.read_acl.attribute_set.len());

    let read_acl_layers: Vec<String> = acl
        .read_acl
        .attribute_set
        .iter()
        .flat_map(|requirement_set| requirement_set.attribute.clone())
        .collect();

    // Then check that both layers exist in the combined set.
    assert!(read_acl_layers.contains(&"client".to_owned()));
    assert!(read_acl_layers.contains(&"server".to_owned()));

    // Check that the correct number of write ACL exists.
    assert_eq!(3, acl.component_write_acl.len());

    // Test that position is correctly inserted.
    let maybe_position_acl = acl.component_write_acl.get(&Position::ID);
    assert!(maybe_position_acl.is_some());
    let position_acl = maybe_position_acl.unwrap();
    assert_eq!(1, position_acl.attribute_set.len());
    assert!(position_acl.attribute_set[0]
        .attribute
        .contains(&"position_acl".to_owned()));

    // Test that arbitrary components are correctly inserted.
    let maybe_metadata_acl = acl.component_write_acl.get(&Metadata::ID);
    assert!(maybe_metadata_acl.is_some());
    let metadata_acl = maybe_metadata_acl.unwrap();
    assert_eq!(1, metadata_acl.attribute_set.len());
    assert!(metadata_acl.attribute_set[0]
        .attribute
        .contains(&"metadata_acl".to_owned()));

    // Test that set write access is correctly inserted.
    let maybe_entity_acl_acl = acl.component_write_acl.get(&EntityAcl::ID);
    assert!(maybe_entity_acl_acl.is_some());
    let entity_acl_acl = maybe_entity_acl_acl.unwrap();
    assert_eq!(1, entity_acl_acl.attribute_set.len());
    assert!(entity_acl_acl.attribute_set[0]
        .attribute
        .contains(&"entity_acl_acl".to_owned()));
}

#[test]
fn metadata_is_serialized_correctly() {
    let entity = EntityBuilder::new(0.0, 0.0, 0.0, "rusty")
        .set_metadata("my_entity", "rusty")
        .build()
        .unwrap();

    let maybe_metadata = entity.get::<Metadata>();
    assert!(maybe_metadata.is_some());
    let metadata = maybe_metadata.unwrap();

    assert_eq!("my_entity", metadata.entity_type);
}

#[test]
fn persistence_component_is_added_if_set() {
    let entity = EntityBuilder::new(0.0, 0.0, 0.0, "rusty")
        .set_persistent("rusty")
        .build()
        .unwrap();

    assert!(entity.get::<Persistence>().is_some());
}

#[test]
fn error_is_returned_if_invalid_entity() {
    let result = EntityBuilder::new(0.0, 0.0, 0.0, "rusty")
        .add_component(
            Position {
                coords: Coordinates {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            "rusty",
        )
        .build();

    assert!(result.is_err());
}
