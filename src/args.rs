use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub make: Option<String>,

    #[arg(long)]
    pub model: Option<String>,
}
