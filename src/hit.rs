#[derive(Debug, Eq, PartialEq)]
pub enum Price {
    EUR(i32, i16),
    USD(i32, i16),
    GBP(i32, i16),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Hit {
    pub make: String,
    pub model: String,
    pub price: Price,
}
