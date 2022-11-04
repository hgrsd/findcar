use clap::Parser;

mod args;
mod emit;
mod engine;
mod hit;
mod query;
mod search;

use emit::{Emit, JsonEmitter, TextEmitter};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let mut query = query::Query::new();

    if let Some(make) = args.make {
        query.make(&make);
    }

    if let Some(model) = args.model {
        query.model(&model);
    }

    if let Some(min_year) = args.min_year {
        query.min_year(&min_year);
    }

    if let Some(max_year) = args.max_year {
        query.max_year(&max_year);
    }

    if let Some(min_kms) = args.min_kms {
        query.min_kms(&min_kms);
    }

    if let Some(max_kms) = args.max_kms {
        query.max_kms(&max_kms);
    }

    if let Some(min_price) = args.min_price {
        query.min_price(&min_price);
    }

    if let Some(max_price) = args.max_price {
        query.max_price(&max_price);
    }

    if let Some(sort_by) = args.sort_by {
        query.sort_by(&sort_by);
    }

    if let Some(sort_order) = args.sort_order {
        query.sort_order(&sort_order);
    }

    if let Some(limit) = args.limit {
        query.limit(limit);
    }

    let searchers: Vec<Box<dyn search::Searcher>> = vec![Box::new(search::DoneDealIE {})];
    let engine = engine::Engine::with_searchers(searchers);
    let results = engine.search(&query).await;

    let emitter: Box<dyn Emit> = match args.emitter {
        Some(val) => {
            if val.to_uppercase() == "JSON" {
                Box::new(JsonEmitter::new())
            } else {
                Box::new(TextEmitter::new())
            }
        }
        None => Box::new(TextEmitter::new()),
    };
    emitter.emit(results);
}
