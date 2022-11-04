use clap::Parser;

mod args;
mod emit;
mod engine;
mod hit;
mod search;
mod target;

use emit::{Emit, TextEmitter};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let mut target = target::Target::new();

    if let Some(make) = args.make {
        target.make(&make);
    }

    if let Some(model) = args.model {
        target.model(&model);
    }

    if let Some(min_year) = args.min_year {
        target.min_year(&min_year);
    }

    if let Some(max_year) = args.max_year {
        target.max_year(&max_year);
    }

    if let Some(min_kms) = args.min_kms {
        target.min_kms(&min_kms);
    }

    if let Some(max_kms) = args.max_kms {
        target.max_kms(&max_kms);
    }

    if let Some(min_price) = args.min_price {
        target.min_price(&min_price);
    }

    if let Some(max_price) = args.max_price {
        target.max_price(&max_price);
    }

    let searchers: Vec<Box<dyn search::Searcher>> = vec![Box::new(search::DoneDealIE {})];
    let engine = engine::Engine::with_searchers(searchers);
    let results = engine.search(&target).await;

    let emitter = TextEmitter::new();
    emitter.emit(results);
}
