mod command_request;
mod command_response;
mod component_data;
mod component_update;
mod object;
mod primitives;

pub mod owned;

pub use self::{
    command_request::*, command_response::*, component_data::*, component_update::*, object::*,
    owned::Owned, primitives::*,
};

pub(crate) use self::private::PointerType;

pub type FieldId = u32;

pub trait SchemaPrimitiveField {
    type RustType: Sized;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType;
    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Self::RustType;
    fn count(object: &SchemaObject, field: FieldId) -> usize;
    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType);
    fn add_list(object: &mut SchemaObject, field: FieldId, value: &[Self::RustType]);

    fn get_list(object: &SchemaObject, field: FieldId) -> Vec<Self::RustType> {
        let count = Self::count(object, field);
        let mut result = Vec::with_capacity(count);
        for index in 0..count {
            result.push(Self::index(object, field, index));
        }
        result
    }
}

mod private {
    pub unsafe trait PointerType: Sized {
        type Raw;

        unsafe fn from_raw<'a>(raw: *mut Self::Raw) -> &'a Self {
            &*(raw as *mut _)
        }

        unsafe fn from_raw_mut<'a>(raw: *mut Self::Raw) -> &'a mut Self {
            &mut *(raw as *mut _)
        }

        fn as_ptr(&self) -> *mut Self::Raw {
            self as *const _ as *mut _
        }
    }
}
