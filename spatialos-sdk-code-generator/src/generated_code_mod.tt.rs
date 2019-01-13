use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use <#= vec!["super".to_string(); self.depth + 1].join("::") #>::generated as generated;

/* Enums. */<# for enum_name in &self.enums { let enum_def = self.get_enum_definition(enum_name); #>
#[derive(Debug)]
pub enum <#= enum_def.identifier.name #> {
}
<# } #>
/* Types. */<# for type_name in &self.types { let type_def = self.get_type_definition(type_name); #>
#[derive(Debug)]
pub struct <#= type_def.identifier.name #> {<#
    for field in &type_def.field_definitions {
    #>
    <#= field.identifier.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeSerializer<<#= type_def.identifier.name #>> for <#= type_def.identifier.name #> {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &type_def.field_definitions {
        #>
        <#= self.serialize_field(field, &format!("&input.{}", field.identifier.name), "output") #>;<# } #>
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {<#
            for field in &type_def.field_definitions {
                let field_expr = format!("input.field::<{}>({})", get_field_schema_type(field), field.field_id);
            #>
            <#= field.identifier.name #>: <#= self.deserialize_field(field, &field_expr) #>,<# } #>
        })
    }
}
<# } #>
/* Components. */ <# for component_name in &self.components {
    let component = self.get_component_definition(component_name);
    let component_fields = self.get_component_fields(&component); #>
#[derive(Debug)]
pub struct <#= component.identifier.name #> {<# 
    for field in &component_fields {
    #>
    <#= field.identifier.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeSerializer<<#= component.identifier.name #>> for <#= component.identifier.name #> {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &component_fields {
        #>
        <#= self.serialize_field(field, &format!("&input.{}", field.identifier.name), "output") #>;<# } #>
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {<#
            for field in &component_fields {
                let field_expr = format!("input.field::<{}>({})", get_field_schema_type(field), field.field_id);
            #>
            <#= field.identifier.name #>: <#= self.deserialize_field(field, &field_expr) #>,<# } #>
        })
    }
}
impl ComponentData<<#= component.identifier.name #>> for <#= self.generate_identifier(&component.identifier) #> {
    fn merge(&mut self, update: <#= self.generate_identifier(&component.identifier) #>Update) {<# 
        for field in &component_fields {
        #>
        if let Some(value) = update.<#= field.identifier.name #> { self.<#= field.identifier.name #> = Some(value); }<# } #>
    }
}

#[derive(Debug)]
pub struct <#= component.identifier.name #>Update {<# 
    for field in &component_fields {
    #>
    <#= field.identifier.name #>: Option<<#= self.generate_field_type(field) #>>,<# } #>
}
impl TypeSerializer<<#= component.identifier.name #>Update> for <#= component.identifier.name #>Update {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &component_fields {
        #>
        if let Some(ref value) = input.<#= field.identifier.name #> {
            <#= self.serialize_field(field, "value", "output") #>;
        }<# } #>
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
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
            output.<#= field.identifier.name #> = <#= self.deserialize_field(field, "field") #>;
        }<# } #>
        Ok(output)
    }
}
impl ComponentUpdate<<#= component.identifier.name #>> for <#= self.generate_identifier(&component.identifier) #>Update {
    fn merge(&mut self, update: <#= self.generate_identifier(&component.identifier) #>Update) {<#
        for field in &self.get_component_fields(&component) {
        #>
        if update.<#= field.identifier.name #>.is_some() { self.<#= field.identifier.name #> = update.<#= field.identifier.name #>; }<# } #>
    }
}

impl ComponentMetaclass for <#= component.identifier.name #> {
    type Data = <#= self.generate_identifier(&component.identifier) #>;
    type Update = <#= self.generate_identifier(&component.identifier) #>Update;
    type CommandRequest = <#= self.generate_identifier(&component.identifier) #>CommandRequest;
    type CommandResponse = <#= self.generate_identifier(&component.identifier) #>CommandResponse;

    fn component_id() -> ComponentId {
        <#= component.component_id #>
    }
}

#[derive(Debug)]
pub enum <#= component.identifier.name #>CommandRequest {<#
    for command in &component.command_definitions {
    #>
    <#= command.identifier.name #>(<#= self.generate_value_type_reference(&command.request_type) #>),<# } #>
}

#[derive(Debug)]
pub enum <#= component.identifier.name #>CommandResponse {<#
    for command in &component.command_definitions {
    #>
    <#= command.identifier.name #>(<#= self.generate_value_type_reference(&command.response_type) #>),<# } #>
}

impl ComponentVtable<<#= component.identifier.name #>> for <#= component.identifier.name #> {
    fn serialize_data(data: &<#= self.generate_identifier(&component.identifier) #>) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<<#= self.generate_identifier(&component.identifier) #>>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<<#= self.generate_identifier(&component.identifier) #>, String> {
        TypeSerializer::<<#= self.generate_identifier(&component.identifier) #>>::deserialize(&data.fields())
    }

    fn serialize_update(update: &<#= self.generate_identifier(&component.identifier) #>Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<<#= self.generate_identifier(&component.identifier) #>Update>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<<#= self.generate_identifier(&component.identifier) #>Update, String> {
        TypeSerializer::<<#= self.generate_identifier(&component.identifier) #>Update>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &<#= self.generate_identifier(&component.identifier) #>CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {<#
            for command in &component.command_definitions {
            #>
            <#= component.identifier.name #>CommandRequest::<#= command.identifier.name #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {<#
            for command in &component.command_definitions {
            #>
            <#= component.identifier.name #>CommandRequest::<#= command.identifier.name #>(ref data) => {
                TypeSerializer::<<#= self.generate_value_type_reference(&command.request_type) #>>::serialize(data, serialized_request.object_mut());
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<<#= self.generate_identifier(&component.identifier) #>CommandRequest, String> {
        match request.command_index() {<#
            for command in &component.command_definitions {
            #>
            <#= command.command_index #> => {
                Some(<#= component.identifier.name #>CommandRequest::<#= command.identifier.name #>(
                    TypeSerializer::<<#= self.generate_value_type_reference(&command.request_type) #>>::deserialize(request.object())
                ))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component <#= component.identifier.name #>.", request.command_index())
        }
    }

    fn serialize_command_response(response: &<#= self.generate_identifier(&component.identifier) #>CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {<#
            for command in &component.command_definitions {
            #>
            <#= component.identifier.name #>CommandResponse::<#= command.identifier.name #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {<#
            for command in &component.command_definitions {
            #>
            <#= component.identifier.name #>CommandResponse::<#= command.identifier.name #>(ref data) => {
                TypeSerializer::<<#= self.generate_value_type_reference(&command.response_type) #>>::serialize(data, serialized_response.object_mut())
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<<#= self.generate_identifier(&component.identifier) #>CommandResponse, String> {
        match response.command_index() {<#
            for command in &component.command_definitions {
            #>
            <#= command.command_index #> => {
                Some(<#= component.identifier.name #>CommandResponse::<#= command.identifier.name #>(
                    TypeSerializer::<<#= self.generate_value_type_reference(&command.response_type) #>>::deserialize(response.object());
                ))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component <#= component.identifier.name #>.", request.command_index())
        }
    }
}
<# } #>
