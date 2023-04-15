use crate::hit::Hit;

pub mod limit;
pub mod sort;

/// A trait for any post-processing action that takes hits and process them
pub trait Action {
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit>;
}

/// This struct expresses numerous actions which will be performed in-order, to process a vector of
/// Hits.
pub struct Pipeline {
    /// The actions that this Pipeline is made up of
    actions: Vec<Box<dyn Action>>,
}

impl Pipeline {
    pub fn from(actions: Vec<Box<dyn Action>>) -> Pipeline {
        Pipeline { actions }
    }
}

impl Action for Pipeline {
    /// Execute the entire pipeline of actions
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit> {
        self.actions.iter().fold(hits, |acc, cur| cur.execute(acc))
    }
}
