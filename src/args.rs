use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub make: Option<String>,

    #[arg(long)]
    pub model: Option<String>,

    #[arg(long)]
    pub min_year: Option<String>,

    #[arg(long)]
    pub max_year: Option<String>,

    #[arg(long)]
    pub min_kms: Option<String>,

    #[arg(long)]
    pub max_kms: Option<String>,

    #[arg(long)]
    pub min_price: Option<String>,

    #[arg(long)]
    pub max_price: Option<String>,

    #[arg(long)]
    pub sort_by: Option<String>,

    #[arg(long)]
    pub sort_order: Option<String>,

    #[arg(long)]
    pub limit: Option<usize>,

    #[arg(long)]
    pub emitter: Option<String>,
}
