#[derive(Debug, Clone)]
pub enum SortBy {
    Price,
    Mileage,
    Year,
    NoSort,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

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
    pub limit: Option<usize>,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
}

impl Query {
    pub fn new() -> Self {
        Query {
            sort_by: SortBy::NoSort,
            sort_order: SortOrder::Asc,
            make: None,
            model: None,
            min_price: None,
            max_price: None,
            min_year: None,
            max_year: None,
            min_kms: None,
            max_kms: None,
            limit: None,
        }
    }

    pub fn make(&mut self, make: &str) -> &mut Self {
        self.make = Some(make.to_string());
        self
    }

    pub fn model(&mut self, model: &str) -> &mut Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn min_price(&mut self, min_price: &str) -> &mut Self {
        self.min_price = Some(min_price.to_string());
        self
    }

    pub fn max_price(&mut self, max_price: &str) -> &mut Self {
        self.max_price = Some(max_price.to_string());
        self
    }

    pub fn min_year(&mut self, min_year: &str) -> &mut Self {
        self.min_year = Some(min_year.to_string());
        self
    }

    pub fn max_year(&mut self, max_year: &str) -> &mut Self {
        self.max_year = Some(max_year.to_string());
        self
    }

    pub fn min_kms(&mut self, min_kms: &str) -> &mut Self {
        self.min_kms = Some(min_kms.to_string());
        self
    }

    pub fn max_kms(&mut self, max_kms: &str) -> &mut Self {
        self.max_kms = Some(max_kms.to_string());
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn sort_by(&mut self, sort_by: &str) -> &mut Self {
        self.sort_by = match sort_by.to_uppercase().as_str() {
            "PRICE" => SortBy::Price,
            "YEAR" => SortBy::Year,
            "MILEAGE" => SortBy::Mileage,
            _ => {
                println!("Unrecognised sort field: {}, ignoring.", sort_by);
                SortBy::NoSort
            }
        };

        self
    }

    pub fn sort_order(&mut self, sort_order: &str) -> &mut Self {
        self.sort_order = match sort_order.to_uppercase().as_str() {
            "ASC" => SortOrder::Asc,
            "DESC" => SortOrder::Desc,
            _ => {
                println!("Unrecognised sort order value: {}, ignoring.", sort_order);
                SortOrder::Asc
            }
        };

        self
    }
}
