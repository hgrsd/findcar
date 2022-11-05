use crate::{args::Args, hit::Hit};

use self::{limit::Limit, sort::Sort};

mod limit;
mod sort;

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

impl From<&Args> for Pipeline {
    fn from(args: &Args) -> Self {
        let mut actions: Vec<Box<dyn Action>> = vec![];

        let sort: Option<Box<Sort>> = args.into();
        if let Some(s) = sort {
            actions.push(s);
        }

        let limit: Option<Box<Limit>> = args.into();
        if let Some(l) = limit {
            actions.push(l);
        }

        Pipeline { actions }
    }
}

impl Action for Pipeline {
    /// Execute the entire pipeline of actions
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit> {
        self.actions.iter().fold(hits, |acc, cur| cur.execute(acc))
    }
}
