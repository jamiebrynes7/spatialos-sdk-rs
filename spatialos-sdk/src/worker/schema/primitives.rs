use super::{SchemaFieldContainer, SchemaPrimitiveField};
use spatialos_sdk_sys::worker::*;

macro_rules! impl_primitive_field {
    ($rust_type:ty, $schema_type:ident, $schema_get:ident, $schema_index:ident, $schema_count:ident, $schema_add:ident, $schema_add_list:ident) => {
        #[derive(Debug)]
        pub struct $schema_type;

        impl<'a> SchemaPrimitiveField<$rust_type> for SchemaFieldContainer<'a, $schema_type> {
            fn get_or_default(&self) -> $rust_type {
                unsafe { $schema_get(self.container.internal, self.field_id) }
            }
            fn index(&self, index: usize) -> $rust_type {
                unsafe { $schema_index(self.container.internal, self.field_id, index as u32) }
            }
            fn count(&self) -> usize {
                unsafe { $schema_count(self.container.internal, self.field_id) as usize }
            }

            fn add(&mut self, value: $rust_type) {
                unsafe {
                    $schema_add(self.container.internal, self.field_id, value);
                }
            }
            fn add_list(&mut self, value: &[$rust_type]) {
                unsafe {
                    let ptr = value.as_ptr();
                    $schema_add_list(
                        self.container.internal,
                        self.field_id,
                        ptr,
                        value.len() as u32,
                    );
                }
            }
        }
    };
}

impl_primitive_field!(
    f32,
    SchemaFloat,
    Schema_GetFloat,
    Schema_IndexFloat,
    Schema_GetFloatCount,
    Schema_AddFloat,
    Schema_AddFloatList
);
impl_primitive_field!(
    f64,
    SchemaDouble,
    Schema_GetDouble,
    Schema_IndexDouble,
    Schema_GetDoubleCount,
    Schema_AddDouble,
    Schema_AddDoubleList
);
impl_primitive_field!(
    i32,
    SchemaInt32,
    Schema_GetInt32,
    Schema_IndexInt32,
    Schema_GetInt32Count,
    Schema_AddInt32,
    Schema_AddInt32List
);
impl_primitive_field!(
    i64,
    SchemaInt64,
    Schema_GetInt64,
    Schema_IndexInt64,
    Schema_GetInt64Count,
    Schema_AddInt64,
    Schema_AddInt64List
);
impl_primitive_field!(
    u32,
    SchemaUint32,
    Schema_GetUint32,
    Schema_IndexUint32,
    Schema_GetUint32Count,
    Schema_AddUint32,
    Schema_AddUint32List
);
impl_primitive_field!(
    u64,
    SchemaUint64,
    Schema_GetUint64,
    Schema_IndexUint64,
    Schema_GetUint64Count,
    Schema_AddUint64,
    Schema_AddUint64List
);
impl_primitive_field!(
    i32,
    SchemaSint32,
    Schema_GetSint32,
    Schema_IndexSint32,
    Schema_GetSint32Count,
    Schema_AddSint32,
    Schema_AddSint32List
);
impl_primitive_field!(
    i64,
    SchemaSint64,
    Schema_GetSint64,
    Schema_IndexSint64,
    Schema_GetSint64Count,
    Schema_AddSint64,
    Schema_AddSint64List
);
impl_primitive_field!(
    u32,
    SchemaFixed32,
    Schema_GetFixed32,
    Schema_IndexFixed32,
    Schema_GetFixed32Count,
    Schema_AddFixed32,
    Schema_AddFixed32List
);
impl_primitive_field!(
    u64,
    SchemaFixed64,
    Schema_GetFixed64,
    Schema_IndexFixed64,
    Schema_GetFixed64Count,
    Schema_AddFixed64,
    Schema_AddFixed64List
);
impl_primitive_field!(
    i32,
    SchemaSfixed32,
    Schema_GetSfixed32,
    Schema_IndexSfixed32,
    Schema_GetSfixed32Count,
    Schema_AddSfixed32,
    Schema_AddSfixed32List
);
impl_primitive_field!(
    i64,
    SchemaSfixed64,
    Schema_GetSfixed64,
    Schema_IndexSfixed64,
    Schema_GetSfixed64Count,
    Schema_AddSfixed64,
    Schema_AddSfixed64List
);
impl_primitive_field!(
    u32,
    SchemaEnum,
    Schema_GetEnum,
    Schema_IndexEnum,
    Schema_GetEnumCount,
    Schema_AddEnum,
    Schema_AddEnumList
);

#[derive(Debug)]
pub struct SchemaBool;
#[derive(Debug)]
pub struct SchemaEntityId;
#[derive(Debug)]
pub struct SchemaBytes;
#[derive(Debug)]
pub struct SchemaString;
