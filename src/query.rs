use crate::args::Args;

#[derive(Debug, Clone)]
pub struct Query {
    pub make: Option<String>,
    pub model: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub min_year: Option<String>,
    pub max_year: Option<String>,
    pub min_kms: Option<String>,
    pub max_kms: Option<String>,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            make: None,
            model: None,
            min_price: None,
            max_price: None,
            min_year: None,
            max_year: None,
            min_kms: None,
            max_kms: None,
        }
    }
}

impl From<&Args> for Query {
    fn from(args: &Args) -> Self {
        Self {
            make: args.make.clone(),
            model: args.model.clone(),
            min_price: args.min_price.clone(),
            max_price: args.max_price.clone(),
            min_year: args.min_year.clone(),
            max_year: args.max_year.clone(),
            min_kms: args.min_kms.clone(),
            max_kms: args.max_kms.clone(),
        }
    }
}
