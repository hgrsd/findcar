use futures::stream::{FuturesUnordered, StreamExt};

use crate::hit::Hit;
use crate::search::{SearchResult, Searcher};
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
            match result {
                Ok(ref mut inner) => {
                    successes.append(inner);
                }
                Err(error) => {
                    println!("An error was encountered in a search engine, results unavailable.\nReason: {}", error);
                }
            }
        }

        successes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    use crate::{
        hit::{Mileage, Price},
        target::Target,
    };
    use std::io::{Error, ErrorKind};

    #[tokio::test]
    async fn single_searcher() {
        struct S {}
        #[async_trait]
        impl Searcher for S {
            async fn search(&self, _target: &Target) -> SearchResult {
                Ok(vec![
                    Hit {
                        mileage: Mileage::Km(10000),
                        year: 2022,
                        search_engine: "bla".to_string(),
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::Eur(19995),
                        url: "https://mycar.com/car".to_string(),
                    },
                    Hit {
                        mileage: Mileage::Km(10000),
                        year: 2022,
                        search_engine: "bla".to_string(),
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::Eur(19995),
                        url: "https://mycar.com/car".to_string(),
                    },
                ])
            }
        }

        let searchers: Vec<Box<dyn Searcher>> = vec![Box::new(S {})];
        let engine = Engine::with_searchers(searchers);

        let target = Target::new();
        let results = engine.search(&target).await;
        assert_eq!(
            results,
            vec!(
                Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::Eur(19995),
                    url: "https://mycar.com/car".to_string(),
                },
                Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::Eur(19995),
                    url: "https://mycar.com/car".to_string(),
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
                Ok(vec![
                    Hit {
                        mileage: Mileage::Km(10000),
                        year: 2022,
                        search_engine: "bla".to_string(),
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::Eur(19995),
                        url: "https://mycar.com/car".to_string(),
                    },
                    Hit {
                        mileage: Mileage::Km(10000),
                        year: 2022,
                        search_engine: "bla".to_string(),
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::Eur(19995),
                        url: "https://mycar.com/car".to_string(),
                    },
                ])
            }
        }

        #[async_trait]
        impl Searcher for S1 {
            async fn search(&self, _target: &Target) -> SearchResult {
                Ok(vec![Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Volkswagen".to_string(),
                    model: "Golf".to_string(),
                    price: Price::Eur(25000),
                    url: "https://mycar.com/car".to_string(),
                }])
            }
        }

        let searchers: Vec<Box<dyn Searcher>> = vec![Box::new(S0 {}), Box::new(S1 {})];
        let engine = Engine::with_searchers(searchers);

        let target = Target::new();
        let results = engine.search(&target).await;
        assert_eq!(
            results,
            vec!(
                Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::Eur(19995),
                    url: "https://mycar.com/car".to_string(),
                },
                Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::Eur(19995),
                    url: "https://mycar.com/car".to_string(),
                },
                Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Volkswagen".to_string(),
                    model: "Golf".to_string(),
                    price: Price::Eur(25000),
                    url: "https://mycar.com/car".to_string(),
                },
            )
        );
    }

    #[tokio::test]
    async fn errors_in_one_searcher_do_not_crash_engine() {
        struct S0 {}
        struct S1 {}

        #[async_trait]
        impl Searcher for S0 {
            async fn search(&self, _target: &Target) -> SearchResult {
                Ok(vec![
                    Hit {
                        mileage: Mileage::Km(10000),
                        year: 2022,
                        search_engine: "bla".to_string(),
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::Eur(19995),
                        url: "https://mycar.com/car".to_string(),
                    },
                    Hit {
                        mileage: Mileage::Km(10000),
                        year: 2022,
                        search_engine: "bla".to_string(),
                        make: "Skoda".to_string(),
                        model: "Fabia".to_string(),
                        price: Price::Eur(19995),
                        url: "https://mycar.com/car".to_string(),
                    },
                ])
            }
        }

        #[async_trait]
        impl Searcher for S1 {
            async fn search(&self, _target: &Target) -> SearchResult {
                Err(Error::new(ErrorKind::Other, "oh no"))
            }
        }

        let searchers: Vec<Box<dyn Searcher>> = vec![Box::new(S0 {}), Box::new(S1 {})];
        let engine = Engine::with_searchers(searchers);

        let target = Target::new();
        let results = engine.search(&target).await;
        assert_eq!(
            results,
            vec!(
                Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::Eur(19995),
                    url: "https://mycar.com/car".to_string(),
                },
                Hit {
                    mileage: Mileage::Km(10000),
                    year: 2022,
                    search_engine: "bla".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    price: Price::Eur(19995),
                    url: "https://mycar.com/car".to_string(),
                },
            )
        );
    }
}
