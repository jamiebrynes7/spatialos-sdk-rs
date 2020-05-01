use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use spatialos_sdk::worker::commands::*;
use std::{collections::BTreeMap, convert::TryFrom};

use <#= vec!["super".to_string(); self.depth() + 1].join("::") #>::generated as generated;

/* Enums. */<# for enum_name in &self.enums {
let enum_def = self.get_enum_definition(enum_name);
let enum_rust_name = self.rust_name(&enum_def.qualified_name);
#>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            <# for enum_value in &enum_def.values { #>
            <#= enum_value.value #> => Ok(<#= enum_rust_name #>::<#= enum_value.name #>), <# } #>
            _ => Err(UnknownDiscriminantError {
                type_name: std::any::type_name::<Self>(),
                value,
            }),
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct <#= self.rust_name(&type_def.qualified_name) #> {<#
    for field in &type_def.fields {
    #>
    pub <#= field.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl ObjectField for <#= self.rust_name(&type_def.qualified_name) #> {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {<#
            for field in &type_def.fields {
            #>
            <#= field.name #>: <#= self.deserialize_field(field, "input") #>,<# } #>
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {<#
        for field in &type_def.fields {
        #>
        <#= self.serialize_field(field, "output") #>;<# } #>
    }
}
<# } #>
/* Components. */ <# for component_name in &self.components {
    let component = self.get_component_definition(component_name);
    let component_fields = self.get_component_fields(&component);
    let component_name = self.rust_name(&component.qualified_name);
    let update_name = format!("{}Update", component_name); #>
#[derive(Debug, Clone)]
pub struct <#= component_name #> {<#
    for field in &component_fields {
    #>
    pub <#= field.name #>: <#= self.generate_field_type(field) #>,<# } #>
}

impl ObjectField for <#= component_name #> {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {<#
            for field in &component_fields {#>
            <#= field.name #>: <#= self.deserialize_field(field, "input") #>,<# } #>
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {<#
        for field in &component_fields {
        #>
        <#= self.serialize_field(field, "output") #>;<# } #>
    }
}

#[derive(Debug, Clone, Default)]
pub struct <#= update_name #> {<#
    for field in &component_fields {
    #>
    pub <#= field.name #>: Option<<#= self.generate_field_type(field) #>>,<# } #>
}

impl Update for <#= update_name #> {
    type Component = <#= component_name #>;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {<#
            for field in &component_fields {#>
            <#= field.name #>: <#= self.deserialize_update_field(field, "update") #>,<# } #>
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {<#
        for field in &component_fields {#>
        <#= self.serialize_update_field(field, "update") #>;<# } #>
    }

    fn merge(&mut self, update: Self) {<#
        for field in &component_fields {
        #>
        if update.<#= field.name #>.is_some() { self.<#= field.name #> = update.<#= field.name #>; }<# } #>
    }
}

<# if (!&component.commands.is_empty()) { #>

#[derive(Debug, Clone)]
pub enum <#= component_name #>CommandRequest {<#
    for command in &component.commands {
    #>
    <#= command.name.to_camel_case() #>(<#= self.rust_fqname(&command.request_type) #>),<# } #>
}

impl Request for <#= component_name #>CommandRequest {
    type Commands = <#= component_name #>;

    fn from_schema(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<Self> {
        match command_index {<#
            for command in &component.commands {
            #>
            <#= command.command_index #> => {
                <<#= self.rust_fqname(&command.request_type) #> as ObjectField>::from_object(&request.object())
                    .map(Self::<#= command.name.to_camel_case() #>)
            },<# } #>
            _ => Err(Error::unknown_command::<Self>(command_index))
        }
    }

    fn into_schema(&self, request: &mut SchemaCommandRequest) -> CommandIndex {
        match self {<#
            for command in &component.commands {
            #>
            Self::<#= command.name.to_camel_case() #>(ref inner) => {
                <<#= self.rust_fqname(&command.request_type) #> as ObjectField>::into_object(inner, &mut request.object_mut());
                <#= command.command_index #>
            }, <# } #>
        }
    }
}

#[derive(Debug, Clone)]
pub enum <#= component_name #>CommandResponse {<#
    for command in &component.commands {
    #>
    <#= command.name.to_camel_case() #>(<#= self.rust_fqname(&command.response_type) #>),<# } #>
}

impl Response for <#= component_name #>CommandResponse {
    type Commands = <#= component_name #>;

    fn from_schema(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<Self> {
        match command_index { <#
            for command in &component.commands {
            #>
            <#= command.command_index #> => {
                <<#= self.rust_fqname(&command.response_type) #> as ObjectField>::from_object(&response.object())
                    .map(<#= component_name #>CommandResponse::<#= command.name.to_camel_case() #>)
            }, <# } #>
            _ => Err(Error::unknown_command::<Self>(command_index))
        }
    }

    fn into_schema(&self, response: &mut SchemaCommandResponse) -> CommandIndex {
        match self {<#
            for command in &component.commands {
            #>
            Self::<#= command.name.to_camel_case() #>(ref inner) => {
                <<#= self.rust_fqname(&command.response_type) #> as ObjectField>::into_object(inner, &mut response.object_mut());
                <#= command.command_index #>
            },<# } #>
        }
    }
}

impl Commands for <#= component_name #> {
    type Component = <#= component_name #>;
    type Request = <#= component_name #>CommandRequest;
    type Response = <#= component_name #>CommandResponse;
}

<# } #>

impl Component for <#= component_name #> {
    type Update = <#= update_name #>;

    const ID: ComponentId = <#= component.component_id #>;

    fn merge_update(&mut self, update: Self::Update) {<#
        for field in &component_fields {
        #>
        if let Some(value) = update.<#= field.name #> { self.<#= field.name #> = value; }<# } #>
    }
}
<# } #>
