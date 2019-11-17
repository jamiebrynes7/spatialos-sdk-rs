use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use std::{collections::BTreeMap, convert::TryFrom};

use <#= vec!["super".to_string(); self.depth() + 1].join("::") #>::generated as generated;

/* Enums. */<# for enum_name in &self.enums {
let enum_def = self.get_enum_definition(enum_name);
let enum_rust_name = self.rust_name(&enum_def.qualified_name);
#>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum <#= enum_rust_name #> {
<# for enum_value in &enum_def.values { #>
    <#= enum_value.name #>,<# } #>
}

impl EnumField for <#= enum_rust_name #> {}

impl Default for <#= enum_rust_name #> {
    fn default() -> Self {
        <#= enum_rust_name #>::<#= &enum_def.values[0].name #>
    }
}

impl TryFrom<u32> for <#= enum_rust_name #> {
    type Error = UnknownDiscriminantError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            <# for enum_value in &enum_def.values { #>
            <#= enum_value.value #> => Ok(<#= enum_rust_name #>::<#= enum_value.name #>), <# } #>
            _ => Err(UnknownDiscriminantError)
        }
    }
}

impl Into<u32> for <#= enum_rust_name #> {
    fn into(self) -> u32 {
        match self {
            <# for enum_value in &enum_def.values { #>
            <#= enum_rust_name #>::<#= enum_value.name #> => <#= enum_value.value #>, <# } #>
        }
    }
}

