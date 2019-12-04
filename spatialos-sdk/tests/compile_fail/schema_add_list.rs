use spatialos_sdk::worker::schema::*;

fn main() {
    let mut component_data = SchemaComponentData::new();
    let fields = component_data.fields_mut();

    {
        let data = vec![1, 2, 3, 4];

        // BAD! Should be an error!
        fields.add_list::<SchemaInt32>(1, &data);
    }

    dbg!(fields);
}
