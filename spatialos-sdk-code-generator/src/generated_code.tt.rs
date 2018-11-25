use spatialos_sdk::worker::internal::schema::{self, SchemaField};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};

// Enums.
<# for (_, enum_def) in &self.enums { #>
#[derive(Debug)]
pub enum <#= enum_def.identifier.name #> {
}
<# } #>

// Types.
<# for (_, type_def) in &self.types { #>
#[derive(Debug)]
pub struct <#= type_def.identifier.name #> {
    <#
    for field in &type_def.field_definitions {
    #><#= field.identifier.name #>: <#= self.generate_field_type(field) #>,
    <# } #>
}
<# } #>

// Components.
<# for (_, component) in &self.components { #>
#[derive(Debug)]
pub struct <#= component.identifier.name #>;

#[derive(Debug)]
pub struct <#= component.identifier.name #>Update {
    <# 
    let data_type_ref = component.data_definition.type_reference.as_ref().unwrap();
    let data_type = self.resolve_type_reference(data_type_ref);
    for field in &data_type.field_definitions {
    #><#= field.identifier.name #>: Option<<#= self.generate_field_type(field) #>>,
    <# } #>
}

impl ComponentMetaclass for <#= component.identifier.name #> {
    type Data = <#= generate_identifier(&self.resolve_type_reference(&component.data_definition.type_reference.as_ref().unwrap()).identifier) #>;
    type Update = <#= generate_identifier(&component.identifier) #>Update;
    type CommandRequest = <#= generate_identifier(&component.identifier) #>CommandRequest;
    type CommandResponse = <#= generate_identifier(&component.identifier) #>CommandResponse;
}

#[derive(Debug)]
impl ComponentData<<#= component.identifier.name #>> for <#= generate_identifier(&self.resolve_type_reference(&component.data_definition.type_reference.as_ref().unwrap()).identifier) #> {
    fn merge(&mut self, update: <#= generate_identifier(&component.identifier) #>Update) {
        <# 
        let data_type_ref = component.data_definition.type_reference.as_ref().unwrap();
        let data_type = self.resolve_type_reference(data_type_ref);
        for field in &data_type.field_definitions {
        #><#= field.identifier.name #>: Option<<#= self.generate_field_type(field) #>>,
        <# } #>
    }
}
<# } #>
