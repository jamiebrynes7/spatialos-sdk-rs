#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[allow(non_camel_case_types)]
pub mod schemabundle;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use schemabundle;

    #[test]
    fn deserialize_bundle() {
        let mut file = File::open("data/test.bundle.json").expect("Unable to open the test schema bundle.");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the test schema bundle");

        let bundle = schemabundle::load_bundle(&contents);
        assert!(bundle.is_ok(), "Schema bundle contains an error: {:?}", bundle.err().unwrap());
        println!("Bundle contents: {:#?}", bundle.unwrap());
    }
}