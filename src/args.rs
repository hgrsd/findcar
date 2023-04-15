use clap::Parser;

use crate::post_processing::{
    limit::Limit,
    sort::{Sort, SortBy, SortOrder},
    Action, Pipeline,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Optional, make of the car to search for
    #[arg(long)]
    pub make: Option<String>,

    /// Optional, model of the car to search for
    #[arg(long)]
    pub model: Option<String>,

    /// Optional, minimum year of registration
    #[arg(long)]
    pub min_year: Option<String>,

    /// Optional, maximum year of registration
    #[arg(long)]
    pub max_year: Option<String>,

    /// Optional, minimum kms
    #[arg(long)]
    pub min_kms: Option<String>,

    /// Optional, maximum kms
    #[arg(long)]
    pub max_kms: Option<String>,

    /// Optional, minimum price
    #[arg(long)]
    pub min_price: Option<String>,

    /// Optional, maximum price
    #[arg(long)]
    pub max_price: Option<String>,

    /// Optional, value to sort by. Options are: price, year, mileage
    #[arg(long)]
    pub sort_by: Option<String>,

    /// Optional, sort order. Options are ASC, DESC. If not specified, but a sort-by value *is*,
    /// then ASC will be used by default.
    #[arg(long)]
    pub sort_order: Option<String>,

    /// Optional, maximum number of results to return
    #[arg(long)]
    pub limit: Option<usize>,

    /// Optional, emitter for the results. Options are: csv, json, text. Default is text.
    #[arg(long)]
    pub emitter: Option<String>,

    /// Optional, search engine to use. Options are donedeal_ie, carzone_ie. Default is to use all available engines.
    /// Example: ./findcar [other opts] --search-engine carzone_ie --search-engine donedeal_ie
    #[arg(long)]
    pub search_engine: Option<Vec<String>>,
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

        Pipeline::from(actions)
    }
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

impl From<&Args> for Option<Box<Limit>> {
    fn from(args: &Args) -> Self {
        match args.limit {
            Some(lim) => Some(Box::new(Limit::new(lim))),
            None => None,
        }
    }
}
