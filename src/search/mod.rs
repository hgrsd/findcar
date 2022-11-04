use crate::{hit::Hit, query::Query};

mod donedeal_ie;
pub use donedeal_ie::DoneDealIE;
pub type SearchResult = Result<Vec<Hit>, std::io::Error>;

#[async_trait::async_trait]
pub trait Searcher {
    async fn search(&self, query: &Query) -> SearchResult;
}
