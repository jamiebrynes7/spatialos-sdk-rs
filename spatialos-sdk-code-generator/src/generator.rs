use std::collections::hash_map::HashMap;
use schema_bundle::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::hash_set::HashSet;

fn generate_identifier(identifier: &Identifier) -> String {
    identifier.path.iter().cloned().fold("generated_code".to_string(), |type_name, next| type_name + "::" + &next)
}

fn get_package_path(identifier: &Identifier) -> &[String] {
    let path = identifier.path.as_slice();
    return &path[..path.len() - 1]
}

fn get_primitive_type(primitive_type: &PrimitiveType) -> &str {
    match primitive_type {
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
    }
}

#[derive(Debug)]
#[derive(Template)]
#[TemplatePath = "./src/generated_code_mod.tt.rs"]
struct Package {
    generated_code: Rc<RefCell<GeneratedCode>>,
    name: String,
    depth: usize,
    subpackages: HashMap<String, Package>,
    enums: HashSet<String>,
    types: HashSet<String>,
    components: HashSet<String>
}

impl Package {
    fn new(generated_code: Rc<RefCell<GeneratedCode>>, name: &str, depth: usize) -> Package {
        Package {
            generated_code: generated_code,
            name: name.to_string(),
            depth: depth,
            subpackages: HashMap::new(),
            enums: HashSet::new(),
            types: HashSet::new(),
            components: HashSet::new()
        }
    }

    fn root_module(&self) -> String {
        if self.depth == 0 {
            "self".to_string()
        } else {
            vec!["super".to_string(); self.depth].join("::")
        }
    }

    fn get_enum_definition(&self, qualified_name: &str) -> EnumDefinition {
       self.generated_code.borrow().enums.get(&qualified_name.to_string()).unwrap().clone()
    }

    fn get_type_definition(&self, qualified_name: &str) -> TypeDefinition {
       self.generated_code.borrow().types.get(&qualified_name.to_string()).unwrap().clone()
    }

    fn get_component_definition(&self, qualified_name: &str) -> ComponentDefinition {
       self.generated_code.borrow().components.get(&qualified_name.to_string()).unwrap().clone()
    }

    fn resolve_enum_reference(&self, reference: &EnumReference) -> EnumDefinition {
       self.generated_code.borrow().resolve_enum_reference(reference).clone()
    }

    fn resolve_type_reference(&self, reference: &TypeReference) -> TypeDefinition {
       self.generated_code.borrow().resolve_type_reference(reference).clone()
    }

    fn get_component_fields(&self, component: &ComponentDefinition) -> Vec<FieldDefinition> {
        let data_type_ref = component.data_definition.type_reference.as_ref().unwrap();
        let data_type = self.resolve_type_reference(data_type_ref);
        data_type.field_definitions.clone()
    }
    
    fn generate_value_type_reference(&self, value_type: &ValueTypeReference) -> String {
       self.generated_code.borrow().generate_value_type_reference(value_type)
    }

    fn generate_field_type(&self, field: &FieldDefinition) -> String {
       self.generated_code.borrow().generate_field_type(field)
    }

    fn serialize_update_field(&self, field: &FieldDefinition, update: &str, schema_object: &str) -> String {
        "UNKNOWN (UpdateField)".to_string()
    }

    fn serialize_field(&self, field: &FieldDefinition, data: &str, schema_object: &str) -> String {
        if let Some(ref singular_type) = field.singular_type {
            self.serialize_type(field.field_id, &singular_type.type_reference, &format!("{}.{}", data, field.identifier.name), schema_object)
        } else if let Some(ref option_type) = field.option_type {
            "UNKNOWN (OptionType)".to_string()
        } else if let Some(ref list_type) = field.list_type {
            "UNKNOWN (ListType)".to_string()
        } else if let Some(ref map_type) = field.map_type {
            "UNKNOWN (MapType)".to_string()
        } else {
            panic!("Field doesn't have a type. {:?}", field);
        }
    }

    // Generates an expression which serializes a value from an expression into a schema object.
    fn serialize_type(&self, field_id: u32, value_type: &ValueTypeReference, expression: &str, schema_object: &str) -> String {
        if let Some(ref primitive_type) = value_type.primitive_reference {
            format!("{}.field::<{}>.add({})", schema_object, get_primitive_type(&primitive_type), expression)
        } else if let Some(ref enum_type) = value_type.enum_reference {
            format!("{}.field::<u32>.add({} as u32)", schema_object, expression)
        } else if let Some(ref type_ref) = value_type.type_reference {
            "UNKNOWN (TypeRef)".to_string()
        } else {
            panic!("Unknown value type reference. {:?}", value_type);
        }
    }

    // Generates an expression which deserializes a field from a schema component update fields object.
    fn deserialize_update_field(&self, field: &FieldDefinition, schema_object: &str) -> String {
        if let Some(ref singular_type) = field.singular_type {
            self.deserialize_type(field.field_id, &singular_type.type_reference, schema_object, true)
        } else if let Some(ref option_type) = field.option_type {
            "UNKNOWN (OptionType Update)".to_string()
        } else if let Some(ref list_type) = field.list_type {
            "UNKNOWN (ListType Update)".to_string()
        } else if let Some(ref map_type) = field.map_type {
            "UNKNOWN (MapType Update)".to_string()
        } else {
            panic!("Field doesn't have a type. {:?}", field);
        }
    }
    
