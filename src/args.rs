use clap::Parser;

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
