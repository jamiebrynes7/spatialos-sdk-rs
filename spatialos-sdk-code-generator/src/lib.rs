#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate t4rust_derive;

extern crate heck;

pub mod generator;
#[allow(non_camel_case_types)]
pub mod schema_bundle;

#[cfg(test)]
mod tests {
    use generator;
    use schema_bundle;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn deserialize_bundle() {
        let mut file =
            File::open("data/test.sb.json").expect("Unable to open the test schema bundle.");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the test schema bundle");

        let bundle = schema_bundle::load_bundle(&contents);
        assert!(
            bundle.is_ok(),
            "Schema bundle contains an error: {:?}",
            bundle.err().unwrap()
        );
        println!("Bundle contents: {:#?}", bundle);
        println!(
            "Generated code: {}",
            generator::generate_code(bundle.unwrap())
        );
    }
}
