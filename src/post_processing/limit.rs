use crate::{args::Args, hit::Hit};

use super::Action;

/// A limit action, which applies a cutoff to a set of Hits.
pub struct Limit {
    limit: usize,
}

impl Limit {
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }
}

impl Action for Limit {
    fn execute(&self, hits: Vec<Hit>) -> Vec<Hit> {
        hits.into_iter().take(self.limit).collect()
    }
}

impl From<&Args> for Option<Box<Limit>> {
    fn from(args: &Args) -> Self {
        match args.limit {
            Some(lim) => Some(Box::new(Limit::new(lim))),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hit::{Mileage, Price};

    use super::*;

    #[test]
    fn limit_one() {
        let hits: Vec<Hit> = vec![
            Hit {
                search_engine: "foo".to_string(),
                make: "Skoda".to_string(),
                model: "Fabia".to_string(),
                mileage: Mileage::Km(100),
                year: 2001,
                price: Price::Eur(100),
                url: "bla".to_string(),
            },
            Hit {
                search_engine: "foo".to_string(),
                make: "Skoda".to_string(),
                model: "Fabia".to_string(),
                mileage: Mileage::Km(1000),
                year: 1999,
                price: Price::Eur(101),
                url: "bla".to_string(),
            },
            Hit {
                search_engine: "foo".to_string(),
                make: "Skoda".to_string(),
                model: "Fabia".to_string(),
                mileage: Mileage::Km(1001),
                year: 2022,
                price: Price::Eur(21),
                url: "bla".to_string(),
            },
        ];

        let limiter = Limit::new(1);
        let result = limiter.execute(hits);

        assert_eq!(
            result,
            vec![Hit {
                search_engine: "foo".to_string(),
                make: "Skoda".to_string(),
                model: "Fabia".to_string(),
                mileage: Mileage::Km(100),
                year: 2001,
                price: Price::Eur(100),
                url: "bla".to_string(),
            },]
        );
    }

    #[test]
    fn limit_exceeding_length() {
        let hits: Vec<Hit> = vec![
            Hit {
                search_engine: "foo".to_string(),
                make: "Skoda".to_string(),
                model: "Fabia".to_string(),
                mileage: Mileage::Km(100),
                year: 2001,
                price: Price::Eur(100),
                url: "bla".to_string(),
            },
            Hit {
                search_engine: "foo".to_string(),
                make: "Skoda".to_string(),
                model: "Fabia".to_string(),
                mileage: Mileage::Km(1000),
                year: 1999,
                price: Price::Eur(101),
                url: "bla".to_string(),
            },
            Hit {
                search_engine: "foo".to_string(),
                make: "Skoda".to_string(),
                model: "Fabia".to_string(),
                mileage: Mileage::Km(1001),
                year: 2022,
                price: Price::Eur(21),
                url: "bla".to_string(),
            },
        ];

        let limiter = Limit::new(10);
        let result = limiter.execute(hits);

        assert_eq!(
            result,
            vec![
                Hit {
                    search_engine: "foo".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    mileage: Mileage::Km(100),
                    year: 2001,
                    price: Price::Eur(100),
                    url: "bla".to_string(),
                },
                Hit {
                    search_engine: "foo".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    mileage: Mileage::Km(1000),
                    year: 1999,
                    price: Price::Eur(101),
                    url: "bla".to_string(),
                },
                Hit {
                    search_engine: "foo".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    mileage: Mileage::Km(1001),
                    year: 2022,
                    price: Price::Eur(21),
                    url: "bla".to_string(),
                },
            ]
        );
    }
}
