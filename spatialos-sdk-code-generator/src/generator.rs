use crate::schema_bundle::*;
use heck::CamelCase;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

fn primitive_type_name(primitive_type: &PrimitiveType) -> &'static str {
    match primitive_type {
        PrimitiveType::Invalid => panic!("Encountered invalid primitive."),
        PrimitiveType::Int32 => "SchemaInt32",
        PrimitiveType::Sint32 => "SchemaSint32",
        PrimitiveType::Sfixed32 => "SchemaSfixed32",
        PrimitiveType::Int64 => "SchemaInt64",
        PrimitiveType::Sint64 => "SchemaSint64",
        PrimitiveType::Sfixed64 => "SchemaSfixed64",
        PrimitiveType::Uint32 => "SchemaUint32",
        PrimitiveType::Fixed32 => "SchemaFixed32",
        PrimitiveType::Uint64 => "SchemaUint64",
        PrimitiveType::Fixed64 => "SchemaFixed64",
        PrimitiveType::Bool => "SchemaBool",
        PrimitiveType::Float => "SchemaFloat",
        PrimitiveType::Double => "SchemaDouble",
        PrimitiveType::String => "SchemaString",
        PrimitiveType::EntityId => "SchemaEntityId",
        PrimitiveType::Entity => panic!("Entity serialization unimplemented."),
        PrimitiveType::Bytes => "SchemaBytes",
    }
}

#[derive(Debug, Template)]
#[TemplatePath = "./src/generated_code_mod.tt.rs"]
struct Package {
    generated_code: Rc<RefCell<GeneratedCode>>,
    name: String,
    path: Vec<String>,
    subpackages: BTreeMap<String, Package>,
    enums: BTreeSet<String>,
    types: BTreeSet<String>,
    components: BTreeSet<String>,
}

#[allow(clippy::needless_bool)]
impl Package {
    fn new(generated_code: Rc<RefCell<GeneratedCode>>, name: &str, path: Vec<String>) -> Package {
        Package {
            generated_code,
            name: name.to_string(),
            path,
            subpackages: BTreeMap::new(),
            enums: BTreeSet::new(),
            types: BTreeSet::new(),
            components: BTreeSet::new(),
        }
    }

    pub fn get_subpackage(&self, package_part: &str) -> Option<&Package> {
        self.subpackages.get(package_part)
    }

    fn depth(&self) -> usize {
        self.path.len()
    }

    fn rust_name(&self, qualified_name: &str) -> String {
        let tokens: Vec<&str> = qualified_name.split('.').collect();
        tokens[self.path.len()..].join("_")
    }

    fn rust_fqname(&self, qualified_name: &str) -> String {
        let gen_code = self.generated_code.borrow();
        let identifier_package = gen_code.get_package(qualified_name);

        [
            "generated".to_string(),
            identifier_package.path.join("::"),
            identifier_package.rust_name(qualified_name),
        ]
        .join("::")
    }

