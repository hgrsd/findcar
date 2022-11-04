#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Price {
    EUR(i32, i32),
    USD(i32, i32),
    GBP(i32, i32),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hit {
    pub search_engine: String,
    pub make: String,
    pub model: String,
    pub price: Option<Price>,
    pub url: String,
}
