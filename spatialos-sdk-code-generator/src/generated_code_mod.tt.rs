use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject, SchemaObjectField};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use <#= vec!["super".to_string(); self.depth() + 1].join("::") #>::generated as generated;

/* Enums. */<# for enum_name in &self.enums { let enum_def = self.get_enum_definition(enum_name); #>
#[derive(Debug)]
pub enum <#= self.rust_name(&enum_def.identifier) #> {
}
<# } #>
/* Types. */<# for type_name in &self.types { let type_def = self.get_type_definition(type_name); #>
#[derive(Debug)]
pub struct <#= self.rust_name(&type_def.identifier) #> {<#
    for field in &type_def.field_definitions {
    #>
    <#= field.identifier.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeSerializer<<#= self.rust_name(&type_def.identifier) #>> for <#= self.rust_name(&type_def.identifier) #> {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &type_def.field_definitions {
        #>
        <#= self.serialize_field(field, &format!("input.{}", field.identifier.name), "output") #>;<# } #>
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
pub struct <#= self.rust_name(&component.identifier) #> {<# 
    for field in &component_fields {
    #>
    <#= field.identifier.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeSerializer<<#= self.rust_name(&component.identifier) #>> for <#= self.rust_name(&component.identifier) #> {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &component_fields {
        #>
        <#= self.serialize_field(field, &format!("input.{}", field.identifier.name), "output") #>;<# } #>
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
impl ComponentData<<#= self.rust_name(&component.identifier) #>> for <#= self.rust_name(&component.identifier) #> {
    fn merge(&mut self, update: <#= self.rust_name(&component.identifier) #>Update) {<# 
        for field in &component_fields {
        #>
        if let Some(value) = update.<#= field.identifier.name #> { self.<#= field.identifier.name #> = value; }<# } #>
    }
}

#[derive(Debug)]
pub struct <#= self.rust_name(&component.identifier) #>Update {<# 
    for field in &component_fields {
    #>
    <#= field.identifier.name #>: Option<<#= self.generate_field_type(field) #>>,<# } #>
}
impl TypeSerializer<<#= self.rust_name(&component.identifier) #>Update> for <#= self.rust_name(&component.identifier) #>Update {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &component_fields {
        #>
        if let Some(ref value) = input.<#= field.identifier.name #> {
            <#= self.serialize_field(field, "value.clone()", "output") #>;
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
            output.<#= field.identifier.name #> = Some(<#= self.deserialize_field(field, "field") #>);
        }<# } #>
        Ok(output)
    }
}
impl ComponentUpdate<<#= self.rust_name(&component.identifier) #>> for <#= self.rust_name(&component.identifier) #>Update {
    fn merge(&mut self, update: <#= self.rust_name(&component.identifier) #>Update) {<#
        for field in &self.get_component_fields(&component) {
        #>
        if update.<#= field.identifier.name #>.is_some() { self.<#= field.identifier.name #> = update.<#= field.identifier.name #>; }<# } #>
    }
}

impl ComponentMetaclass for <#= self.rust_name(&component.identifier) #> {
    type Data = <#= self.rust_fqname(&component.identifier) #>;
    type Update = <#= self.rust_fqname(&component.identifier) #>Update;
    type CommandRequest = <#= self.rust_fqname(&component.identifier) #>CommandRequest;
    type CommandResponse = <#= self.rust_fqname(&component.identifier) #>CommandResponse;

    fn component_id() -> ComponentId {
        <#= component.component_id #>
    }
}

#[derive(Debug)]
pub enum <#= self.rust_name(&component.identifier) #>CommandRequest {<#
    for command in &component.command_definitions {
    #>
    <#= command.identifier.name #>(<#= self.generate_value_type_reference(&command.request_type) #>),<# } #>
}

#[derive(Debug)]
pub enum <#= self.rust_name(&component.identifier) #>CommandResponse {<#
    for command in &component.command_definitions {
    #>
    <#= command.identifier.name #>(<#= self.generate_value_type_reference(&command.response_type) #>),<# } #>
}

impl ComponentVtable<<#= self.rust_name(&component.identifier) #>> for <#= self.rust_name(&component.identifier) #> {
    fn serialize_data(data: &<#= self.rust_fqname(&component.identifier) #>) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<<#= self.rust_fqname(&component.identifier) #>>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<<#= self.rust_fqname(&component.identifier) #>, String> {
        TypeSerializer::<<#= self.rust_fqname(&component.identifier) #>>::deserialize(&data.fields())
    }

    fn serialize_update(update: &<#= self.rust_fqname(&component.identifier) #>Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<<#= self.rust_fqname(&component.identifier) #>Update>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<<#= self.rust_fqname(&component.identifier) #>Update, String> {
        TypeSerializer::<<#= self.rust_fqname(&component.identifier) #>Update>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &<#= self.rust_fqname(&component.identifier) #>CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandRequest::<#= command.identifier.name #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandRequest::<#= command.identifier.name #>(ref data) => {
                TypeSerializer::<<#= self.generate_value_type_reference(&command.request_type) #>>::serialize(data, &mut serialized_request.object_mut());
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<<#= self.rust_fqname(&component.identifier) #>CommandRequest, String> {
        match request.command_index() {<#
            for command in &component.command_definitions {
            #>
            <#= command.command_index #> => {
                let result = TypeSerializer::<<#= self.generate_value_type_reference(&command.request_type) #>>::deserialize(&request.object());
                result.and_then(|deserialized| Ok(<#= self.rust_name(&component.identifier) #>CommandRequest::<#= command.identifier.name #>(deserialized)))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component <#= self.rust_name(&component.identifier) #>.", request.command_index()))
        }
    }

    fn serialize_command_response(response: &<#= self.rust_fqname(&component.identifier) #>CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandResponse::<#= command.identifier.name #>(_) => <#= command.command_index #>,<# } #>
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {<#
            for command in &component.command_definitions {
            #>
            <#= self.rust_name(&component.identifier) #>CommandResponse::<#= command.identifier.name #>(ref data) => {
                TypeSerializer::<<#= self.generate_value_type_reference(&command.response_type) #>>::serialize(data, &mut serialized_response.object_mut());
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<<#= self.rust_fqname(&component.identifier) #>CommandResponse, String> {
        match response.command_index() {<#
            for command in &component.command_definitions {
            #>
            <#= command.command_index #> => {
                let result = TypeSerializer::<<#= self.generate_value_type_reference(&command.response_type) #>>::deserialize(&response.object());
                result.and_then(|deserialized| Ok(<#= self.rust_name(&component.identifier) #>CommandResponse::<#= command.identifier.name #>(deserialized)))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component <#= self.rust_name(&component.identifier) #>.", response.command_index()))
        }
    }
}
<# } #>