    // Generates an expression which deserializes a field from a schema object.
    fn deserialize_field(&self, field: &FieldDefinition, schema_object: &str) -> String {
        if let Some(ref singular_type) = field.singular_type {
            self.deserialize_type(field.field_id, &singular_type.type_reference, schema_object, false)
        } else if let Some(ref option_type) = field.option_type {
            "UNKNOWN (OptionType)".to_string()
        } else if let Some(ref list_type) = field.list_type {
            "UNKNOWN (ListType)".to_string()
        } else if let Some(ref map_type) = field.map_type {
            "UNKNOWN (MapType)".to_string()
        } else {
            panic!("Field doesn't have a type. {:?}", field);
        }
    }

    // Generates an expression which deserializes a value from a schema object.
    fn deserialize_type(&self, field_id: u32, value_type: &ValueTypeReference, schema_object: &str, option: bool) -> String {
        let field_get = if option { "get" } else { "get_or_default" };
        if let Some(ref primitive_type) = value_type.primitive_reference {
            format!("{}.field::<{}>.{}()", schema_object, get_primitive_type(&primitive_type), field_get)
        } else if let Some(ref enum_type) = value_type.enum_reference {
            format!("{}.field::<u32>.{}()", schema_object, field_get)
        } else if let Some(ref type_ref) = value_type.type_reference {
            "UNKNOWN (TypeRef)".to_string()
        } else {
            panic!("Unknown value type reference. {:?}", value_type);
        }
    }
}

#[derive(Debug)]
struct GeneratedCode {
    root_package: Option<Package>,
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
    
    fn generate_value_type_reference(&self, value_type: &ValueTypeReference) -> String {
        if let Some(ref primitive) = value_type.primitive_reference {
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
        } else if let Some(ref enum_ref) = value_type.enum_reference {
            generate_identifier(&self.resolve_enum_reference(&enum_ref).identifier)
        } else if let Some(ref type_ref) = value_type.type_reference {
            generate_identifier(&self.resolve_type_reference(&type_ref).identifier)
        } else {
            panic!("Unknown value type reference. {:?}", value_type);
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

fn get_or_create_package<'a>(package: &'a mut Package, path: &[String]) -> &'a mut Package {
    if path.is_empty() {
        return package;
    }
    let package_name = &path[0];
    if !package.subpackages.contains_key(package_name) {
        package.subpackages.insert(package_name.clone(), Package::new(Rc::clone(&package.generated_code), package_name, package.depth + 1));
    }
    return get_or_create_package(package.subpackages.get_mut(package_name).unwrap(), &path[1..])
}

fn generate_module(package: &Package) -> String {
    let submodules = if package.subpackages.len() > 0 {
        package.subpackages.iter().map(|(_, pkg)| {
            generate_module(&pkg)
        }).fold("".to_string(), |submodule, next| submodule + "\n" + &next)
    } else {
        "".to_string()
    };
    let module_contents = format!("{}\n{}", package, submodules);
    if package.depth == 0 {
        module_contents
    } else {
        format!("mod {} {{\n{}\n}}\n", package.name, module_contents)
    }
}

pub fn generate_code(bundle: SchemaBundle) -> String {
    let v1 = bundle.v1.unwrap();
    let mut generated_code = Rc::new(RefCell::new(
        GeneratedCode {
            root_package: None,
            enums: HashMap::new(),
            types: HashMap::new(),
            components: HashMap::new()
        }
    ));
    let mut root_package = Package::new(Rc::clone(&generated_code), "", 0);
    for enum_def in v1.enum_definitions {
        let mut package = get_or_create_package(&mut root_package, get_package_path(&enum_def.identifier));
        let qualified_name = enum_def.identifier.qualified_name.clone();
        generated_code.borrow_mut().enums.insert(qualified_name.clone(), enum_def);
        package.enums.insert(qualified_name.clone());
    }
    for type_def in v1.type_definitions {
        let mut package = get_or_create_package(&mut root_package, get_package_path(&type_def.identifier));
        let qualified_name = type_def.identifier.qualified_name.clone();
        generated_code.borrow_mut().types.insert(qualified_name.clone(), type_def);
        package.types.insert(qualified_name.clone());
    }
    for component in v1.component_definitions {
        let mut package = get_or_create_package(&mut root_package, get_package_path(&component.identifier));
        let qualified_name = component.identifier.qualified_name.clone();
        generated_code.borrow_mut().components.insert(qualified_name.clone(), component);
        package.components.insert(qualified_name.clone());
    }
    generated_code.borrow_mut().root_package = Some(root_package);
    //println!("{:#?}", generated_code.root_package.as_mut().unwrap());
    let generated_code_ref = generated_code.borrow();
    generate_module(&generated_code_ref.root_package.as_ref().unwrap())
}