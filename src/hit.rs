use serde::Serialize;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Serialize)]
pub enum Price {
    Eur(i32),
    Usd(i32),
    Gbp(i32),
    Unknown,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Serialize)]
pub enum Mileage {
    Km(i32),
    Mi(i32),
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct Hit {
    pub search_engine: String,
    pub make: String,
    pub model: String,
    pub mileage: Mileage,
    pub year: u16,
    pub price: Price,
    pub url: String,
}
