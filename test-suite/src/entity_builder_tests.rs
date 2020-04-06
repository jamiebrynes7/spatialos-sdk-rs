use crate::generated::improbable::*;
use approx;
use spatialos_sdk::worker::component::Component;
use spatialos_sdk::worker::entity_builder::EntityBuilder;

#[test]
fn position_is_serialized_correctly() {
    let builder = EntityBuilder::new(10.0, -10.0, 7.5, "rusty");
    let entity = builder.build().unwrap();

    let position = entity
        .get::<Position>()
        .expect("No `Position` component found")
        .expect("Failed to deserialize `Position`");

    approx::abs_diff_eq!(10.0, position.coords.x.0);
    approx::abs_diff_eq!(-10.0, position.coords.y.0);
    approx::abs_diff_eq!(7.5, position.coords.z.0);
}

#[test]
fn entity_acl_is_serialized_correctly() {
    let mut builder = EntityBuilder::new(0.0, 0.0, 0.0, "position_acl");
    builder.add_component(
        Metadata {
            entity_type: "test".to_owned(),
        },
        "metadata_acl",
    );
    builder.set_entity_acl_write_access("entity_acl_acl");
    builder.add_read_access("client");
    builder.add_read_access("server");

    let entity = builder.build().unwrap();

    let acl = entity
        .get::<EntityAcl>()
        .expect("No `EntityAcl` component found")
        .expect("Failed to deserialize `EntityAcl`");

    // First check that we insert each layer into a different set.
    assert_eq!(5, acl.read_acl.attribute_set.len());

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
    let metadata_acl = acl
        .component_write_acl
        .get(&Metadata::ID)
        .expect("No entry for `Metadata` in `EntityAcl::component_write_acl`");
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
    let mut builder = EntityBuilder::new(0.0, 0.0, 0.0, "rusty");
    builder.set_metadata("my_entity", "rusty");
    let entity = builder.build().unwrap();

    let metadata = entity
        .get::<Metadata>()
        .expect("No `Metadata` component found")
        .expect("Failed to deserialize `Metadata`");

    assert_eq!("my_entity", metadata.entity_type);
}

#[test]
fn persistence_component_is_added_if_set() {
    let mut builder = EntityBuilder::new(0.0, 0.0, 0.0, "rusty");
    builder.set_persistent("rusty");
    let entity = builder.build().unwrap();

    assert!(entity.get::<Persistence>().is_some());
}

#[test]
fn error_is_returned_if_invalid_entity() {
    let mut builder = EntityBuilder::new(0.0, 0.0, 0.0, "rusty");
    builder.add_component(
        Position {
            coords: Coordinates {
                x: 0.0.into(),
                y: 0.0.into(),
                z: 0.0.into(),
            },
        },
        "rusty",
    );
    let result = builder.build();

    assert!(result.is_err());
}
