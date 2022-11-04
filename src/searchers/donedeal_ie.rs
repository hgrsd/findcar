#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

use crate::{
    hit::{Hit, Price},
    searcher::{SearchResult, Searcher},
    target::Target,
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
    from: String,
    to: String,
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

fn request_from_target(target: &Target, from: u32) -> DonedealRequestBody {
    DonedealRequestBody {
        makeModelFilters: vec![MakeModelFilter {
            make: target.make.clone().unwrap_or("".to_string()),
            model: target.model.clone().unwrap_or("".to_string()),
        }],
        paging: Paging { from, pageSize: 40 },
        filters: vec![],
        ranges: vec![],
        sections: vec!["cars".to_string()],
    }
}

#[async_recursion::async_recursion]
async fn recurse(
    client: &reqwest::Client,
    target: &Target,
    n: u32,
    mut collected: Vec<DonedealAd>,
) -> Result<Vec<DonedealAd>, Error> {
    let req = request_from_target(target, n);
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
        Ok(recurse(client, target, res.paging.nextFrom, collected).await?)
    } else {
        Ok(collected)
    }
}

#[async_trait::async_trait]
impl Searcher for DoneDealIE {
    async fn search(&self, target: &Target) -> SearchResult {
        let client = reqwest::Client::new();
        let ads = recurse(&client, target, 0, vec![]).await?;
        Ok(ads.iter().map(Hit::from).collect())
    }
}

impl From<&DonedealAd> for Option<Price> {
    fn from(ad: &DonedealAd) -> Self {
        if ad.price.is_none() {
            return None;
        }

        let unwrapped = ad.price.as_ref().unwrap();
        let parsed: i32 = unwrapped.replace(",", "").parse().unwrap();
        Some(match ad.currency.as_str() {
            "EUR" => Price::EUR(parsed, 0),
            "GBP" => Price::GBP(parsed, 0),
            "USD" => Price::USD(parsed, 0),
            _ => unreachable!(),
        })
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

        Hit {
            search_engine: "donedeal.ie".to_string(),
            make: make.to_string(),
            model: model.to_string(),
            price: ad.into(),
            url: ad.friendlyUrl.to_string(),
        }
    }
}
