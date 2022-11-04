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
    let query: query::Query = (&args).into();

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
