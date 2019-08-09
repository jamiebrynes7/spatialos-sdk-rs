use serde::{Deserialize, Deserializer};

fn empty_string_is_none<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let o: Option<String> = Option::deserialize(d)?;
    Ok(o.filter(|s| !s.is_empty()))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    pub line: u32,
    pub column: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PrimitiveType {
    Invalid = 0,
    Int32 = 1,
    Int64 = 2,
    Uint32 = 3,
    Uint64 = 4,
    Sint32 = 5,
    Sint64 = 6,
    Fixed32 = 7,
    Fixed64 = 8,
    Sfixed32 = 9,
    Sfixed64 = 10,
    Bool = 11,
    Float = 12,
    Double = 13,
    String = 14,
    EntityId = 15,
    Bytes = 16,
    Entity = 17,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TypeReference {
    #[serde(rename = "primitive")]
    Primitive(PrimitiveType),
    #[serde(rename = "enum")]
    Enum(String),
    #[serde(rename = "type")]
    Type(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_TypeValue_FieldValue {
    pub source_reference: SourceReference,
    pub name: String,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_TypeValue {
    #[serde(rename = "type")]
    type_reference: String,
    fields: Vec<Value_TypeValue_FieldValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_MapValue_KeyValuePair {
    pub key: Value,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Value_Value {
    #[serde(rename = "boolValue")]
    BoolValue(bool),
    #[serde(rename = "uint32Value")]
    Uint32Value(u32),
    #[serde(rename = "uint64Value")]
    Uint64Value(u64),
    #[serde(rename = "int32Value")]
    Int32Value(i32),
    #[serde(rename = "int64Value")]
    Int64Value(i64),
    #[serde(rename = "floatValue")]
    FloatValue(f32),
    #[serde(rename = "doubleValue")]
    DoubleValue(f64),
    #[serde(rename = "stringValue")]
    StringValue(String),
    #[serde(rename = "bytesValue")]
    BytesValue(String),
    #[serde(rename = "entityIdValue")]
    EntityIdValue(i64),
    #[serde(rename = "enumValue")]
    EnumValue {
        #[serde(rename = "enum")]
        enum_reference: String,
        value: String,
    },
    #[serde(rename = "typeValue")]
    TypeValue {
        #[serde(rename = "type")]
        type_reference: String,
        fields: Vec<Value_TypeValue_FieldValue>,
    },
    #[serde(rename = "optionValue")]
    OptionValue { value: Box<Value> },
    #[serde(rename = "listValue")]
    ListValue { values: Vec<Value> },
    #[serde(rename = "mapValue")]
    MapValue {
        values: Vec<Value_MapValue_KeyValuePair>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub source_reference: SourceReference,
    #[serde(flatten)]
    pub value: Value_Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub source_reference: SourceReference,
    pub type_value: Value_TypeValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition_EnumValueDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub name: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub qualified_name: String,
    pub name: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub outer_type: Option<String>,
    pub values: Vec<EnumDefinition_EnumValueDefinition>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FieldDefinition_FieldType {
    #[serde(rename = "singularType")]
    Singular {
        #[serde(rename = "type")]
        type_reference: TypeReference,
    },
    #[serde(rename = "optionType")]
    Option {
        #[serde(rename = "innerType")]
        inner_type: TypeReference,
    },
    #[serde(rename = "listType")]
    List {
        #[serde(rename = "innerType")]
        inner_type: TypeReference,
    },
    #[serde(rename = "mapType")]
    Map {
        #[serde(rename = "keyType")]
        key_type: TypeReference,
        #[serde(rename = "valueType")]
        value_type: TypeReference,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub name: String,
    pub field_id: u32,
    pub transient: bool,
    #[serde(flatten)]
    pub field_type: FieldDefinition_FieldType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub qualified_name: String,
    pub name: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub outer_type: Option<String>,
    pub fields: Vec<FieldDefinition>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition_EventDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_reference: String,
    pub event_index: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition_CommandDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub name: String,
    pub request_type: String,
    pub response_type: String,
    pub command_index: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub qualified_name: String,
    pub name: String,
    pub component_id: u32,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub data_definition: Option<String>,
    pub fields: Vec<FieldDefinition>,
    pub events: Vec<ComponentDefinition_EventDefinition>,
    pub commands: Vec<ComponentDefinition_CommandDefinition>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaFile_Package {
    pub source_reference: SourceReference,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaFile_Import {
    pub source_reference: SourceReference,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaFile {
    pub canonical_path: String,
    pub package: SchemaFile_Package,
    pub imports: Vec<SchemaFile_Import>,
    pub enums: Vec<EnumDefinition>,
    pub types: Vec<TypeDefinition>,
    pub components: Vec<ComponentDefinition>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBundle {
    pub schema_files: Vec<SchemaFile>,
}

pub fn load_bundle(data: &str) -> Result<SchemaBundle, serde_json::Error> {
    serde_json::from_str::<SchemaBundle>(data)
}
