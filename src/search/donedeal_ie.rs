#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

use super::{SearchResult, Searcher};
use crate::{
    hit::{Hit, Mileage, Price},
    query::Query,
};

const API_ROOT: &str = "https://www.donedeal.ie/ddapi/v1/search";

#[derive(Deserialize)]
struct DisplayAttribute {
    name: String,
    value: String,
}

#[derive(Deserialize)]
struct DonedealAd {
    currency: String,
    price: Option<String>,
    displayAttributes: Vec<DisplayAttribute>,
    friendlyUrl: String,
}

#[derive(Deserialize)]
struct PagingInformation {
    nextFrom: u32,
}

#[derive(Deserialize)]
struct DonedealResponse {
    ads: Option<Vec<DonedealAd>>,
    paging: PagingInformation,
}

#[derive(Serialize)]
struct MakeModelFilter {
    make: String,
    model: String,
}

#[derive(Serialize)]
struct Paging {
    from: u32,
    pageSize: u32,
}

#[derive(Serialize)]
struct Filter {
    name: String,
    values: Vec<String>,
}

#[derive(Serialize)]
struct Range {
    name: String,
    from: Option<String>,
    to: Option<String>,
}

#[derive(Serialize)]
struct DonedealRequestBody {
    makeModelFilters: Vec<MakeModelFilter>,
    paging: Paging,
    filters: Vec<Filter>,
    ranges: Vec<Range>,
    sections: Vec<String>,
}

pub struct DoneDealIE {}

fn ranges_from_query(query: &Query) -> Vec<Range> {
    let mut ranges = vec![];

    if query.min_year.is_some() || query.max_year.is_some() {
        ranges.push(Range {
            name: "year".to_string(),
            from: query.min_year.clone(),
            to: query.max_year.clone(),
        })
    }

    if query.min_kms.is_some() || query.max_kms.is_some() {
        ranges.push(Range {
            name: "mileage".to_string(),
            from: query.min_kms.clone(),
            to: query.max_kms.clone(),
        })
    }

    if query.min_price.is_some() || query.max_price.is_some() {
        ranges.push(Range {
            name: "price".to_string(),
            from: query.min_price.clone(),
            to: query.max_price.clone(),
        })
    }

    ranges
}

fn request_from_query(query: &Query, from: u32) -> DonedealRequestBody {
    DonedealRequestBody {
        makeModelFilters: vec![MakeModelFilter {
            make: query.make.clone().unwrap_or("".to_string()),
            model: query.model.clone().unwrap_or("".to_string()),
        }],
        paging: Paging { from, pageSize: 40 },
        filters: vec![],
        ranges: ranges_from_query(query),
        sections: vec!["cars".to_string()],
    }
}

#[async_recursion::async_recursion]
async fn recursive_fetch(
    client: &reqwest::Client,
    query: &Query,
    n: u32,
    mut collected: Vec<DonedealAd>,
) -> Result<Vec<DonedealAd>, Error> {
    let req = request_from_query(query, n);
    let res = client
        .post(API_ROOT)
        .json(&req)
        .send()
        .await
        .map_err(|error| Error::new(ErrorKind::Other, error))?
        .json::<DonedealResponse>()
        .await
        .map_err(|error| Error::new(ErrorKind::Other, error))?;
    let mut current_ads = res.ads.unwrap_or(vec![]);
    collected.append(&mut current_ads);
    if res.paging.nextFrom > 0 {
        Ok(recursive_fetch(client, query, res.paging.nextFrom, collected).await?)
    } else {
        Ok(collected)
    }
}

#[async_trait::async_trait]
impl Searcher for DoneDealIE {
    async fn search(&self, query: &Query) -> SearchResult {
        let client = reqwest::Client::new();
        let ads = recursive_fetch(&client, query, 0, vec![]).await?;
        Ok(ads.iter().map(Hit::from).collect())
    }
}

impl From<&DonedealAd> for Price {
    fn from(ad: &DonedealAd) -> Self {
        if ad.price.is_none() {
            return Price::Unknown;
        }

        let unwrapped = ad.price.as_ref().unwrap();
        let parsed: i32 = unwrapped.replace(",", "").parse().unwrap();
        match ad.currency.to_uppercase().as_str() {
            "EUR" => Price::Eur(parsed),
            "GBP" => Price::Gbp(parsed),
            "USD" => Price::Usd(parsed),
            _ => Price::Unknown,
        }
    }
}

impl From<&DonedealAd> for Mileage {
    fn from(ad: &DonedealAd) -> Self {
        let mileage = ad
            .displayAttributes
            .iter()
            .find(|attr| attr.name == "mileage");
        match mileage {
            None => Mileage::Unknown,
            Some(m) => {
                let cleaned = m.value.replace(',', "");
                let split: Vec<&str> = cleaned.split_whitespace().collect();

                if split.len() != 2 {
                    return Mileage::Unknown;
                }

                let number: i32 = split[0].parse().unwrap_or(0);
                match split[1] {
                    "km" => Mileage::Km(number),
                    "mi" => Mileage::Mi(number),
                    _ => Mileage::Unknown,
                }
            }
        }
    }
}

impl From<&DonedealAd> for Hit {
    fn from(ad: &DonedealAd) -> Self {
        let make = ad
            .displayAttributes
            .iter()
            .find(|attr| attr.name == "make")
            .map_or("N/A", |make| &make.value);

        let model = ad
            .displayAttributes
            .iter()
            .find(|attr| attr.name == "model")
            .map_or("N/A", |model| &model.value);

        let year = ad
            .displayAttributes
            .iter()
            .find(|attr| attr.name == "year")
            .map_or(0, |year| year.value.parse().unwrap_or(0));

        Hit {
            mileage: ad.into(),
            year,
            search_engine: "donedeal.ie".to_string(),
            make: make.to_string(),
            model: model.to_string(),
            price: ad.into(),
            url: ad.friendlyUrl.to_string(),
        }
    }
}
