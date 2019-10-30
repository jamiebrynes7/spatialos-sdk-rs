use spatialos_sdk::worker::internal::schema::*;
use spatialos_sdk::worker::component::*;
use std::collections::BTreeMap;

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

impl From<u32> for <#= enum_rust_name #> {
    fn from(value: u32) -> Self {
        match value {
<# for enum_value in &enum_def.values { #>
            <#= enum_value.value #> => <#= enum_rust_name #>::<#= enum_value.name #>, <# } #>
            _ => panic!(format!("Could not convert {} to enum <#= enum_rust_name #>.", value))
        }
    }
}

impl <#= enum_rust_name #> {
    pub(crate) fn as_u32(self) -> u32 {
        match self {
            <# for enum_value in &enum_def.values { #>
            <#= enum_rust_name #>::<#= enum_value.name #> => <#= enum_value.value #>, <# } #>
        }
    }
}
<# } #>
/* Types. */<# for type_name in &self.types { let type_def = self.get_type_definition(type_name); #>
#[derive(Debug, Clone)]
pub struct <#= self.rust_name(&type_def.qualified_name) #> {<#
    for field in &type_def.fields {
    #>
    pub <#= field.name #>: <#= self.generate_field_type(field) #>,<# } #>
}
impl TypeConversion for <#= self.rust_name(&type_def.qualified_name) #> {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {<#
            for field in &type_def.fields {
                let field_expr = format!("input.field::<{}>({})", get_field_schema_type(field), field.field_id);
            #>
            <#= field.name #>: <#= self.deserialize_field(field, &field_expr) #>,<# } #>
        })
    }
    fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {<#
        for field in &type_def.fields {
            let borrow = if self.field_needs_borrow(field) {
                "&"
            } else {
                ""
            };
        #>
        <#= self.serialize_field(field, &format!("{}input.{}", borrow, field.name), "output") #>;<# } #>
        Ok(())
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
impl TypeConversion for <#= self.rust_name(&component.qualified_name) #> {
    fn from_type(input: &SchemaObject) -> Result<Self, String> {
        Ok(Self {<#
            for field in &component_fields {
                let field_expr = format!("input.field::<{}>({})", get_field_schema_type(field), field.field_id);
            #>
            <#= field.name #>: <#= self.deserialize_field(field, &field_expr) #>,<# } #>
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
        <#= self.serialize_field(field, &format!("{}input.{}", borrow, field.name), "output") #>;<# } #>
        Ok(())
    }
}
impl ComponentData<<#= self.rust_name(&component.qualified_name) #>> for <#= self.rust_name(&component.qualified_name) #> {
    fn merge(&mut self, update: <#= self.rust_name(&component.qualified_name) #>Update) {<#
        for field in &component_fields {
        #>
        if let Some(value) = update.<#= field.name #> { self.<#= field.name #> = value; }<# } #>
    }

    fn merge_ref(&mut self, update: &<#= self.rust_name(&component.qualified_name) #>Update) {
        let copy = <#= self.rust_name(&component.qualified_name) #>Update {<# for field in &component_fields { #>
            <#= field.name #>: update.<#= field.name #><#= if self.field_needs_clone(&field) { ".clone()" } else { "" }#>,<# } #> <# for event in &component.events { #>
            <#= event.name #> : Vec::new(),<# }#>
        };

        self.merge(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct <#= self.rust_name(&component.qualified_name) #>Update {<#
    for field in &component_fields {
    #>
    pub <#= field.name #>: Option<<#= self.generate_field_type(field) #>>,<# } #><#
    for event in &component.events { #>
    pub <#= event.name #>: Vec<<#= self.rust_fqname(&event.type_reference) #>>,<# } #>
}

impl <#= self.rust_name(&component.qualified_name) #>Update {
    pub fn from_schema(update: &SchemaComponentUpdate) -> Result<Self, String> {
        let fields = update.fields();
        let events = update.events();

        let mut output = Self {<# for field in &component_fields { #>
        <#= field.name #>: None,<# } #><# for event in &component.events { #>
        <#= event.name #>: Vec::new(),<# }#>
        }; <# for field in &component_fields { #>
        let _field_<#= field.name #> = fields.field::<<#= get_field_schema_type(field) #>>(<#= field.field_id #>);
        if _field_<#= field.name #>.count() > 0 {
            let field = &_field_<#= field.name #>;
            output.<#= field.name #> = Some(<#= self.deserialize_field(field, "field") #>);
        }<# } #> <# for event in &component.events { #>
        let _field_<#= event.name #> = events.field::<SchemaObject>(<#= event.event_index #>);
        for i in 0.._field_<#= event.name #>.count() {
            output.<#= event.name #>.push(<<#= self.rust_fqname(&event.type_reference) #> as TypeConversion>::from_type(&_field_<#= event.name #>.index(i))?);
        }<# } #>

        Ok(output)
    }

    pub fn to_schema(&self, update: &mut SchemaComponentUpdate) -> Result<(), String> {
        let mut fields = update.fields_mut();<#
        for field in &component_fields {
            let ref_decorator = if self.field_needs_borrow(field) {
                "ref "
            } else {
                ""
            };
        #>
        if let Some(<#= ref_decorator #>value) = self.<#= field.name #> {
            <#= self.serialize_field(field, "value", "fields") #>;
        }<# } #>

        let mut events = update.events_mut();<# for event in &component.events { #>
        for ev in &self.<#= event.name #> {
            let mut field = events.field::<SchemaObject>(<#= event.event_index #>);
            <<#= self.rust_fqname(&event.type_reference) #> as TypeConversion>::to_type(ev, &mut field.add())?;
        }<# } #>

        Ok(())
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
        <<#= self.rust_fqname(&component.qualified_name) #> as TypeConversion>::from_type(&data.fields())
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<Self::Update, String> {
        Self::Update::from_schema(update)
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<<#= self.rust_fqname(&component.qualified_name) #>CommandRequest, String> {
        match command_index {<#
            for command in &component.commands {
            #>
            <#= command.command_index #> => {
                let result = <<#= self.rust_fqname(&command.request_type) #> as TypeConversion>::from_type(&request.object());
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
                let result = <<#= self.rust_fqname(&command.response_type) #> as TypeConversion>::from_type(&response.object());
                result.and_then(|deserialized| Ok(<#= self.rust_name(&component.qualified_name) #>CommandResponse::<#= command.name.to_camel_case() #>(deserialized)))
            },<# } #>
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component <#= self.rust_name(&component.qualified_name) #>.", command_index))
        }
    }

    fn to_data(data: &<#= self.rust_fqname(&component.qualified_name) #>) -> Result<SchemaComponentData, String> {
        let mut serialized_data = SchemaComponentData::new();
        <<#= self.rust_fqname(&component.qualified_name) #> as TypeConversion>::to_type(data, &mut serialized_data.fields_mut())?;
        Ok(serialized_data)
    }

    fn to_update(update: &Self::Update) -> Result<SchemaComponentUpdate, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        update.to_schema(&mut serialized_update)?;
        Ok(serialized_update)
    }

    fn to_request(request: &<#= self.rust_fqname(&component.qualified_name) #>CommandRequest) -> Result<SchemaCommandRequest, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {<#
            for command in &component.commands {
            #>
            <#= self.rust_name(&component.qualified_name) #>CommandRequest::<#= command.name.to_camel_case() #>(ref data) => {
                <<#= self.rust_fqname(&command.request_type) #> as TypeConversion>::to_type(data, &mut serialized_request.object_mut())?;
            },<# } #>
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &<#= self.rust_fqname(&component.qualified_name) #>CommandResponse) -> Result<SchemaCommandResponse, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {<#
            for command in &component.commands {
            #>
            <#= self.rust_name(&component.qualified_name) #>CommandResponse::<#= command.name.to_camel_case() #>(ref data) => {
                <<#= self.rust_fqname(&command.response_type) #> as TypeConversion>::to_type(data, &mut serialized_response.object_mut())?;
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
