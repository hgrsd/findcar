#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Price {
    Eur(i32),
    Usd(i32),
    Gbp(i32),
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Mileage {
    Km(i32),
    Mi(i32),
    Unknown,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hit {
    pub search_engine: String,
    pub make: String,
    pub model: String,
    pub mileage: Mileage,
    pub year: u16,
    pub price: Option<Price>,
    pub url: String,
}
