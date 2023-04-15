use std::cmp::Reverse;

use crate::hit::Hit;

use super::Action;

/// A sorting action, which sorts a given set of Hits by the provided SortBy using the provided
/// SortOrder.
pub struct Sort {
    by: SortBy,
    order: SortOrder,
}

pub enum SortBy {
    Price,
    Year,
    Mileage,
}

pub enum SortOrder {
    Asc,
    Desc,
}

impl Sort {
    pub fn new(by: SortBy, order: SortOrder) -> Self {
        Self { by, order }
    }
}

impl Action for Sort {
    fn execute(&self, mut hits: Vec<Hit>) -> Vec<Hit> {
        match (&self.by, &self.order) {
            (SortBy::Price, SortOrder::Asc) => {
                hits.sort_by_key(|x| x.price.clone());
            }
            (SortBy::Price, SortOrder::Desc) => {
                hits.sort_by_key(|x| Reverse(x.price.clone()));
            }
            (SortBy::Year, SortOrder::Asc) => {
                hits.sort_by_key(|x| x.year);
            }
            (SortBy::Year, SortOrder::Desc) => {
                hits.sort_by_key(|x| Reverse(x.year));
            }
            (SortBy::Mileage, SortOrder::Asc) => {
                hits.sort_by_key(|x| x.mileage.clone());
            }
            (SortBy::Mileage, SortOrder::Desc) => {
                hits.sort_by_key(|x| Reverse(x.mileage.clone()));
            }
        };
        hits
    }
}

#[cfg(test)]
mod tests {
    use crate::hit::{Mileage, Price};

    use super::*;

    #[test]
    fn sort_price_asc() {
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

        let sorter = Sort::new(SortBy::Price, SortOrder::Asc);
        let result = sorter.execute(hits);

        assert_eq!(
            result,
            vec![
                Hit {
                    search_engine: "foo".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    mileage: Mileage::Km(1001),
                    year: 2022,
                    price: Price::Eur(21),
                    url: "bla".to_string(),
                },
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
            ],
        );
    }

    #[test]
    fn sort_price_desc() {
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

        let sorter = Sort::new(SortBy::Price, SortOrder::Desc);
        let result = sorter.execute(hits);

        assert_eq!(
            result,
            vec![
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
                    mileage: Mileage::Km(100),
                    year: 2001,
                    price: Price::Eur(100),
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
            ],
        );
    }

    #[test]
    fn sort_year_asc() {
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

        let sorter = Sort::new(SortBy::Year, SortOrder::Asc);
        let result = sorter.execute(hits);

        assert_eq!(
            result,
            vec![
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
                    mileage: Mileage::Km(100),
                    year: 2001,
                    price: Price::Eur(100),
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
            ],
        );
    }

    #[test]
    fn sort_year_desc() {
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

        let sorter = Sort::new(SortBy::Year, SortOrder::Desc);
        let result = sorter.execute(hits);

        assert_eq!(
            result,
            vec![
                Hit {
                    search_engine: "foo".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    mileage: Mileage::Km(1001),
                    year: 2022,
                    price: Price::Eur(21),
                    url: "bla".to_string(),
                },
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
            ],
        );
    }

    #[test]
    fn sort_mileage_asc() {
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

        let sorter = Sort::new(SortBy::Mileage, SortOrder::Asc);
        let result = sorter.execute(hits);

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
            ],
        );
    }

    #[test]
    fn sort_mileage_desc() {
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

        let sorter = Sort::new(SortBy::Mileage, SortOrder::Desc);
        let result = sorter.execute(hits);

        assert_eq!(
            result,
            vec![
                Hit {
                    search_engine: "foo".to_string(),
                    make: "Skoda".to_string(),
                    model: "Fabia".to_string(),
                    mileage: Mileage::Km(1001),
                    year: 2022,
                    price: Price::Eur(21),
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
                    mileage: Mileage::Km(100),
                    year: 2001,
                    price: Price::Eur(100),
                    url: "bla".to_string(),
                },
            ],
        );
    }
}