impl_field_for_enum_field!(<#= enum_rust_name #>);
<# } #>
/* Types. */<# for type_name in &self.types { let type_def = self.get_type_definition(type_name); #>
#[derive(Debug, Clone)]
pub struct <#= self.rust_name(&type_def.qualified_name) #> {<#
    for field in &type_def.fields {
    #>
    pub <#= field.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl ObjectField for <#= self.rust_name(&type_def.qualified_name) #> {
    fn from_object(input: &SchemaObject) -> Self {
        Self {<#
            for field in &type_def.fields {
            #>
            <#= field.name #>: <#= self.deserialize_field(field, "input") #>,<# } #>
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {<#
        for field in &type_def.fields {
        #>
        <#= self.serialize_field(field, &format!("&input.{}", field.name), "output") #>;<# } #>
    }
}
<# } #>
/* Components. */ <# for component_name in &self.components {
    let component = self.get_component_definition(component_name);
    let component_fields = self.get_component_fields(&component); #>
#[derive(Debug, Clone)]
pub struct <#= self.rust_name(&component.qualified_name) #> {<#
    for field in &component_fields {
    #>
    pub <#= field.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl ObjectField for <#= self.rust_name(&component.qualified_name) #> {
    fn from_object(input: &SchemaObject) -> Self {
        Self {<#
            for field in &component_fields {#>
            <#= field.name #>: <#= self.deserialize_field(field, "input") #>,<# } #>
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {<#
        for field in &component_fields {
        #>
        <#= self.serialize_field(field, &format!("&input.{}", field.name), "output") #>;<# } #>
    }
}
impl ComponentData<<#= self.rust_name(&component.qualified_name) #>> for <#= self.rust_name(&component.qualified_name) #> {
    fn merge(&mut self, update: <#= self.rust_name(&component.qualified_name) #>Update) {<#
        for field in &component_fields {
        #>
        if let Some(value) = update.<#= field.name #> { self.<#= field.name #> = value; }<# } #>
    }
}

#[derive(Debug, Clone, Default)]
pub struct <#= self.rust_name(&component.qualified_name) #>Update {<#
    for field in &component_fields {
    #>
    pub <#= field.name #>: Option<<#= self.generate_field_type(field) #>>,<# } #>
}
impl ObjectField for <#= self.rust_name(&component.qualified_name) #>Update {
    fn from_object(input: &SchemaObject) -> Self {
        Self {<#
            for field in &component_fields {#>
            <#= field.name #>: <#= self.deserialize_update_field(field, "input") #>,<# } #>
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {<#
        for field in &component_fields {
        #>
        if let Some(value) = &input.<#= field.name #> {
            <#= self.serialize_field(field, "value", "output") #>;
        }<# } #>
    }
}
impl ComponentUpdate<<#= self.rust_name(&component.qualified_name) #>> for <#= self.rust_name(&component.qualified_name) #>Update {
    fn merge(&mut self, update: <#= self.rust_name(&component.qualified_name) #>Update) {<#
        for field in &self.get_component_fields(&component) {
        #>
        if update.<#= field.name #>.is_some() { self.<#= field.name #> = update.<#= field.name #>; }<# } #>
    }
}

#[derive(Debug, Clone)]
pub enum <#= self.rust_name(&component.qualified_name) #>CommandRequest {<#
    for command in &component.commands {
    #>
    <#= command.name.to_camel_case() #>(<#= self.rust_fqname(&command.request_type) #>),<# } #>
}

#[derive(Debug, Clone)]
pub enum <#= self.rust_name(&component.qualified_name) #>CommandResponse {<#
    for command in &component.commands {
    #>
    <#= command.name.to_camel_case() #>(<#= self.rust_fqname(&command.response_type) #>),<# } #>
}

impl Component for <#= self.rust_name(&component.qualified_name) #> {
    type Update = <#= self.rust_fqname(&component.qualified_name) #>Update;
    type CommandRequest = <#= self.rust_fqname(&component.qualified_name) #>CommandRequest;
    type CommandResponse = <#= self.rust_fqname(&component.qualified_name) #>CommandResponse;

    const ID: ComponentId = <#= component.component_id #>;

    fn from_data(data: &SchemaComponentData) -> Result<<#= self.rust_fqname(&component.qualified_name) #>, String> {
        <<#= self.rust_fqname(&component.qualified_name) #> as ObjectField>::from_object(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<<#= self.rust_fqname(&component.qualified_name) #>Update, String> {
        <<#= self.rust_fqname(&component.qualified_name) #>Update as ObjectField>::from_object(&update.fields())
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<<#= self.rust_fqname(&component.qualified_name) #>CommandRequest, String> {
        match command_index {<#
            for command in &component.commands {
            #>
            <#= command.command_index #> => {
                let result = <<#= self.rust_fqname(&command.request_type) #> as ObjectField>::from_object(&request.object());
                result.and_then(|deserialized| Ok(<#= self.rust_name(&component.qualified_name) #>CommandRequest::<#= command.name.to_camel_case() #>(deserialized)))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component <#= self.rust_name(&component.qualified_name) #>.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<<#= self.rust_fqname(&component.qualified_name) #>CommandResponse, String> {
        match command_index {<#
            for command in &component.commands {
            #>
            <#= command.command_index #> => {
                let result = <<#= self.rust_fqname(&command.response_type) #> as ObjectField>::from_object(&response.object());
                result.and_then(|deserialized| Ok(<#= self.rust_name(&component.qualified_name) #>CommandResponse::<#= command.name.to_camel_case() #>(deserialized)))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component <#= self.rust_name(&component.qualified_name) #>.", command_index))
        }
    }

    fn to_data(data: &<#= self.rust_fqname(&component.qualified_name) #>) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <<#= self.rust_fqname(&component.qualified_name) #> as ObjectField>::into_object(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &<#= self.rust_fqname(&component.qualified_name) #>Update) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <<#= self.rust_fqname(&component.qualified_name) #>Update as ObjectField>::into_object(update, &mut serialized_update.fields_mut())?;
        Ok(serialized_update)
    }

    fn to_request(request: &<#= self.rust_fqname(&component.qualified_name) #>CommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {<#
            for command in &component.commands {
            #>
            <#= self.rust_name(&component.qualified_name) #>CommandRequest::<#= command.name.to_camel_case() #>(ref data) => {
                <<#= self.rust_fqname(&command.request_type) #> as ObjectField>::into_object(data, &mut serialized_request.object_mut())?;
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &<#= self.rust_fqname(&component.qualified_name) #>CommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {<#
            for command in &component.commands {
            #>
            <#= self.rust_name(&component.qualified_name) #>CommandResponse::<#= command.name.to_camel_case() #>(ref data) => {
                <<#= self.rust_fqname(&command.response_type) #> as ObjectField>::into_object(data, &mut serialized_response.object_mut())?;
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &<#= self.rust_fqname(&component.qualified_name) #>CommandRequest) -> u32 {
        match request {<#
            for command in &component.commands {
            #>
            <#= self.rust_name(&component.qualified_name) #>CommandRequest::<#= command.name.to_camel_case() #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &<#= self.rust_fqname(&component.qualified_name) #>CommandResponse) -> u32 {
        match response {<#
            for command in &component.commands {
            #>
            <#= self.rust_name(&component.qualified_name) #>CommandResponse::<#= command.name.to_camel_case() #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<<#= self.rust_name(&component.qualified_name) #>>());
<# } #>
