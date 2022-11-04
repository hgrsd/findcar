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

    let searchers: Vec<Box<dyn search::Searcher>> = match args.search_engine {
        None => {
            vec![
                Box::new(search::CarZoneIE {}),
                Box::new(search::DoneDealIE {}),
            ]
        }
        Some(ref engines) => {
            let mut vec: Vec<Box<dyn search::Searcher>> = vec![];

            if engines.contains(&"carzone_ie".to_string()) {
                vec.push(Box::new(search::CarZoneIE {}));
            }

            if engines.contains(&"donedeal_ie".to_string()) {
                vec.push(Box::new(search::DoneDealIE {}));
            }
            vec
        }
    };

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
