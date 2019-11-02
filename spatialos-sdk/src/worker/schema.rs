mod command_request;
mod command_response;
mod component_data;
mod component_update;
mod object;
mod primitives;

pub use self::{
    command_request::*, command_response::*, component_data::*, component_update::*, object::*,
    primitives::*,
};

pub type FieldId = u32;

pub trait SchemaPrimitiveField {
    type RustType: Sized;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType;
    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Self::RustType;
    fn count(object: &SchemaObject, field: FieldId) -> usize;
    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType);
    fn add_list(object: &mut SchemaObject, field: FieldId, value: &[Self::RustType]);
}
