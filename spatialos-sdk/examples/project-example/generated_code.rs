#[allow(unused_imports)]
#[allow(unreachable_code)]
#[allow(unreachable_patterns)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(unused_mut)]
mod generated {
    use spatialos_sdk::worker::component::*;
    use spatialos_sdk::worker::internal::schema::*;
    use std::collections::BTreeMap;

    use super::generated;

    /* Enums. */
    /* Types. */
    /* Components. */

    pub mod example {
        use spatialos_sdk::worker::component::*;
        use spatialos_sdk::worker::internal::schema::*;
        use std::collections::BTreeMap;

        use super::super::generated;

        /* Enums. */
        /* Types. */
        #[derive(Debug)]
        pub struct CommandData {
            value: i32,
        }
        impl TypeConversion for CommandData {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    value: input.field::<SchemaInt32>(1).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaInt32>(1).add(input.value);
                Ok(())
            }
        }

        /* Components. */

        #[derive(Debug)]
        pub struct Example {
            x: f32,
        }
        impl TypeConversion for Example {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    x: input.field::<SchemaFloat>(1).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaFloat>(1).add(input.x);
                Ok(())
            }
        }
        impl ComponentData<Example> for Example {
            fn merge(&mut self, update: ExampleUpdate) {
                if let Some(value) = update.x {
                    self.x = value;
                }
            }
        }

        #[derive(Debug)]
        pub struct ExampleUpdate {
            x: Option<f32>,
        }
        impl TypeConversion for ExampleUpdate {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                let mut output = Self { x: None };
                let _field_x = input.field::<SchemaFloat>(1);
                if _field_x.count() > 0 {
                    let field = &_field_x;
                    output.x = Some(field.get_or_default());
                }
                Ok(output)
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                if let Some(value) = input.x {
                    output.field::<SchemaFloat>(1).add(value);
                }
                Ok(())
            }
        }
        impl ComponentUpdate<Example> for ExampleUpdate {
            fn merge(&mut self, update: ExampleUpdate) {
                if update.x.is_some() {
                    self.x = update.x;
                }
            }
        }

        #[derive(Debug)]
        pub enum ExampleCommandRequest {
            TestCommand(generated::example::CommandData),
        }

        #[derive(Debug)]
        pub enum ExampleCommandResponse {
            TestCommand(generated::example::CommandData),
        }

        impl Component for Example {
            type Update = generated::example::ExampleUpdate;
            type CommandRequest = generated::example::ExampleCommandRequest;
            type CommandResponse = generated::example::ExampleCommandResponse;

            fn component_id() -> ComponentId {
                1000
            }

            fn from_data(
                data: &SchemaComponentData,
            ) -> Result<generated::example::Example, String> {
                <generated::example::Example as TypeConversion>::from_type(&data.fields())
            }

            fn from_update(
                update: &SchemaComponentUpdate,
            ) -> Result<generated::example::ExampleUpdate, String> {
                <generated::example::ExampleUpdate as TypeConversion>::from_type(&update.fields())
            }

            fn from_request(
                request: &SchemaCommandRequest,
            ) -> Result<generated::example::ExampleCommandRequest, String> {
                match request.command_index() {
            1 => {
                let result = <generated::example::CommandData as TypeConversion>::from_type(&request.object());
                result.and_then(|deserialized| Ok(ExampleCommandRequest::TestCommand(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Example.", request.command_index()))
        }
            }

            fn from_response(
                response: &SchemaCommandResponse,
            ) -> Result<generated::example::ExampleCommandResponse, String> {
                match response.command_index() {
            1 => {
                let result = <generated::example::CommandData as TypeConversion>::from_type(&response.object());
                result.and_then(|deserialized| Ok(ExampleCommandResponse::TestCommand(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Example.", response.command_index()))
        }
            }

            fn to_data(data: &generated::example::Example) -> Result<SchemaComponentData, String> {
                let mut serialized_data = SchemaComponentData::new(Self::component_id());
                <generated::example::Example as TypeConversion>::to_type(
                    data,
                    &mut serialized_data.fields_mut(),
                )?;
                Ok(serialized_data)
            }

            fn to_update(
                update: &generated::example::ExampleUpdate,
            ) -> Result<SchemaComponentUpdate, String> {
                let mut serialized_update = SchemaComponentUpdate::new(Self::component_id());
                <generated::example::ExampleUpdate as TypeConversion>::to_type(
                    update,
                    &mut serialized_update.fields_mut(),
                )?;
                Ok(serialized_update)
            }

            fn to_request(
                request: &generated::example::ExampleCommandRequest,
            ) -> Result<SchemaCommandRequest, String> {
                let command_index = match request {
                    ExampleCommandRequest::TestCommand(_) => 1,
                    _ => unreachable!(),
                };
                let mut serialized_request =
                    SchemaCommandRequest::new(Self::component_id(), command_index);
                match request {
                    ExampleCommandRequest::TestCommand(ref data) => {
                        <generated::example::CommandData as TypeConversion>::to_type(
                            data,
                            &mut serialized_request.object_mut(),
                        )?;
                    }
                    _ => unreachable!(),
                }
                Ok(serialized_request)
            }

            fn to_response(
                response: &generated::example::ExampleCommandResponse,
            ) -> Result<SchemaCommandResponse, String> {
                let command_index = match response {
                    ExampleCommandResponse::TestCommand(_) => 1,
                    _ => unreachable!(),
                };
                let mut serialized_response =
                    SchemaCommandResponse::new(Self::component_id(), command_index);
                match response {
                    ExampleCommandResponse::TestCommand(ref data) => {
                        <generated::example::CommandData as TypeConversion>::to_type(
                            data,
                            &mut serialized_response.object_mut(),
                        )?;
                    }
                    _ => unreachable!(),
                }
                Ok(serialized_response)
            }
        }

    }

    pub mod improbable {
        use spatialos_sdk::worker::component::*;
        use spatialos_sdk::worker::internal::schema::*;
        use std::collections::BTreeMap;

        use super::super::generated;

        /* Enums. */
        /* Types. */
        #[derive(Debug)]
        pub struct ComponentInterest {
            queries: Vec<generated::improbable::ComponentInterest_Query>,
        }
        impl TypeConversion for ComponentInterest {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    queries: {
                        let size = input.field::<SchemaObject>(1).count();
                        let mut l = Vec::with_capacity(size);
                        for i in 0..size {
                            l.push(<generated::improbable::ComponentInterest_Query as TypeConversion>::from_type(&input.field::<SchemaObject>(1).index(i))?);
                        }
                        l
                    },
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                for element in (&input.queries).iter() {
                    <generated::improbable::ComponentInterest_Query as TypeConversion>::to_type(
                        &element,
                        &mut output.field::<SchemaObject>(1).add(),
                    )?;
                }
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_BoxConstraint {
            center: generated::improbable::Coordinates,
            edge_length: generated::improbable::EdgeLength,
        }
        impl TypeConversion for ComponentInterest_BoxConstraint {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    center: <generated::improbable::Coordinates as TypeConversion>::from_type(
                        &input.field::<SchemaObject>(1).get_or_default(),
                    )?,
                    edge_length: <generated::improbable::EdgeLength as TypeConversion>::from_type(
                        &input.field::<SchemaObject>(2).get_or_default(),
                    )?,
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                <generated::improbable::Coordinates as TypeConversion>::to_type(
                    &&input.center,
                    &mut output.field::<SchemaObject>(1).add(),
                )?;
                <generated::improbable::EdgeLength as TypeConversion>::to_type(
                    &&input.edge_length,
                    &mut output.field::<SchemaObject>(2).add(),
                )?;
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_CylinderConstraint {
            center: generated::improbable::Coordinates,
            radius: f64,
        }
        impl TypeConversion for ComponentInterest_CylinderConstraint {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    center: <generated::improbable::Coordinates as TypeConversion>::from_type(
                        &input.field::<SchemaObject>(1).get_or_default(),
                    )?,
                    radius: input.field::<SchemaDouble>(2).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                <generated::improbable::Coordinates as TypeConversion>::to_type(
                    &&input.center,
                    &mut output.field::<SchemaObject>(1).add(),
                )?;
                output.field::<SchemaDouble>(2).add(input.radius);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_Query {
            constraint: generated::improbable::ComponentInterest_QueryConstraint,
            full_snapshot_result: Option<bool>,
            result_component_id: Vec<u32>,
            frequency: Option<f32>,
        }
        impl TypeConversion for ComponentInterest_Query {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
            constraint: <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::from_type(&input.field::<SchemaObject>(1).get_or_default())?,
            full_snapshot_result: if let Some(data) = input.field::<SchemaBool>(2).get() { Some(data) } else { None },
            result_component_id: { let size = input.field::<SchemaUint32>(3).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(input.field::<SchemaUint32>(3).index(i)); }; l },
            frequency: if let Some(data) = input.field::<SchemaFloat>(4).get() { Some(data) } else { None },
        })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::to_type(&&input.constraint, &mut output.field::<SchemaObject>(1).add())?;
                if let Some(data) = input.full_snapshot_result {
                    output.field::<SchemaBool>(2).add(data);
                };
                output
                    .field::<SchemaUint32>(3)
                    .add_list(&&input.result_component_id[..]);
                if let Some(data) = input.frequency {
                    output.field::<SchemaFloat>(4).add(data);
                };
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_QueryConstraint {
            sphere_constraint: Option<generated::improbable::ComponentInterest_SphereConstraint>,
            cylinder_constraint:
                Option<generated::improbable::ComponentInterest_CylinderConstraint>,
            box_constraint: Option<generated::improbable::ComponentInterest_BoxConstraint>,
            relative_sphere_constraint:
                Option<generated::improbable::ComponentInterest_RelativeSphereConstraint>,
            relative_cylinder_constraint:
                Option<generated::improbable::ComponentInterest_RelativeCylinderConstraint>,
            relative_box_constraint:
                Option<generated::improbable::ComponentInterest_RelativeBoxConstraint>,
            entity_id_constraint: Option<i64>,
            component_constraint: Option<u32>,
            and_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
            or_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
        }
        impl TypeConversion for ComponentInterest_QueryConstraint {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    sphere_constraint: if let Some(data) = input.field::<SchemaObject>(1).get() {
                        Some(<generated::improbable::ComponentInterest_SphereConstraint as TypeConversion>::from_type(&data)?)
                    } else {
                        None
                    },
                    cylinder_constraint: if let Some(data) = input.field::<SchemaObject>(2).get() {
                        Some(<generated::improbable::ComponentInterest_CylinderConstraint as TypeConversion>::from_type(&data)?)
                    } else {
                        None
                    },
                    box_constraint: if let Some(data) = input.field::<SchemaObject>(3).get() {
                        Some(<generated::improbable::ComponentInterest_BoxConstraint as TypeConversion>::from_type(&data)?)
                    } else {
                        None
                    },
                    relative_sphere_constraint: if let Some(data) =
                        input.field::<SchemaObject>(4).get()
                    {
                        Some(<generated::improbable::ComponentInterest_RelativeSphereConstraint as TypeConversion>::from_type(&data)?)
                    } else {
                        None
                    },
                    relative_cylinder_constraint: if let Some(data) =
                        input.field::<SchemaObject>(5).get()
                    {
                        Some(<generated::improbable::ComponentInterest_RelativeCylinderConstraint as TypeConversion>::from_type(&data)?)
                    } else {
                        None
                    },
                    relative_box_constraint: if let Some(data) =
                        input.field::<SchemaObject>(6).get()
                    {
                        Some(<generated::improbable::ComponentInterest_RelativeBoxConstraint as TypeConversion>::from_type(&data)?)
                    } else {
                        None
                    },
                    entity_id_constraint: if let Some(data) = input.field::<SchemaInt64>(7).get() {
                        Some(data)
                    } else {
                        None
                    },
                    component_constraint: if let Some(data) = input.field::<SchemaUint32>(8).get() {
                        Some(data)
                    } else {
                        None
                    },
                    and_constraint: {
                        let size = input.field::<SchemaObject>(9).count();
                        let mut l = Vec::with_capacity(size);
                        for i in 0..size {
                            l.push(<generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::from_type(&input.field::<SchemaObject>(9).index(i))?);
                        }
                        l
                    },
                    or_constraint: {
                        let size = input.field::<SchemaObject>(10).count();
                        let mut l = Vec::with_capacity(size);
                        for i in 0..size {
                            l.push(<generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::from_type(&input.field::<SchemaObject>(10).index(i))?);
                        }
                        l
                    },
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                if let Some(ref data) = &input.sphere_constraint {
                    <generated::improbable::ComponentInterest_SphereConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(1).add())?;
                };
                if let Some(ref data) = &input.cylinder_constraint {
                    <generated::improbable::ComponentInterest_CylinderConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(2).add())?;
                };
                if let Some(ref data) = &input.box_constraint {
                    <generated::improbable::ComponentInterest_BoxConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(3).add())?;
                };
                if let Some(ref data) = &input.relative_sphere_constraint {
                    <generated::improbable::ComponentInterest_RelativeSphereConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(4).add())?;
                };
                if let Some(ref data) = &input.relative_cylinder_constraint {
                    <generated::improbable::ComponentInterest_RelativeCylinderConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(5).add())?;
                };
                if let Some(ref data) = &input.relative_box_constraint {
                    <generated::improbable::ComponentInterest_RelativeBoxConstraint as TypeConversion>::to_type(&data, &mut output.field::<SchemaObject>(6).add())?;
                };
                if let Some(data) = input.entity_id_constraint {
                    output.field::<SchemaInt64>(7).add(data);
                };
                if let Some(data) = input.component_constraint {
                    output.field::<SchemaUint32>(8).add(data);
                };
                for element in (&input.and_constraint).iter() {
                    <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::to_type(&element, &mut output.field::<SchemaObject>(9).add())?;
                }
                for element in (&input.or_constraint).iter() {
                    <generated::improbable::ComponentInterest_QueryConstraint as TypeConversion>::to_type(&element, &mut output.field::<SchemaObject>(10).add())?;
                }
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_RelativeBoxConstraint {
            edge_length: generated::improbable::EdgeLength,
        }
        impl TypeConversion for ComponentInterest_RelativeBoxConstraint {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    edge_length: <generated::improbable::EdgeLength as TypeConversion>::from_type(
                        &input.field::<SchemaObject>(1).get_or_default(),
                    )?,
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                <generated::improbable::EdgeLength as TypeConversion>::to_type(
                    &&input.edge_length,
                    &mut output.field::<SchemaObject>(1).add(),
                )?;
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_RelativeCylinderConstraint {
            radius: f64,
        }
        impl TypeConversion for ComponentInterest_RelativeCylinderConstraint {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    radius: input.field::<SchemaDouble>(1).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaDouble>(1).add(input.radius);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_RelativeSphereConstraint {
            radius: f64,
        }
        impl TypeConversion for ComponentInterest_RelativeSphereConstraint {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    radius: input.field::<SchemaDouble>(1).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaDouble>(1).add(input.radius);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct ComponentInterest_SphereConstraint {
            center: generated::improbable::Coordinates,
            radius: f64,
        }
        impl TypeConversion for ComponentInterest_SphereConstraint {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    center: <generated::improbable::Coordinates as TypeConversion>::from_type(
                        &input.field::<SchemaObject>(1).get_or_default(),
                    )?,
                    radius: input.field::<SchemaDouble>(2).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                <generated::improbable::Coordinates as TypeConversion>::to_type(
                    &&input.center,
                    &mut output.field::<SchemaObject>(1).add(),
                )?;
                output.field::<SchemaDouble>(2).add(input.radius);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct Coordinates {
            x: f64,
            y: f64,
            z: f64,
        }
        impl TypeConversion for Coordinates {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    x: input.field::<SchemaDouble>(1).get_or_default(),
                    y: input.field::<SchemaDouble>(2).get_or_default(),
                    z: input.field::<SchemaDouble>(3).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaDouble>(1).add(input.x);
                output.field::<SchemaDouble>(2).add(input.y);
                output.field::<SchemaDouble>(3).add(input.z);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct EdgeLength {
            x: f64,
            y: f64,
            z: f64,
        }
        impl TypeConversion for EdgeLength {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    x: input.field::<SchemaDouble>(1).get_or_default(),
                    y: input.field::<SchemaDouble>(2).get_or_default(),
                    z: input.field::<SchemaDouble>(3).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaDouble>(1).add(input.x);
                output.field::<SchemaDouble>(2).add(input.y);
                output.field::<SchemaDouble>(3).add(input.z);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct Vector3d {
            x: f64,
            y: f64,
            z: f64,
        }
        impl TypeConversion for Vector3d {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    x: input.field::<SchemaDouble>(1).get_or_default(),
                    y: input.field::<SchemaDouble>(2).get_or_default(),
                    z: input.field::<SchemaDouble>(3).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaDouble>(1).add(input.x);
                output.field::<SchemaDouble>(2).add(input.y);
                output.field::<SchemaDouble>(3).add(input.z);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct Vector3f {
            x: f32,
            y: f32,
            z: f32,
        }
        impl TypeConversion for Vector3f {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    x: input.field::<SchemaFloat>(1).get_or_default(),
                    y: input.field::<SchemaFloat>(2).get_or_default(),
                    z: input.field::<SchemaFloat>(3).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaFloat>(1).add(input.x);
                output.field::<SchemaFloat>(2).add(input.y);
                output.field::<SchemaFloat>(3).add(input.z);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct WorkerAttributeSet {
            attribute: Vec<String>,
        }
        impl TypeConversion for WorkerAttributeSet {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    attribute: {
                        let size = input.field::<SchemaString>(1).count();
                        let mut l = Vec::with_capacity(size);
                        for i in 0..size {
                            l.push(input.field::<SchemaString>(1).index(i));
                        }
                        l
                    },
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output
                    .field::<SchemaString>(1)
                    .add_list(&&input.attribute[..]);
                Ok(())
            }
        }

        #[derive(Debug)]
        pub struct WorkerRequirementSet {
            attribute_set: Vec<generated::improbable::WorkerAttributeSet>,
        }
        impl TypeConversion for WorkerRequirementSet {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    attribute_set: {
                        let size = input.field::<SchemaObject>(1).count();
                        let mut l = Vec::with_capacity(size);
                        for i in 0..size {
                            l.push(<generated::improbable::WorkerAttributeSet as TypeConversion>::from_type(&input.field::<SchemaObject>(1).index(i))?);
                        }
                        l
                    },
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                for element in (&input.attribute_set).iter() {
                    <generated::improbable::WorkerAttributeSet as TypeConversion>::to_type(
                        &element,
                        &mut output.field::<SchemaObject>(1).add(),
                    )?;
                }
                Ok(())
            }
        }

        /* Components. */

        #[derive(Debug)]
        pub struct EntityAcl {
            read_acl: generated::improbable::WorkerRequirementSet,
            component_write_acl: BTreeMap<u32, generated::improbable::WorkerRequirementSet>,
        }
        impl TypeConversion for EntityAcl {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    read_acl:
                        <generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(
                            &input.field::<SchemaObject>(1).get_or_default(),
                        )?,
                    component_write_acl: {
                        let size = input.field::<SchemaObject>(2).count();
                        let mut m = BTreeMap::new();
                        for i in 0..size {
                            let kv = input.field::<SchemaObject>(2).index(i);
                            m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?);
                        }
                        m
                    },
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(
                    &&input.read_acl,
                    &mut output.field::<SchemaObject>(1).add(),
                )?;
                for (k, v) in &input.component_write_acl {
                    let object = output.field::<SchemaObject>(2).add();
                    object.field::<SchemaUint32>(1).add(*k);
                    <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(
                        &v,
                        &mut object.field::<SchemaObject>(2).add(),
                    )?;
                }
                Ok(())
            }
        }
        impl ComponentData<EntityAcl> for EntityAcl {
            fn merge(&mut self, update: EntityAclUpdate) {
                if let Some(value) = update.read_acl {
                    self.read_acl = value;
                }
                if let Some(value) = update.component_write_acl {
                    self.component_write_acl = value;
                }
            }
        }

        #[derive(Debug)]
        pub struct EntityAclUpdate {
            read_acl: Option<generated::improbable::WorkerRequirementSet>,
            component_write_acl: Option<BTreeMap<u32, generated::improbable::WorkerRequirementSet>>,
        }
        impl TypeConversion for EntityAclUpdate {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                let mut output = Self {
                    read_acl: None,
                    component_write_acl: None,
                };
                let _field_read_acl = input.field::<SchemaObject>(1);
                if _field_read_acl.count() > 0 {
                    let field = &_field_read_acl;
                    output.read_acl = Some(
                        <generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(
                            &field.get_or_default(),
                        )?,
                    );
                }
                let _field_component_write_acl = input.field::<SchemaObject>(2);
                if _field_component_write_acl.count() > 0 {
                    let field = &_field_component_write_acl;
                    output.component_write_acl = Some({
                        let size = field.count();
                        let mut m = BTreeMap::new();
                        for i in 0..size {
                            let kv = field.index(i);
                            m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::WorkerRequirementSet as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?);
                        }
                        m
                    });
                }
                Ok(output)
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                if let Some(ref value) = input.read_acl {
                    <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(
                        &value,
                        &mut output.field::<SchemaObject>(1).add(),
                    )?;
                }
                if let Some(ref value) = input.component_write_acl {
                    for (k, v) in value {
                        let object = output.field::<SchemaObject>(2).add();
                        object.field::<SchemaUint32>(1).add(*k);
                        <generated::improbable::WorkerRequirementSet as TypeConversion>::to_type(
                            &v,
                            &mut object.field::<SchemaObject>(2).add(),
                        )?;
                    }
                }
                Ok(())
            }
        }
        impl ComponentUpdate<EntityAcl> for EntityAclUpdate {
            fn merge(&mut self, update: EntityAclUpdate) {
                if update.read_acl.is_some() {
                    self.read_acl = update.read_acl;
                }
                if update.component_write_acl.is_some() {
                    self.component_write_acl = update.component_write_acl;
                }
            }
        }

        #[derive(Debug)]
        pub enum EntityAclCommandRequest {}

        #[derive(Debug)]
        pub enum EntityAclCommandResponse {}

        impl Component for EntityAcl {
            type Update = generated::improbable::EntityAclUpdate;
            type CommandRequest = generated::improbable::EntityAclCommandRequest;
            type CommandResponse = generated::improbable::EntityAclCommandResponse;

            fn component_id() -> ComponentId {
                50
            }

            fn from_data(
                data: &SchemaComponentData,
            ) -> Result<generated::improbable::EntityAcl, String> {
                <generated::improbable::EntityAcl as TypeConversion>::from_type(&data.fields())
            }

            fn from_update(
                update: &SchemaComponentUpdate,
            ) -> Result<generated::improbable::EntityAclUpdate, String> {
                <generated::improbable::EntityAclUpdate as TypeConversion>::from_type(
                    &update.fields(),
                )
            }

            fn from_request(
                request: &SchemaCommandRequest,
            ) -> Result<generated::improbable::EntityAclCommandRequest, String> {
                match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component EntityAcl.", request.command_index()))
        }
            }

            fn from_response(
                response: &SchemaCommandResponse,
            ) -> Result<generated::improbable::EntityAclCommandResponse, String> {
                match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component EntityAcl.", response.command_index()))
        }
            }

            fn to_data(
                data: &generated::improbable::EntityAcl,
            ) -> Result<SchemaComponentData, String> {
                let mut serialized_data = SchemaComponentData::new(Self::component_id());
                <generated::improbable::EntityAcl as TypeConversion>::to_type(
                    data,
                    &mut serialized_data.fields_mut(),
                )?;
                Ok(serialized_data)
            }

            fn to_update(
                update: &generated::improbable::EntityAclUpdate,
            ) -> Result<SchemaComponentUpdate, String> {
                let mut serialized_update = SchemaComponentUpdate::new(Self::component_id());
                <generated::improbable::EntityAclUpdate as TypeConversion>::to_type(
                    update,
                    &mut serialized_update.fields_mut(),
                )?;
                Ok(serialized_update)
            }

            fn to_request(
                request: &generated::improbable::EntityAclCommandRequest,
            ) -> Result<SchemaCommandRequest, String> {
                let command_index = match request {
                    _ => unreachable!(),
                };
                let mut serialized_request =
                    SchemaCommandRequest::new(Self::component_id(), command_index);
                match request {
                    _ => unreachable!(),
                }
                Ok(serialized_request)
            }

            fn to_response(
                response: &generated::improbable::EntityAclCommandResponse,
            ) -> Result<SchemaCommandResponse, String> {
                let command_index = match response {
                    _ => unreachable!(),
                };
                let mut serialized_response =
                    SchemaCommandResponse::new(Self::component_id(), command_index);
                match response {
                    _ => unreachable!(),
                }
                Ok(serialized_response)
            }
        }

        #[derive(Debug)]
        pub struct Interest {
            component_interest: BTreeMap<u32, generated::improbable::ComponentInterest>,
        }
        impl TypeConversion for Interest {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    component_interest: {
                        let size = input.field::<SchemaObject>(1).count();
                        let mut m = BTreeMap::new();
                        for i in 0..size {
                            let kv = input.field::<SchemaObject>(1).index(i);
                            m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::ComponentInterest as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?);
                        }
                        m
                    },
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                for (k, v) in &input.component_interest {
                    let object = output.field::<SchemaObject>(1).add();
                    object.field::<SchemaUint32>(1).add(*k);
                    <generated::improbable::ComponentInterest as TypeConversion>::to_type(
                        &v,
                        &mut object.field::<SchemaObject>(2).add(),
                    )?;
                }
                Ok(())
            }
        }
        impl ComponentData<Interest> for Interest {
            fn merge(&mut self, update: InterestUpdate) {
                if let Some(value) = update.component_interest {
                    self.component_interest = value;
                }
            }
        }

        #[derive(Debug)]
        pub struct InterestUpdate {
            component_interest: Option<BTreeMap<u32, generated::improbable::ComponentInterest>>,
        }
        impl TypeConversion for InterestUpdate {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                let mut output = Self {
                    component_interest: None,
                };
                let _field_component_interest = input.field::<SchemaObject>(1);
                if _field_component_interest.count() > 0 {
                    let field = &_field_component_interest;
                    output.component_interest = Some({
                        let size = field.count();
                        let mut m = BTreeMap::new();
                        for i in 0..size {
                            let kv = field.index(i);
                            m.insert(kv.field::<SchemaUint32>(1).get_or_default(), <generated::improbable::ComponentInterest as TypeConversion>::from_type(&kv.field::<SchemaObject>(2).get_or_default())?);
                        }
                        m
                    });
                }
                Ok(output)
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                if let Some(ref value) = input.component_interest {
                    for (k, v) in value {
                        let object = output.field::<SchemaObject>(1).add();
                        object.field::<SchemaUint32>(1).add(*k);
                        <generated::improbable::ComponentInterest as TypeConversion>::to_type(
                            &v,
                            &mut object.field::<SchemaObject>(2).add(),
                        )?;
                    }
                }
                Ok(())
            }
        }
        impl ComponentUpdate<Interest> for InterestUpdate {
            fn merge(&mut self, update: InterestUpdate) {
                if update.component_interest.is_some() {
                    self.component_interest = update.component_interest;
                }
            }
        }

        #[derive(Debug)]
        pub enum InterestCommandRequest {}

        #[derive(Debug)]
        pub enum InterestCommandResponse {}

        impl Component for Interest {
            type Update = generated::improbable::InterestUpdate;
            type CommandRequest = generated::improbable::InterestCommandRequest;
            type CommandResponse = generated::improbable::InterestCommandResponse;

            fn component_id() -> ComponentId {
                58
            }

            fn from_data(
                data: &SchemaComponentData,
            ) -> Result<generated::improbable::Interest, String> {
                <generated::improbable::Interest as TypeConversion>::from_type(&data.fields())
            }

            fn from_update(
                update: &SchemaComponentUpdate,
            ) -> Result<generated::improbable::InterestUpdate, String> {
                <generated::improbable::InterestUpdate as TypeConversion>::from_type(
                    &update.fields(),
                )
            }

            fn from_request(
                request: &SchemaCommandRequest,
            ) -> Result<generated::improbable::InterestCommandRequest, String> {
                match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Interest.", request.command_index()))
        }
            }

            fn from_response(
                response: &SchemaCommandResponse,
            ) -> Result<generated::improbable::InterestCommandResponse, String> {
                match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Interest.", response.command_index()))
        }
            }

            fn to_data(
                data: &generated::improbable::Interest,
            ) -> Result<SchemaComponentData, String> {
                let mut serialized_data = SchemaComponentData::new(Self::component_id());
                <generated::improbable::Interest as TypeConversion>::to_type(
                    data,
                    &mut serialized_data.fields_mut(),
                )?;
                Ok(serialized_data)
            }

            fn to_update(
                update: &generated::improbable::InterestUpdate,
            ) -> Result<SchemaComponentUpdate, String> {
                let mut serialized_update = SchemaComponentUpdate::new(Self::component_id());
                <generated::improbable::InterestUpdate as TypeConversion>::to_type(
                    update,
                    &mut serialized_update.fields_mut(),
                )?;
                Ok(serialized_update)
            }

            fn to_request(
                request: &generated::improbable::InterestCommandRequest,
            ) -> Result<SchemaCommandRequest, String> {
                let command_index = match request {
                    _ => unreachable!(),
                };
                let mut serialized_request =
                    SchemaCommandRequest::new(Self::component_id(), command_index);
                match request {
                    _ => unreachable!(),
                }
                Ok(serialized_request)
            }

            fn to_response(
                response: &generated::improbable::InterestCommandResponse,
            ) -> Result<SchemaCommandResponse, String> {
                let command_index = match response {
                    _ => unreachable!(),
                };
                let mut serialized_response =
                    SchemaCommandResponse::new(Self::component_id(), command_index);
                match response {
                    _ => unreachable!(),
                }
                Ok(serialized_response)
            }
        }

        #[derive(Debug)]
        pub struct Metadata {
            entity_type: String,
        }
        impl TypeConversion for Metadata {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    entity_type: input.field::<SchemaString>(1).get_or_default(),
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                output.field::<SchemaString>(1).add(&&input.entity_type);
                Ok(())
            }
        }
        impl ComponentData<Metadata> for Metadata {
            fn merge(&mut self, update: MetadataUpdate) {
                if let Some(value) = update.entity_type {
                    self.entity_type = value;
                }
            }
        }

        #[derive(Debug)]
        pub struct MetadataUpdate {
            entity_type: Option<String>,
        }
        impl TypeConversion for MetadataUpdate {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                let mut output = Self { entity_type: None };
                let _field_entity_type = input.field::<SchemaString>(1);
                if _field_entity_type.count() > 0 {
                    let field = &_field_entity_type;
                    output.entity_type = Some(field.get_or_default());
                }
                Ok(output)
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                if let Some(ref value) = input.entity_type {
                    output.field::<SchemaString>(1).add(&value);
                }
                Ok(())
            }
        }
        impl ComponentUpdate<Metadata> for MetadataUpdate {
            fn merge(&mut self, update: MetadataUpdate) {
                if update.entity_type.is_some() {
                    self.entity_type = update.entity_type;
                }
            }
        }

        #[derive(Debug)]
        pub enum MetadataCommandRequest {}

        #[derive(Debug)]
        pub enum MetadataCommandResponse {}

        impl Component for Metadata {
            type Update = generated::improbable::MetadataUpdate;
            type CommandRequest = generated::improbable::MetadataCommandRequest;
            type CommandResponse = generated::improbable::MetadataCommandResponse;

            fn component_id() -> ComponentId {
                53
            }

            fn from_data(
                data: &SchemaComponentData,
            ) -> Result<generated::improbable::Metadata, String> {
                <generated::improbable::Metadata as TypeConversion>::from_type(&data.fields())
            }

            fn from_update(
                update: &SchemaComponentUpdate,
            ) -> Result<generated::improbable::MetadataUpdate, String> {
                <generated::improbable::MetadataUpdate as TypeConversion>::from_type(
                    &update.fields(),
                )
            }

            fn from_request(
                request: &SchemaCommandRequest,
            ) -> Result<generated::improbable::MetadataCommandRequest, String> {
                match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Metadata.", request.command_index()))
        }
            }

            fn from_response(
                response: &SchemaCommandResponse,
            ) -> Result<generated::improbable::MetadataCommandResponse, String> {
                match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Metadata.", response.command_index()))
        }
            }

            fn to_data(
                data: &generated::improbable::Metadata,
            ) -> Result<SchemaComponentData, String> {
                let mut serialized_data = SchemaComponentData::new(Self::component_id());
                <generated::improbable::Metadata as TypeConversion>::to_type(
                    data,
                    &mut serialized_data.fields_mut(),
                )?;
                Ok(serialized_data)
            }

            fn to_update(
                update: &generated::improbable::MetadataUpdate,
            ) -> Result<SchemaComponentUpdate, String> {
                let mut serialized_update = SchemaComponentUpdate::new(Self::component_id());
                <generated::improbable::MetadataUpdate as TypeConversion>::to_type(
                    update,
                    &mut serialized_update.fields_mut(),
                )?;
                Ok(serialized_update)
            }

            fn to_request(
                request: &generated::improbable::MetadataCommandRequest,
            ) -> Result<SchemaCommandRequest, String> {
                let command_index = match request {
                    _ => unreachable!(),
                };
                let mut serialized_request =
                    SchemaCommandRequest::new(Self::component_id(), command_index);
                match request {
                    _ => unreachable!(),
                }
                Ok(serialized_request)
            }

            fn to_response(
                response: &generated::improbable::MetadataCommandResponse,
            ) -> Result<SchemaCommandResponse, String> {
                let command_index = match response {
                    _ => unreachable!(),
                };
                let mut serialized_response =
                    SchemaCommandResponse::new(Self::component_id(), command_index);
                match response {
                    _ => unreachable!(),
                }
                Ok(serialized_response)
            }
        }

        #[derive(Debug)]
        pub struct Persistence {}
        impl TypeConversion for Persistence {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {})
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                Ok(())
            }
        }
        impl ComponentData<Persistence> for Persistence {
            fn merge(&mut self, update: PersistenceUpdate) {}
        }

        #[derive(Debug)]
        pub struct PersistenceUpdate {}
        impl TypeConversion for PersistenceUpdate {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                let mut output = Self {};
                Ok(output)
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                Ok(())
            }
        }
        impl ComponentUpdate<Persistence> for PersistenceUpdate {
            fn merge(&mut self, update: PersistenceUpdate) {}
        }

        #[derive(Debug)]
        pub enum PersistenceCommandRequest {}

        #[derive(Debug)]
        pub enum PersistenceCommandResponse {}

        impl Component for Persistence {
            type Update = generated::improbable::PersistenceUpdate;
            type CommandRequest = generated::improbable::PersistenceCommandRequest;
            type CommandResponse = generated::improbable::PersistenceCommandResponse;

            fn component_id() -> ComponentId {
                55
            }

            fn from_data(
                data: &SchemaComponentData,
            ) -> Result<generated::improbable::Persistence, String> {
                <generated::improbable::Persistence as TypeConversion>::from_type(&data.fields())
            }

            fn from_update(
                update: &SchemaComponentUpdate,
            ) -> Result<generated::improbable::PersistenceUpdate, String> {
                <generated::improbable::PersistenceUpdate as TypeConversion>::from_type(
                    &update.fields(),
                )
            }

            fn from_request(
                request: &SchemaCommandRequest,
            ) -> Result<generated::improbable::PersistenceCommandRequest, String> {
                match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Persistence.", request.command_index()))
        }
            }

            fn from_response(
                response: &SchemaCommandResponse,
            ) -> Result<generated::improbable::PersistenceCommandResponse, String> {
                match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Persistence.", response.command_index()))
        }
            }

            fn to_data(
                data: &generated::improbable::Persistence,
            ) -> Result<SchemaComponentData, String> {
                let mut serialized_data = SchemaComponentData::new(Self::component_id());
                <generated::improbable::Persistence as TypeConversion>::to_type(
                    data,
                    &mut serialized_data.fields_mut(),
                )?;
                Ok(serialized_data)
            }

            fn to_update(
                update: &generated::improbable::PersistenceUpdate,
            ) -> Result<SchemaComponentUpdate, String> {
                let mut serialized_update = SchemaComponentUpdate::new(Self::component_id());
                <generated::improbable::PersistenceUpdate as TypeConversion>::to_type(
                    update,
                    &mut serialized_update.fields_mut(),
                )?;
                Ok(serialized_update)
            }

            fn to_request(
                request: &generated::improbable::PersistenceCommandRequest,
            ) -> Result<SchemaCommandRequest, String> {
                let command_index = match request {
                    _ => unreachable!(),
                };
                let mut serialized_request =
                    SchemaCommandRequest::new(Self::component_id(), command_index);
                match request {
                    _ => unreachable!(),
                }
                Ok(serialized_request)
            }

            fn to_response(
                response: &generated::improbable::PersistenceCommandResponse,
            ) -> Result<SchemaCommandResponse, String> {
                let command_index = match response {
                    _ => unreachable!(),
                };
                let mut serialized_response =
                    SchemaCommandResponse::new(Self::component_id(), command_index);
                match response {
                    _ => unreachable!(),
                }
                Ok(serialized_response)
            }
        }

        #[derive(Debug)]
        pub struct Position {
            coords: generated::improbable::Coordinates,
        }
        impl TypeConversion for Position {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                Ok(Self {
                    coords: <generated::improbable::Coordinates as TypeConversion>::from_type(
                        &input.field::<SchemaObject>(1).get_or_default(),
                    )?,
                })
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                <generated::improbable::Coordinates as TypeConversion>::to_type(
                    &&input.coords,
                    &mut output.field::<SchemaObject>(1).add(),
                )?;
                Ok(())
            }
        }
        impl ComponentData<Position> for Position {
            fn merge(&mut self, update: PositionUpdate) {
                if let Some(value) = update.coords {
                    self.coords = value;
                }
            }
        }

        #[derive(Debug)]
        pub struct PositionUpdate {
            coords: Option<generated::improbable::Coordinates>,
        }
        impl TypeConversion for PositionUpdate {
            fn from_type(input: &SchemaObject) -> Result<Self, String> {
                let mut output = Self { coords: None };
                let _field_coords = input.field::<SchemaObject>(1);
                if _field_coords.count() > 0 {
                    let field = &_field_coords;
                    output.coords = Some(
                        <generated::improbable::Coordinates as TypeConversion>::from_type(
                            &field.get_or_default(),
                        )?,
                    );
                }
                Ok(output)
            }
            fn to_type(input: &Self, output: &mut SchemaObject) -> Result<(), String> {
                if let Some(ref value) = input.coords {
                    <generated::improbable::Coordinates as TypeConversion>::to_type(
                        &value,
                        &mut output.field::<SchemaObject>(1).add(),
                    )?;
                }
                Ok(())
            }
        }
        impl ComponentUpdate<Position> for PositionUpdate {
            fn merge(&mut self, update: PositionUpdate) {
                if update.coords.is_some() {
                    self.coords = update.coords;
                }
            }
        }

        #[derive(Debug)]
        pub enum PositionCommandRequest {}

        #[derive(Debug)]
        pub enum PositionCommandResponse {}

        impl Component for Position {
            type Update = generated::improbable::PositionUpdate;
            type CommandRequest = generated::improbable::PositionCommandRequest;
            type CommandResponse = generated::improbable::PositionCommandResponse;

            fn component_id() -> ComponentId {
                54
            }

            fn from_data(
                data: &SchemaComponentData,
            ) -> Result<generated::improbable::Position, String> {
                <generated::improbable::Position as TypeConversion>::from_type(&data.fields())
            }

            fn from_update(
                update: &SchemaComponentUpdate,
            ) -> Result<generated::improbable::PositionUpdate, String> {
                <generated::improbable::PositionUpdate as TypeConversion>::from_type(
                    &update.fields(),
                )
            }

            fn from_request(
                request: &SchemaCommandRequest,
            ) -> Result<generated::improbable::PositionCommandRequest, String> {
                match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Position.", request.command_index()))
        }
            }

            fn from_response(
                response: &SchemaCommandResponse,
            ) -> Result<generated::improbable::PositionCommandResponse, String> {
                match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Position.", response.command_index()))
        }
            }

            fn to_data(
                data: &generated::improbable::Position,
            ) -> Result<SchemaComponentData, String> {
                let mut serialized_data = SchemaComponentData::new(Self::component_id());
                <generated::improbable::Position as TypeConversion>::to_type(
                    data,
                    &mut serialized_data.fields_mut(),
                )?;
                Ok(serialized_data)
            }

            fn to_update(
                update: &generated::improbable::PositionUpdate,
            ) -> Result<SchemaComponentUpdate, String> {
                let mut serialized_update = SchemaComponentUpdate::new(Self::component_id());
                <generated::improbable::PositionUpdate as TypeConversion>::to_type(
                    update,
                    &mut serialized_update.fields_mut(),
                )?;
                Ok(serialized_update)
            }

            fn to_request(
                request: &generated::improbable::PositionCommandRequest,
            ) -> Result<SchemaCommandRequest, String> {
                let command_index = match request {
                    _ => unreachable!(),
                };
                let mut serialized_request =
                    SchemaCommandRequest::new(Self::component_id(), command_index);
                match request {
                    _ => unreachable!(),
                }
                Ok(serialized_request)
            }

            fn to_response(
                response: &generated::improbable::PositionCommandResponse,
            ) -> Result<SchemaCommandResponse, String> {
                let command_index = match response {
                    _ => unreachable!(),
                };
                let mut serialized_response =
                    SchemaCommandResponse::new(Self::component_id(), command_index);
                match response {
                    _ => unreachable!(),
                }
                Ok(serialized_response)
            }
        }

    }
}

pub use self::generated::example;
pub use self::generated::improbable;
