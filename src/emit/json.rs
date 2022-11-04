use crate::hit::Hit;

use super::Emit;

pub struct JsonEmitter {}

impl JsonEmitter {
    pub fn new() -> Self {
        JsonEmitter {}
    }
}

impl Emit for JsonEmitter {
    fn emit(&self, hits: Vec<Hit>) {
        let serialized = serde_json::to_string_pretty(&hits).unwrap();
        println!("{}", serialized);
    }
}
