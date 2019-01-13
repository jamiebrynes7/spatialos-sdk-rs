use std::collections::hash_map::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub qualified_name: String,
    pub name: String,
    pub path: Vec<String>
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeReference {
    pub qualified_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumReference {
    pub qualified_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueReference {
    pub qualified_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldReference {
    pub qualified_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValueTypeReference {
    #[serde(rename = "primitive")]
    pub primitive_reference: Option<PrimitiveType>,
    #[serde(rename = "enum")]
    pub enum_reference: Option<EnumReference>,
    #[serde(rename = "type")]
    pub type_reference: Option<TypeReference>
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
pub struct Value_MapValue_MapPairValue {
  pub key: Value,
  pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_MapValue {
  pub values: Vec<Value_MapValue_MapPairValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
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
  pub enum_value: Option<EnumValue>,
  pub type_value: Option<TypeValue>,
  pub option_value: Option<Value_OptionValue>,
  pub list_value: Option<Value_ListValue>,
  pub map_value: Option<Value_MapValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
  pub enum_value: EnumValueReference,
  #[serde(rename = "enum")]
  pub enum_reference: EnumReference,
  pub name: String,
  pub value: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeValue_FieldValue {
  pub field: FieldReference,
  pub name: String,
  pub number: u32,
  pub value: Value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeValue {
  #[serde(rename = "type")]
  pub type_reference: TypeReference,
  pub fields: Vec<TypeValue_FieldValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
  pub type_value: TypeValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueDefinition {
    pub identifier: Identifier,
    pub value: u32,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub identifier: Identifier,
    pub value_definitions: Vec<EnumValueDefinition>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldDefinition_SingularType {
    #[serde(rename = "type")]
    pub type_reference: ValueTypeReference
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_OptionType {
    pub inner_type: ValueTypeReference
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_ListType {
    pub inner_type: ValueTypeReference
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_MapType {
    pub key_type: ValueTypeReference,
    pub value_type: ValueTypeReference
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    pub identifier: Identifier,
    pub field_id: u32,
    pub transient: bool,
    pub singular_type: Option<FieldDefinition_SingularType>,
    pub option_type: Option<FieldDefinition_OptionType>,
    pub list_type: Option<FieldDefinition_ListType>,
    pub map_type: Option<FieldDefinition_MapType>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinition {
    pub identifier: Identifier,
    pub field_definitions: Vec<FieldDefinition>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition_EventDefinition {
    pub identifier: Identifier,
    pub event_index: u32,
    #[serde(rename="type")]
    pub type_reference: ValueTypeReference,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition_CommandDefinition {
    pub identifier: Identifier,
    pub command_index: u32,
    pub request_type: ValueTypeReference,
    pub response_type: ValueTypeReference,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition {
    pub identifier: Identifier,
    pub component_id: u32,
    pub data_definition: Option<TypeReference>,
    pub field_definitions: Vec<FieldDefinition>,
    pub event_definitions: Vec<ComponentDefinition_EventDefinition>,
    pub command_definitions: Vec<ComponentDefinition_CommandDefinition>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBundleV1 {
    pub enum_definitions: Vec<EnumDefinition>,
    pub type_definitions: Vec<TypeDefinition>,
    pub component_definitions: Vec<ComponentDefinition>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    pub file_path: String,
    pub line: u32,
    pub column: u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSourceMapV1 {
    pub source_references: HashMap<String, SourceReference>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBundle {
    pub v1: Option<SchemaBundleV1>,
    pub source_map_v1: Option<SchemaSourceMapV1>
}

pub fn load_bundle(data: &str) -> Result<SchemaBundle, serde_json::Error> {
    serde_json::from_str::<SchemaBundle>(data)
}