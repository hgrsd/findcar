use crate::{args::Args, hit::Hit};

use self::sort::{Sort, SortBy, SortOrder};

mod limit;
mod sort;

pub trait Action {
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit>;
}

pub struct Pipeline {
    actions: Vec<Box<dyn Action>>,
}

impl From<&Args> for Option<Box<Sort>> {
    fn from(args: &Args) -> Self {
        match &args.sort_by {
            None => None,
            Some(key) => {
                let sort_by = match key.as_str() {
                    "price" => SortBy::Price,
                    "mileage" => SortBy::Mileage,
                    "year" => SortBy::Year,
                    _ => {
                        println!("Unrecognised key for sorting: {}, defaulting to price", key);
                        SortBy::Price
                    }
                };

                let sort_order = match &args.sort_order {
                    Some(o) => match o.as_str() {
                        "ASC" => SortOrder::Asc,
                        "DESC" => SortOrder::Desc,
                        _ => {
                            println!("Unrecognised sort order: {}, default to ascending", o);
                            SortOrder::Asc
                        }
                    },
                    None => SortOrder::Asc,
                };

                Some(Box::new(Sort::new(sort_by, sort_order)))
            }
        }
    }
}
impl From<&Args> for Pipeline {
    fn from(args: &Args) -> Self {
        let mut actions: Vec<Box<dyn Action>> = vec![];

        let sort: Option<Box<Sort>> = args.into();
        if let Some(s) = sort {
            actions.push(s);
        }

        Pipeline { actions }
    }
}

impl Action for Pipeline {
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit> {
        self.actions.iter().fold(hits, |acc, cur| cur.execute(acc))
    }
}
