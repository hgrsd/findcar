use crate::{hit::Hit, query::Query};

mod donedeal_ie;
pub use donedeal_ie::DoneDealIE;

mod carzone_ie;
pub use carzone_ie::CarZoneIE;

pub type SearchResult = Result<Vec<Hit>, std::io::Error>;

/// A trait that defines a single operation that any search engine must implement.
/// The root engine will be able to use the results of any struct that implements this trait.
#[async_trait::async_trait]
pub trait Searcher {
    async fn search(&self, query: &Query) -> SearchResult;
}
