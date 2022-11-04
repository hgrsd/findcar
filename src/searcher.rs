use crate::hit::Hit;
use crate::target::Target;

#[async_trait::async_trait]
pub trait Searcher {
    async fn search(&self, target: &Target) -> Vec<Hit>;
}