    fn schema_type_name(&self, type_ref: &TypeReference) -> Cow<'static, str> {
        match type_ref {
            TypeReference::Primitive(prim) => primitive_type_name(prim).into(),
            TypeReference::Enum(name) => self.rust_fqname(name).into(),
            TypeReference::Type(name) => self.rust_fqname(name).into(),
        }
    }

    fn field_type_name(&self, field_ty: &FieldDefinition_FieldType) -> Cow<'static, str> {
        match field_ty {
            FieldDefinition_FieldType::Singular { type_reference } => {
                self.schema_type_name(type_reference)
            }

            FieldDefinition_FieldType::Option { inner_type } => {
                format!("Optional<{}>", self.schema_type_name(inner_type)).into()
            }

            FieldDefinition_FieldType::List { inner_type } => {
                format!("List<{}>", self.schema_type_name(inner_type)).into()
            }

            FieldDefinition_FieldType::Map {
                key_type,
                value_type,
            } => format!(
                "Map<{}, {}>",
                self.schema_type_name(key_type),
                self.schema_type_name(value_type),
            )
            .into(),
        }
    }

    fn get_enum_definition(&self, qualified_name: &str) -> EnumDefinition {
        self.generated_code
            .borrow()
            .enums
            .get(&qualified_name.to_string())
            .unwrap_or_else(|| panic!("Unable to find enum {}", qualified_name))
            .clone()
    }

    fn get_type_definition(&self, qualified_name: &str) -> TypeDefinition {
        self.generated_code
            .borrow()
            .types
            .get(&qualified_name.to_string())
            .unwrap_or_else(|| panic!("Unable to find type {}", qualified_name))
            .clone()
    }

    fn get_component_definition(&self, qualified_name: &str) -> ComponentDefinition {
        self.generated_code
            .borrow()
            .components
            .get(&qualified_name.to_string())
            .unwrap_or_else(|| panic!("Unable to find component {}", qualified_name))
            .clone()
    }

    fn resolve_enum_reference(&self, qualified_name: &str) -> EnumDefinition {
        self.generated_code
            .borrow()
            .resolve_enum_reference(qualified_name)
            .clone()
    }

    fn resolve_type_reference(&self, qualified_name: &str) -> TypeDefinition {
        self.generated_code
            .borrow()
            .resolve_type_reference(qualified_name)
            .clone()
    }

    fn get_component_fields(&self, component: &ComponentDefinition) -> Vec<FieldDefinition> {
        if let Some(ref data_definition) = component.data_definition {
            let data_type = self.resolve_type_reference(&data_definition);
            data_type.fields.clone()
        } else {
            component.fields.clone()
        }
    }

    fn generate_rust_type_name(&self, value_type: &TypeReference) -> String {
        match value_type {
            TypeReference::Primitive(ref primitive) => match primitive {
                PrimitiveType::Invalid => panic!("Encountered invalid primitive."),
                PrimitiveType::Int32 | PrimitiveType::Sint32 | PrimitiveType::Sfixed32 => "i32",
                PrimitiveType::Int64 | PrimitiveType::Sint64 | PrimitiveType::Sfixed64 => "i64",
                PrimitiveType::Uint32 | PrimitiveType::Fixed32 => "u32",
                PrimitiveType::Uint64 | PrimitiveType::Fixed64 => "u64",
                PrimitiveType::Bool => "bool",
                PrimitiveType::Float => "f32",
                PrimitiveType::Double => "f64",
                PrimitiveType::String => "String",
                PrimitiveType::EntityId => "spatialos_sdk::worker::EntityId",
                PrimitiveType::Entity => panic!("Entity serialization unimplemented."),
                PrimitiveType::Bytes => "Vec<u8>",
            }
            .to_string(),
            TypeReference::Enum(ref enum_ref) => {
                self.rust_fqname(&self.resolve_enum_reference(&enum_ref).qualified_name)
            }
            TypeReference::Type(ref type_ref) => {
                self.rust_fqname(&self.resolve_type_reference(&type_ref).qualified_name)
            }
        }
    }

    fn generate_field_type(&self, field: &FieldDefinition) -> String {
        match field.field_type {
            FieldDefinition_FieldType::Singular { ref type_reference } => {
                self.generate_rust_type_name(type_reference)
            }
            FieldDefinition_FieldType::Option { ref inner_type } => {
                format!("Option<{}>", self.generate_rust_type_name(inner_type))
            }
            FieldDefinition_FieldType::List { ref inner_type } => {
                format!("Vec<{}>", self.generate_rust_type_name(inner_type))
            }
            FieldDefinition_FieldType::Map {
                ref key_type,
                ref value_type,
            } => format!(
                "BTreeMap<{}, {}>",
                self.generate_rust_type_name(key_type),
                self.generate_rust_type_name(value_type)
            ),
        }
    }

    // Generates an expression which serializes a field from an expression into a schema object. The generated
    // expression should always have type ().
    fn serialize_field(&self, field: &FieldDefinition, schema_object: &str) -> String {
        format!(
            "{}.add::<{}>({}, &self.{})",
            schema_object,
            self.field_type_name(&field.field_type),
            field.field_id,
            field.name,
        )
    }

    // Generates an expression which deserializes a field from a schema field 'schema_field'.
    fn deserialize_field(&self, field: &FieldDefinition, schema_field: &str) -> String {
        format!(
            "{}.get::<{}>({})",
            schema_field,
            self.field_type_name(&field.field_type),
            field.field_id,
        )
    }

    fn deserialize_update_field(&self, field: &FieldDefinition, update: &str) -> String {
        format!(
            "{}.get_update::<{}>({})",
            update,
            self.field_type_name(&field.field_type),
            field.field_id,
        )
    }

    fn serialize_update_field(&self, field: &FieldDefinition, update: &str) -> String {
        format!(
            "{}.add_update::<{}>({}, &self.{})",
            update,
            self.field_type_name(&field.field_type),
            field.field_id,
            field.name,
        )
    }
}

