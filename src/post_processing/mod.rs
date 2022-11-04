use crate::{args::Args, hit::Hit};

use self::{limit::Limit, sort::Sort};

mod limit;
mod sort;

pub trait Action {
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit>;
}

pub struct Pipeline {
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
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit> {
        self.actions.iter().fold(hits, |acc, cur| cur.execute(acc))
    }
}
