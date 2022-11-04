use clap::Parser;

mod args;
mod engine;
mod hit;
mod searcher;
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

    println!("search target: \n{:?}", target);
}
