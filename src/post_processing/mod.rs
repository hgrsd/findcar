use crate::hit::Hit;

mod limit;
mod sort;

pub trait Action {
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit>;
}

struct Pipeline {
    actions: Vec<Box<dyn Action>>,
}
