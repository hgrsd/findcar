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
        let mut wtr = csv::Writer::from_writer(std::io::stdout());

        for hit in hits {
            wtr.serialize(hit).unwrap();
        }
        wtr.flush().expect("Error flushing CSV to stdout");
    }
}
