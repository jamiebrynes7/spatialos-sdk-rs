use std::collections::hash_map::HashMap;
use schema_bundle::*;

fn generate_identifier(identifier: &Identifier) -> String {
    identifier.path.iter().cloned().fold("".to_string(), |type_name, next| type_name + "::" + &next )
}

#[derive(Template)]
#[TemplatePath = "./src/generated_code.tt.rs"]
struct GeneratedCode {
    enums: HashMap<String, EnumDefinition>,
    types: HashMap<String, TypeDefinition>,
    components: HashMap<String, ComponentDefinition>
}

impl GeneratedCode {
    fn resolve_type_reference(&self, reference: &TypeReference) -> &TypeDefinition {
        self.types.get(&reference.qualified_name).unwrap()
    }

    fn resolve_enum_reference(&self, reference: &EnumReference) -> &EnumDefinition {
        self.enums.get(&reference.qualified_name).unwrap()
    }
    
    fn generate_value_type_reference(&self, value_reference: &ValueTypeReference) -> String {
        if let Some(ref primitive) = value_reference.primitive_reference {
            match primitive {
                PrimitiveType::Invalid => panic!("Encountered invalid primitive."),
                PrimitiveType::Int32 | PrimitiveType::Sint32 | PrimitiveType::Sfixed32 => "i32",
                PrimitiveType::Int64 | PrimitiveType::Sint64 | PrimitiveType::Sfixed64 => "i64",
                PrimitiveType::Uint32 | PrimitiveType::Fixed32 => "u32",
                PrimitiveType::Uint64 | PrimitiveType::Fixed64 => "u64",
                PrimitiveType::Bool => "bool",
                PrimitiveType::Float => "f32",
                PrimitiveType::Double => "f64",
                PrimitiveType::String => "String",
                PrimitiveType::EntityId => "worker::EntityId",
                PrimitiveType::Bytes => "Vec<u8>"
            }.to_string()
        } else if let Some(ref enum_ref) = value_reference.enum_reference {
            generate_identifier(&self.resolve_enum_reference(&enum_ref).identifier)
        } else if let Some(ref type_ref) = value_reference.type_reference {
            generate_identifier(&self.resolve_type_reference(&type_ref).identifier)
        } else {
            panic!("Unknown value type reference. {:?}", value_reference);
        }
    }

    fn generate_field_type(&self, field: &FieldDefinition) -> String {
        if let Some(ref singular) = field.singular_type {
            self.generate_value_type_reference(&singular.type_reference)
        } else if let Some(ref option) = field.option_type {
            format!("Option<{}>", self.generate_value_type_reference(&option.inner_type))
        } else if let Some(ref list) = field.list_type {
            format!("Vec<{}>", self.generate_value_type_reference(&list.inner_type))
        } else if let Some(ref map) = field.map_type {
            format!("HashMap<{}, {}>",
                self.generate_value_type_reference(&map.key_type),
                self.generate_value_type_reference(&map.value_type))
        } else {
            panic!("Unknown field type. {:?}", field)
        }
    }
}

pub fn generate_code(bundle: SchemaBundle) -> String {
    let v1 = bundle.v1.unwrap();
    let mut generated_code = GeneratedCode {
        enums: HashMap::new(),
        types: HashMap::new(),
        components: HashMap::new()
    };
    for enum_def in v1.enum_definitions {
        generated_code.enums.insert(enum_def.identifier.qualified_name.clone(), enum_def);
    }
    for type_def in v1.type_definitions {
        generated_code.types.insert(type_def.identifier.qualified_name.clone(), type_def);
    }
    for component in v1.component_definitions {
        generated_code.components.insert(component.identifier.qualified_name.clone(), component);
    }
    format!("{}", generated_code)
}