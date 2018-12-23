use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use <#= self.root_module() #> as generated_code;

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
    let data_type_ref = component.data_definition.type_reference.as_ref().unwrap();
    let data_type = self.resolve_type_reference(data_type_ref); #>
#[derive(Debug)]
pub struct <#= component.identifier.name #> {<# 
    for field in &data_type.field_definitions {
    #>
    <#= field.identifier.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeSerializer<<#= component.identifier.name #>> for <#= component.identifier.name #> {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &data_type.field_definitions {
        #>
        <#= self.serialize_field(field, &format!("&input.{}", field.identifier.name), "output") #>;<# } #>
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {<#
            for field in &data_type.field_definitions {
                let field_expr = format!("input.field::<{}>({})", get_field_schema_type(field), field.field_id);
            #>
            <#= field.identifier.name #>: <#= self.deserialize_field(field, &field_expr) #>,<# } #>
        })
    }
}
impl ComponentData<<#= component.identifier.name #>> for <#= self.generate_identifier(&component.identifier) #> {
    fn merge(&mut self, update: <#= self.generate_identifier(&component.identifier) #>Update) {<# 
        for field in &self.get_component_fields(&component) {
        #>
        if let Some(value) = update.<#= field.identifier.name #> { self.<#= field.identifier.name #> = Some(value); }<# } #>
    }
}

#[derive(Debug)]
pub struct <#= component.identifier.name #>Update {<# 
    for field in &data_type.field_definitions {
    #>
    <#= field.identifier.name #>: Option<<#= self.generate_field_type(field) #>>,<# } #>
}
impl TypeSerializer<<#= component.identifier.name #>Update> for <#= component.identifier.name #>Update {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {<#
        for field in &data_type.field_definitions {
        #>
        if let Some(ref value) = input.<#= field.identifier.name #> {
            <#= self.serialize_field(field, "value", "output") #>;
        }<# } #>
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        let mut output = Self {<#
            for field in &data_type.field_definitions {
            #>
            <#= field.identifier.name #>: None,<# } #>
        };<#
        for field in &data_type.field_definitions {
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
        if let Some(value) = update.<#= field.identifier.name #> { self.<#= field.identifier.name #> = value; }<# } #>
    }
}

impl ComponentMetaclass for <#= component.identifier.name #> {
    type Data = <#= self.generate_identifier(&component.identifier) #>;
    type Update = <#= self.generate_identifier(&component.identifier) #>Update;
    type CommandRequest = <#= self.generate_identifier(&component.identifier) #>CommandRequest;
    type CommandResponse = <#= self.generate_identifier(&component.identifier) #>CommandResponse;
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
    fn serialize_data(data: &Self::Data) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<Self::Data>::serialize(data, serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<Self::Data, String> {
        TypeSerializer::<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<Self::Update>::serialize(update, serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String> {
        TypeSerializer::<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
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

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        match request.command_index() {<#
            for command in &component.command_definitions {
            #>
            <#= command.command_index #> => {
                Some(<#= component.identifier.name #>CommandRequest::<#= command.identifier.name #>(
                    TypeSerializer::<<#= self.generate_value_type_reference(&command.request_type) #>>::deserialize(request.object());
                ))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component <#= component.identifier.name #>.", request.command_index())
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
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
                TypeSerializer::<<#= self.generate_value_type_reference(&command.response_type) #>>::serialize(data, serialized_response.object_mut());
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
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
