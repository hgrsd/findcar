use clap::Parser;

mod args;
mod emit;
mod engine;
mod hit;
mod post_processing;
mod query;
mod search;

use emit::{Emit, JsonEmitter, TextEmitter};
use post_processing::{Action, Pipeline};

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let query: query::Query = (&args).into();

    let searchers: Vec<Box<dyn search::Searcher>> = vec![
        Box::new(search::DoneDealIE {}),
        Box::new(search::CarZoneIE {}),
    ];

    let engine = engine::Engine::with_searchers(searchers);
    let results = engine.search(&query).await;

    let pipeline: Pipeline = (&args).into();
    let processed = pipeline.execute(results);

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

    emitter.emit(processed);
}
