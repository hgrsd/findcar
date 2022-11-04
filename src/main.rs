use clap::Parser;

mod args;
mod engine;
mod hit;
mod searcher;
mod searchers;
mod target;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    let mut target_builder = target::TargetBuilder::new();

    if let Some(make) = args.make {
        target_builder.make(&make);
    }

    if let Some(model) = args.model {
        target_builder.model(&model);
    }

    let target = target_builder.build();
    let searchers: Vec<Box<dyn searcher::Searcher>> =
        vec![Box::new(searchers::donedeal_ie::DoneDealIE {})];

    let engine = engine::Engine::with_searchers(searchers);

    let results = engine.search(&target).await;

    for result in results {
        println!("{:?}", result);
    }
}
