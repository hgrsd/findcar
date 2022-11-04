use futures::stream::{FuturesUnordered, StreamExt};

use crate::hit::Hit;
use crate::searcher::{SearchResult, Searcher};
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

        let results: Vec<SearchResult> = futures.collect().await;
        let mut successes: Vec<Hit> = vec![];
        for mut result in results {
            if let Ok(ref mut inner) = result {
                successes.append(inner);
            }
        }

        successes
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
            async fn search(&self, _target: &Target) -> SearchResult {
                Ok(vec![
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
                ])
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

    #[tokio::test]
    async fn multiple_searchers_combine_results() {
        struct S0 {}
        struct S1 {}

        #[async_trait]
        impl Searcher for S0 {
            async fn search(&self, _target: &Target) -> SearchResult {
                Ok(
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
                )
            }
        }

        #[async_trait]
        impl Searcher for S1 {
            async fn search(&self, _target: &Target) -> SearchResult {
                Ok(
                vec![
                    Hit {
                        make: "Volkswagen".to_string(),
                        model: "Golf".to_string(),
                        price: Price::EUR(25000, 99),
                    },
                ]
                )
            }
        }

        let searchers: Vec<Box<dyn Searcher>> = vec![Box::new(S0 {}), Box::new(S1 {})];
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
                Hit {
                    make: "Volkswagen".to_string(),
                    model: "Golf".to_string(),
                    price: Price::EUR(25000, 99),
                },
            )
        );
    }
}
