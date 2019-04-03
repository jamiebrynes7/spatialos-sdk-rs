use crate::worker::component::ComponentId;
use spatialos_sdk_sys::worker::*;

pub type FieldId = u32;

#[derive(Debug)]
pub struct SchemaComponentUpdate {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentUpdate,
}

impl SchemaComponentUpdate {
    pub fn new(component_id: ComponentId) -> SchemaComponentUpdate {
        SchemaComponentUpdate {
            component_id,
            internal: unsafe { Schema_CreateComponentUpdate(component_id) },
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentUpdateComponentId(self.internal) }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) },
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) },
        }
    }

    pub fn events(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) },
        }
    }

    pub fn events_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) },
        }
    }

    // TODO: Cleared fields.
}

#[derive(Debug)]
pub struct SchemaComponentData {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentData,
}

impl SchemaComponentData {
    pub fn new(component_id: ComponentId) -> SchemaComponentData {
        SchemaComponentData {
            component_id,
            internal: unsafe { Schema_CreateComponentData(component_id) },
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.internal) }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) },
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) },
        }
    }
}

#[derive(Debug)]
pub struct SchemaCommandRequest {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandRequest,
}

impl SchemaCommandRequest {
    pub fn new(component_id: ComponentId, command_index: FieldId) -> SchemaCommandRequest {
        SchemaCommandRequest {
            component_id,
            internal: unsafe { Schema_CreateCommandRequest(component_id, command_index) },
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetCommandRequestComponentId(self.internal) }
    }

    pub fn command_index(&self) -> FieldId {
        unsafe { Schema_GetCommandRequestCommandIndex(self.internal) }
    }

    pub fn object(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) },
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) },
        }
    }
}

#[derive(Debug)]
pub struct SchemaCommandResponse {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandResponse,
}

impl SchemaCommandResponse {
    pub fn new(component_id: u32, command_index: u32) -> SchemaCommandResponse {
        SchemaCommandResponse {
            component_id,
            internal: unsafe { Schema_CreateCommandResponse(component_id, command_index) },
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetCommandResponseComponentId(self.internal) }
    }

    pub fn command_index(&self) -> FieldId {
        unsafe { Schema_GetCommandResponseCommandIndex(self.internal) }
    }

    pub fn object(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) },
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) },
        }
    }
}

#[derive(Debug)]
pub struct SchemaObject {
    internal: *mut Schema_Object,
}

impl SchemaObject {
    pub fn get<T: SchemaType>(&self, field: FieldId) -> Result<T::RustType, String> {
        T::from_field(self, field)
    }
}

// =================================================================================================
// Schema Conversion Traits
// =================================================================================================

/// A type that can be represented as serialized data in a field of a `SchemaObject`.
pub trait SchemaType {
    type RustType: Sized;

    fn from_field(schema_object: &SchemaObject, field: FieldId) -> Result<Self::RustType, String>;

    fn field_count(schema_object: &SchemaObject, field: FieldId) -> u32;
}

/// A type that can be deserialized from an entire `SchemaObject`.
pub trait SchemaObjectType: SchemaType<RustType = Self>
where
    Self: Sized,
{
    fn from_schema_object(input: &SchemaObject) -> Result<Self, String>;
}

// =================================================================================================
// Schema Conversion Implementations for Primitive Types
// =================================================================================================

