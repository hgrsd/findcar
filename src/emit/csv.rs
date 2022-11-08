use crate::hit::Hit;

use super::Emit;

pub struct CsvEmitter {}

impl CsvEmitter {
    pub fn new() -> Self {
        CsvEmitter {}
    }
}

impl Emit for CsvEmitter {
    fn emit(&self, hits: Vec<Hit>) {
        let mut wtr = csv::Writer::from_writer(vec![]);

        for hit in hits {
            wtr.serialize(hit).unwrap();
        }
        println!("{}", String::from_utf8(wtr.into_inner().unwrap()).unwrap());
    }
}
