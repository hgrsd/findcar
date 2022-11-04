use crate::hit::{Hit, Mileage, Price};

use super::Emit;

pub struct TextEmitter {}

impl std::fmt::Display for Mileage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mileage::Unknown => write!(f, "unknown"),
            Mileage::Km(kms) => write!(f, "{} km", kms),
            Mileage::Mi(mis) => write!(f, "{} mi", mis),
        }
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Price::Unknown => write!(f, "unknown"),
            Price::Eur(eur) => write!(f, "€{:>7}", eur),
            Price::Usd(usd) => write!(f, "${:>7}", usd),
            Price::Gbp(gbp) => write!(f, "£{:>7}", gbp),
        }
    }
}

impl std::fmt::Display for Hit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - ({}, {}) {} {} [{}] @ {}",
            self.price,
            self.year.to_string(),
            self.mileage,
            self.make,
            self.model,
            self.search_engine,
            self.url,
        )
    }
}

impl TextEmitter {
    pub fn new() -> Self {
        TextEmitter {}
    }
}

impl Emit for TextEmitter {
    fn emit(&self, hits: Vec<Hit>) {
        for hit in hits {
            println!("{}", hit);
        }
    }
}
