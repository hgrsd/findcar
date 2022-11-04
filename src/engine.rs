use futures::stream::{FuturesUnordered, StreamExt};

use crate::hit::Hit;
use crate::searcher::Searcher;
use crate::target::Target;

pub struct Engine {
    searchers: Vec<Box<dyn Searcher>>,
}

impl Engine {
    pub fn with_searchers(searchers: Vec<Box<dyn Searcher>>) -> Self {
        Engine { searchers }
    }

    pub async fn search(&self, target: &Target) -> Vec<Hit> {
        let futures = FuturesUnordered::new();
        for searcher in &self.searchers {
            futures.push(searcher.search(target));
        }

        let results: Vec<Vec<Hit>> = futures.collect().await;

        results.into_iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;
    use crate::{hit::Price, searcher::Searcher, target::TargetBuilder};

    #[tokio::test]
    async fn single_searcher() {
        struct S {}
        #[async_trait]
        impl Searcher for S {
            async fn search(&self, _target: &Target) -> Vec<Hit> {
                vec![
                    Hit {
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::EUR(19995, 50),
                    },
                    Hit {
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::EUR(19995, 50),
                    },
                ]
            }
        }

        let searchers: Vec<Box<dyn Searcher>> = vec![Box::new(S {})];
        let engine = Engine::with_searchers(searchers);

        let target = TargetBuilder::new().build();
        let results = engine.search(&target).await;
        assert_eq!(
            results,
            vec!(
                Hit {
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::EUR(19995, 50),
                },
                Hit {
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::EUR(19995, 50),
                },
            )
        );
    }
}
