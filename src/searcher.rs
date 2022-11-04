use crate::hit::Hit;
use crate::target::Target;

pub type SearchResult = Result<Vec<Hit>, std::io::Error>;

#[async_trait::async_trait]
pub trait Searcher {
    async fn search(&self, target: &Target) -> SearchResult;
}
