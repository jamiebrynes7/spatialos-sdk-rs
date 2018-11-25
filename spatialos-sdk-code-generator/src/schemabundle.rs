use std::collections::hash_map::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    qualified_name: String,
    name: String,
    path: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumReference {
    qualified_name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueDefinition {
    identifier: Identifier,
    value: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    identifier: Identifier,
    value_definitions: Vec<EnumValueDefinition>
}

#[derive(Serialize, Deserialize, Debug)]
enum PrimitiveType {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueTypeReference {
    #[serde(rename = "primitive")]
    primitive_reference: Option<PrimitiveType>,
    #[serde(rename = "enum")]
    enum_reference: Option<EnumReference>,
    #[serde(rename = "type")]
    type_reference: Option<TypeReference>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeReference {
    qualified_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldDefinition_SingularType {
    #[serde(rename = "type")]
    type_reference: ValueTypeReference
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_OptionType {
    inner_type: ValueTypeReference
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_ListType {
    inner_type: ValueTypeReference
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition_MapType {
    key_type: ValueTypeReference,
    value_type: ValueTypeReference
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    identifier: Identifier,
    field_id: u32,
    transient: bool,
    singular_type: Option<FieldDefinition_SingularType>,
    option_type: Option<FieldDefinition_OptionType>,
    list_type: Option<FieldDefinition_ListType>,
    map_type: Option<FieldDefinition_MapType>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinition {
    identifier: Identifier,
    field_definitions: Vec<FieldDefinition>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentEventDefinition {
    identifier: Identifier,
    event_index: u32,
    #[serde(rename="type")]
    type_reference: ValueTypeReference
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCommandDefinition {
    identifier: Identifier,
    command_index: u32,
    request_type: ValueTypeReference,
    response_type: ValueTypeReference
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition {
    identifier: Identifier,
    component_id: u32,
    data_definition: ValueTypeReference,
    event_definitions: Vec<ComponentEventDefinition>,
    command_definitions: Vec<ComponentCommandDefinition>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBundleV1 {
    enum_definitions: Vec<EnumDefinition>,
    type_definitions: Vec<TypeDefinition>,
    component_definitions: Vec<ComponentDefinition>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    file_path: String,
    line: u32,
    column: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSourceMapV1 {
    source_references: HashMap<String, SourceReference>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBundle {
    v1: Option<SchemaBundleV1>,
    source_map_v1: Option<SchemaSourceMapV1>
}

pub fn load_bundle(data: &str) -> Result<SchemaBundle, serde_json::Error> {
    serde_json::from_str::<SchemaBundle>(data)
}