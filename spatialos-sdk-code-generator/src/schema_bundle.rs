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
pub struct TypeReference {
    #[serde(rename = "primitive")]
    pub primitive_reference: Option<PrimitiveType>,
    #[serde(rename = "enum")]
    pub enum_reference: Option<String>,
    #[serde(rename = "type")]
    pub type_reference: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_EnumValue {
    #[serde(rename = "enum")]
    pub enum_reference: String,
    pub value: String,
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
    pub type_reference: String,
    pub fields: Vec<Value_TypeValue_FieldValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_OptionValue {
    pub value: Option<Box<Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_ListValue {
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_MapValue_KeyValuePair {
    pub key: Value,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_MapValue {
    pub values: Vec<Value_MapValue_KeyValuePair>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub source_reference: SourceReference,
    pub bool_value: Option<bool>,
    pub uint32_value: Option<u32>,
    pub uint64_value: Option<u64>,
    pub int32_value: Option<i32>,
    pub int64_value: Option<i64>,
    pub float_value: Option<f32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub bytes_value: Option<String>,
    pub entity_id_value: Option<i64>,
    pub enum_value: Option<Value_EnumValue>,
    pub type_value: Option<Value_TypeValue>,
    pub option_value: Option<Value_OptionValue>,
    pub list_value: Option<Value_ListValue>,
    pub map_value: Option<Value_MapValue>,
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
pub struct FieldDefinition_SingularType {
    #[serde(rename = "type")]
    pub type_reference: TypeReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_OptionType {
    pub inner_type: TypeReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_ListType {
    pub inner_type: TypeReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_MapType {
    pub key_type: TypeReference,
    pub value_type: TypeReference,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    pub source_reference: SourceReference,
    pub annotations: Vec<Annotation>,
    pub name: String,
    pub field_id: u32,
    pub transient: bool,
    pub singular_type: Option<FieldDefinition_SingularType>,
    pub option_type: Option<FieldDefinition_OptionType>,
    pub list_type: Option<FieldDefinition_ListType>,
    pub map_type: Option<FieldDefinition_MapType>,
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
