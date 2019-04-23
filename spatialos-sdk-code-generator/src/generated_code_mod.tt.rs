use spatialos_sdk::worker::internal::schema::*;
use spatialos_sdk::worker::component::*;
use std::collections::BTreeMap;

use <#= vec!["super".to_string(); self.depth() + 1].join("::") #>::generated as generated;

/* Enums. */<# for enum_name in &self.enums {
let enum_def = self.get_enum_definition(enum_name);
let enum_rust_name = self.rust_name(&enum_def.identifier);
#>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum <#= enum_rust_name #> {
<# for enum_value in &enum_def.value_definitions { #>
    <#= enum_value.identifier.name #>,<# } #>
}

impl From<u32> for <#= enum_rust_name #> {
    fn from(value: u32) -> Self {
        match value {
<# for enum_value in &enum_def.value_definitions { #>
            <#= enum_value.value #> => <#= enum_rust_name #>::<#= enum_value.identifier.name #>, <# } #>
            _ => panic!(format!("Could not convert {} to enum <#= enum_rust_name #>.", value))
        }
    }
}

impl <#= enum_rust_name #> {
    pub(crate) fn as_u32(&self) -> u32 {
        match &self {
            <# for enum_value in &enum_def.value_definitions { #>
            <#= enum_rust_name #>::<#= enum_value.identifier.name #> => <#= enum_value.value #>, <# } #>
        }
    }
}
<# } #>
/* Types. */<# for type_name in &self.types { let type_def = self.get_type_definition(type_name); #>
#[derive(Debug, Clone)]
pub struct <#= self.rust_name(&type_def.identifier) #> {<#
    for field in &type_def.field_definitions {
    #>
    pub <#= field.identifier.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeConversion for <#= self.rust_name(&type_def.identifier) #> {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {<#
            for field in &type_def.field_definitions {
                let field_expr = format!("input.field::<{}>({})", get_field_schema_type(field), field.field_id);
            #>
            <#= field.identifier.name #>: <#= self.deserialize_field(field, &field_expr) #>,<# } #>
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {<#
        for field in &type_def.field_definitions {
            let borrow = if self.field_needs_borrow(field) {
                "&"
            } else {
                ""
            };
        #>
        <#= self.serialize_field(field, &format!("{}input.{}", borrow, field.identifier.name), "output") #>;<# } #>
        Ok(())
    }
}
<# } #>
/* Components. */ <# for component_name in &self.components {
    let component = self.get_component_definition(component_name);
    let component_fields = self.get_component_fields(&component); #>
#[derive(Debug, Clone)]
pub struct <#= self.rust_name(&component.identifier) #> {<#
    for field in &component_fields {
    #>
    pub <#= field.identifier.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeConversion for <#= self.rust_name(&component.identifier) #> {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {<#
            for field in &component_fields {
                let field_expr = format!("input.field::<{}>({})", get_field_schema_type(field), field.field_id);
            #>
            <#= field.identifier.name #>: <#= self.deserialize_field(field, &field_expr) #>,<# } #>
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {<#
        for field in &component_fields {
            let borrow = if self.field_needs_borrow(field) {
                "&"
            } else {
                ""
            };
        #>
        <#= self.serialize_field(field, &format!("{}input.{}", borrow, field.identifier.name), "output") #>;<# } #>
        Ok(())
    }
}
impl ComponentData<<#= self.rust_name(&component.identifier) #>> for <#= self.rust_name(&component.identifier) #> {
    fn merge(&mut self, update: <#= self.rust_name(&component.identifier) #>Update) {<#
        for field in &component_fields {
        #>
        if let Some(value) = update.<#= field.identifier.name #> { self.<#= field.identifier.name #> = value; }<# } #>
    }
}

#[derive(Debug, Clone, Default)]
pub struct <#= self.rust_name(&component.identifier) #>Update {<#
    for field in &component_fields {
    #>
    pub <#= field.identifier.name #>: Option<<#= self.generate_field_type(field) #>>,<# } #>
}
impl TypeConversion for <#= self.rust_name(&component.identifier) #>Update {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        let mut output = Self {<#
            for field in &component_fields {
            #>
            <#= field.identifier.name #>: None,<# } #>
        };<#
        for field in &component_fields {
        #>
        let _field_<#= field.identifier.name #> = input.field::<<#= get_field_schema_type(field) #>>(<#= field.field_id #>);
        if _field_<#= field.identifier.name #>.count() > 0 {
            let field = &_field_<#= field.identifier.name #>;
            output.<#= field.identifier.name #> = Some(<#= self.deserialize_field(field, "field") #>);
        }<# } #>
        Ok(output)
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {<#
        for field in &component_fields {
            let ref_decorator = if self.field_needs_borrow(field) {
                "ref "
            } else {
                ""
            };
        #>
        if let Some(<#= ref_decorator #>value) = input.<#= field.identifier.name #> {
            <#= self.serialize_field(field, "value", "output") #>;
        }<# } #>
        Ok(())
    }
}
impl ComponentUpdate<<#= self.rust_name(&component.identifier) #>> for <#= self.rust_name(&component.identifier) #>Update {
    fn merge(&mut self, update: <#= self.rust_name(&component.identifier) #>Update) {<#
        for field in &self.get_component_fields(&component) {
        #>
        if update.<#= field.identifier.name #>.is_some() { self.<#= field.identifier.name #> = update.<#= field.identifier.name #>; }<# } #>
    }
}

#[derive(Debug, Clone)]
pub enum <#= self.rust_name(&component.identifier) #>CommandRequest {<#
    for command in &component.command_definitions {
    #>
    <#= command.identifier.name.to_camel_case() #>(<#= self.generate_value_type_reference(&command.request_type) #>),<# } #>
}

#[derive(Debug, Clone)]
pub enum <#= self.rust_name(&component.identifier) #>CommandResponse {<#
    for command in &component.command_definitions {
    #>
    <#= command.identifier.name.to_camel_case() #>(<#= self.generate_value_type_reference(&command.response_type) #>),<# } #>
}

impl Component for <#= self.rust_name(&component.identifier) #> {
    type Update = <#= self.rust_fqname(&component.identifier) #>Update;
    type CommandRequest = <#= self.rust_fqname(&component.identifier) #>CommandRequest;
    type CommandResponse = <#= self.rust_fqname(&component.identifier) #>CommandResponse;

    const ID: ComponentId = <#= component.component_id #>;

    fn from_data(data: &SchemaComponentData) -> Result<<#= self.rust_fqname(&component.identifier) #>, String> {
        <<#= self.rust_fqname(&component.identifier) #> as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<<#= self.rust_fqname(&component.identifier) #>Update, String> {
        <<#= self.rust_fqname(&component.identifier) #>Update as TypeConversion>::from_type(&update.fields())
    }

    fn from_request(request: &SchemaCommandRequest) -> Result<<#= self.rust_fqname(&component.identifier) #>CommandRequest, String> {
        match request.command_index() {<#
            for command in &component.command_definitions {
            #>
            <#= command.command_index #> => {
                let result = <<#= self.generate_value_type_reference(&command.request_type) #> as TypeConversion>::from_type(&request.object());
                result.and_then(|deserialized| Ok(<#= self.rust_name(&component.identifier) #>CommandRequest::<#= command.identifier.name.to_camel_case() #>(deserialized)))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component <#= self.rust_name(&component.identifier) #>.", request.command_index()))
        }
    }

    fn from_response(response: &SchemaCommandResponse) -> Result<<#= self.rust_fqname(&component.identifier) #>CommandResponse, String> {
        match response.command_index() {<#
            for command in &component.command_definitions {
            #>
            <#= command.command_index #> => {
                let result = <<#= self.generate_value_type_reference(&command.response_type) #> as TypeConversion>::from_type(&response.object());
                result.and_then(|deserialized| Ok(<#= self.rust_name(&component.identifier) #>CommandResponse::<#= command.identifier.name.to_camel_case() #>(deserialized)))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component <#= self.rust_name(&component.identifier) #>.", response.command_index()))
        }
    }

    fn to_data(data: &<#= self.rust_fqname(&component.identifier) #>) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new(Self::ID);
        <<#= self.rust_fqname(&component.identifier) #> as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &<#= self.rust_fqname(&component.identifier) #>Update) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new(Self::ID);
        <<#= self.rust_fqname(&component.identifier) #>Update as TypeConversion>::to_type(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &<#= self.rust_fqname(&component.identifier) #>CommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new(Self::ID, Self::get_request_command_index(request));
        match request {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandRequest::<#= command.identifier.name.to_camel_case() #>(ref data) => {
                <<#= self.generate_value_type_reference(&command.request_type) #> as TypeConversion>::to_type(data, &mut serialized_request.object_mut())?;
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &<#= self.rust_fqname(&component.identifier) #>CommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new(Self::ID, Self::get_response_command_index(response));
        match response {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandResponse::<#= command.identifier.name.to_camel_case() #>(ref data) => {
                <<#= self.generate_value_type_reference(&command.response_type) #> as TypeConversion>::to_type(data, &mut serialized_response.object_mut())?;
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &<#= self.rust_fqname(&component.identifier) #>CommandRequest) -> u32 {
        match request {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandRequest::<#= command.identifier.name.to_camel_case() #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &<#= self.rust_fqname(&component.identifier) #>CommandResponse) -> u32 {
        match response {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandResponse::<#= command.identifier.name.to_camel_case() #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<<#= self.rust_name(&component.identifier) #>>());
<# } #>