#[derive(Debug)]
struct GeneratedCode {
    root_package: Option<Package>,
    packages: BTreeSet<String>,
    enums: BTreeMap<String, EnumDefinition>,
    types: BTreeMap<String, TypeDefinition>,
    components: BTreeMap<String, ComponentDefinition>,
}

impl GeneratedCode {
    fn resolve_type_reference(&self, qualified_name: &str) -> &TypeDefinition {
        &self.types[qualified_name]
    }

    fn resolve_enum_reference(&self, qualified_name: &str) -> &EnumDefinition {
        &self.enums[qualified_name]
    }

    pub fn get_package(&self, qualified_name: &str) -> &Package {
        let mut package = self.root_package.as_ref().unwrap();
        let path: Vec<&str> = qualified_name.split('.').collect();
        let mut current_part = 0;

        while current_part < path.len() {
            if let Some(new_package) = package.get_subpackage(&path[current_part]) {
                current_part += 1;
                package = new_package;
            } else {
                break;
            }
        }

        package
    }
}

// This function ensures that given a path ["example", "foo"] and the root package, it will create
// 2 packages with the following structure:
//   Package("root", [Package("example", [Package("foo", [])])])
fn get_or_create_packages<'a>(package: &'a mut Package, path: &[&str]) -> &'a mut Package {
    if path.is_empty() {
        return package;
    }
    // Given a package, and a path. If that package does not have any subpackages with the name of the "next"
    // package in the FQN, create it.
    let package_name = path[0];
    let mut package_path = package.path.clone();
    package_path.push(package_name.to_string());
    if !package.subpackages.contains_key(package_name) {
        package.subpackages.insert(
            package_name.to_string(),
            Package::new(
                Rc::clone(&package.generated_code),
                package_name,
                package_path,
            ),
        );
    }

    // Recurse into the package created above, and create more packages if needed.
    get_or_create_packages(
        package.subpackages.get_mut(package_name).unwrap(),
        &path[1..],
    )
}

fn generate_module(package: &Package) -> String {
    let submodules = if !package.subpackages.is_empty() {
        package
            .subpackages
            .iter()
            .map(|(_, pkg)| generate_module(&pkg))
            .fold("".to_string(), |submodule, next| submodule + "\n" + &next)
    } else {
        "".to_string()
    };
    // Passing `package` to format! causes the T4 template engine to generate output.
    let module_contents = format!("{}\n{}", package, submodules);
    // The only package with a depth of 0 is the root package.
    if package.depth() == 0 {
        let allow_warnings = vec![
            "#![allow(unused_imports)]",
            "#![allow(unreachable_code)]",
            "#![allow(unreachable_patterns)]",
            "#![allow(unused_variables)]",
            "#![allow(dead_code)]",
            "#![allow(non_camel_case_types)]",
            "#![allow(unused_mut)]",
        ]
        .join("\n");
        format!("{}\n\n{}", allow_warnings, module_contents)
    } else {
        format!("pub mod {} {{\n{}}}\n", package.name, module_contents)
    }
}

pub fn generate_code(bundle: SchemaBundle) -> String {
    // Set up the root package.
    let generated_code = Rc::new(RefCell::new(GeneratedCode {
        root_package: None,
        packages: BTreeSet::new(),
        enums: BTreeMap::new(),
        types: BTreeMap::new(),
        components: BTreeMap::new(),
    }));
    let mut root_package = Package::new(Rc::clone(&generated_code), "", vec![]);
    for file in bundle.schema_files {
        let package = get_or_create_packages(
            &mut root_package,
            file.package
                .name
                .split('.')
                .collect::<Vec<&str>>()
                .as_slice(),
        );
        for type_def in file.types {
            package.types.insert(type_def.qualified_name.clone());
            generated_code
                .borrow_mut()
                .types
                .insert(type_def.qualified_name.clone(), type_def);
        }
        for enum_def in file.enums {
            package.enums.insert(enum_def.qualified_name.clone());
            generated_code
                .borrow_mut()
                .enums
                .insert(enum_def.qualified_name.clone(), enum_def);
        }
        for component_def in file.components {
            package
                .components
                .insert(component_def.qualified_name.clone());
            generated_code
                .borrow_mut()
                .components
                .insert(component_def.qualified_name.clone(), component_def);
        }
    }
    generated_code.borrow_mut().root_package = Some(root_package);
    //println!("{:#?}", generated_code.borrow_mut().root_package);
    let generated_code_ref = generated_code.borrow();
    generate_module(&generated_code_ref.root_package.as_ref().unwrap())
}