macro_rules! impl_primitive_field {
    (
        $rust_type:ty,
        $schema_type:ident,
        $schema_get:ident,
        $schema_index:ident,
        $schema_count:ident,
        $schema_add:ident,
        $schema_add_list:ident,
    ) => {
        #[derive(Debug)]
        pub struct $schema_type;

        impl SchemaType for $schema_type {
            type RustType = $rust_type;

            fn from_field(input: &SchemaObject, field: FieldId) -> Result<Self::RustType, String> {
                Ok(unsafe { $schema_get(input.internal, field) })
            }

            fn field_count(input: &SchemaObject, field: FieldId) -> u32 {
                unsafe { $schema_count(input.internal, field) }
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
    Schema_AddFloatList,
);
impl_primitive_field!(
    f64,
    SchemaDouble,
    Schema_GetDouble,
    Schema_IndexDouble,
    Schema_GetDoubleCount,
    Schema_AddDouble,
    Schema_AddDoubleList,
);
impl_primitive_field!(
    i32,
    SchemaInt32,
    Schema_GetInt32,
    Schema_IndexInt32,
    Schema_GetInt32Count,
    Schema_AddInt32,
    Schema_AddInt32List,
);
impl_primitive_field!(
    i64,
    SchemaInt64,
    Schema_GetInt64,
    Schema_IndexInt64,
    Schema_GetInt64Count,
    Schema_AddInt64,
    Schema_AddInt64List,
);
impl_primitive_field!(
    u32,
    SchemaUint32,
    Schema_GetUint32,
    Schema_IndexUint32,
    Schema_GetUint32Count,
    Schema_AddUint32,
    Schema_AddUint32List,
);
impl_primitive_field!(
    u64,
    SchemaUint64,
    Schema_GetUint64,
    Schema_IndexUint64,
    Schema_GetUint64Count,
    Schema_AddUint64,
    Schema_AddUint64List,
);
impl_primitive_field!(
    i32,
    SchemaSint32,
    Schema_GetSint32,
    Schema_IndexSint32,
    Schema_GetSint32Count,
    Schema_AddSint32,
    Schema_AddSint32List,
);
impl_primitive_field!(
    i64,
    SchemaSint64,
    Schema_GetSint64,
    Schema_IndexSint64,
    Schema_GetSint64Count,
    Schema_AddSint64,
    Schema_AddSint64List,
);
impl_primitive_field!(
    u32,
    SchemaFixed32,
    Schema_GetFixed32,
    Schema_IndexFixed32,
    Schema_GetFixed32Count,
    Schema_AddFixed32,
    Schema_AddFixed32List,
);
impl_primitive_field!(
    u64,
    SchemaFixed64,
    Schema_GetFixed64,
    Schema_IndexFixed64,
    Schema_GetFixed64Count,
    Schema_AddFixed64,
    Schema_AddFixed64List,
);
impl_primitive_field!(
    i32,
    SchemaSfixed32,
    Schema_GetSfixed32,
    Schema_IndexSfixed32,
    Schema_GetSfixed32Count,
    Schema_AddSfixed32,
    Schema_AddSfixed32List,
);
impl_primitive_field!(
    i64,
    SchemaSfixed64,
    Schema_GetSfixed64,
    Schema_IndexSfixed64,
    Schema_GetSfixed64Count,
    Schema_AddSfixed64,
    Schema_AddSfixed64List,
);
impl_primitive_field!(
    i64,
    SchemaEntityId,
    Schema_GetEntityId,
    Schema_IndexEntityId,
    Schema_GetEntityIdCount,
    Schema_AddEntityId,
    Schema_AddEntityIdList,
);
impl_primitive_field!(
    u32,
    SchemaEnum,
    Schema_GetEnum,
    Schema_IndexEnum,
    Schema_GetEnumCount,
    Schema_AddEnum,
    Schema_AddEnumList,
);

impl<T: SchemaType> SchemaType for Option<T> {
    type RustType = Option<T::RustType>;

    fn from_field(schema_object: &SchemaObject, field: FieldId) -> Result<Self::RustType, String> {
        let count = T::field_count(schema_object, field);
        match count {
            0 => Ok(None),
            1 => Ok(Some(T::from_field(schema_object, field)?)),
            _ => panic!(
                "Invalid count {} for `option` schema field {}",
                count, field
            ),
        }
    }

    fn field_count(schema_object: &SchemaObject, field: FieldId) -> u32 {
        T::field_count(schema_object, field)
    }
}
