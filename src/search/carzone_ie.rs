#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

use super::{SearchResult, Searcher};
use crate::{
    hit::{Hit, Mileage, Price},
    query::Query,
};

const API_ROOT: &str = "https://www.carzone.ie/rest/1.0/Car/stock";

#[derive(Serialize)]
struct CarzoneQueryParams {
    make: Option<String>,
    model: Option<String>,
    minPrice: Option<String>,
    maxPrice: Option<String>,
    minYear: Option<String>,
    maxYear: Option<String>,
    minMileage: Option<String>,
    maxMileage: Option<String>,
    showPoa: String,
    page: String,
    size: String,
}

#[derive(Deserialize)]
struct PriceDetail {
    gbpPrice: Option<i32>,
    euroPrice: Option<i32>,
}

#[derive(Deserialize)]
struct VehicleMileage {
    mileageKm: i32,
}

#[derive(Deserialize)]
struct Vehicle {
    mileage: VehicleMileage,
    registrationYear: u16,
}

#[derive(Deserialize)]
struct MakeModel {
    cleanMake: String,
    cleanModel: String,
}

#[derive(Deserialize)]
struct SearchDetailSummary {
    mmv: MakeModel,
}

#[derive(Deserialize)]
struct Summary {
    publicReference: String,
    priceDetail: PriceDetail,
    vehicle: Vehicle,
    searchDetailSummary: SearchDetailSummary,
}

#[derive(Deserialize)]
struct CarzoneAd {
    summary: Summary,
}

#[derive(Deserialize)]
struct CarzoneResult {
    items: Vec<CarzoneAd>,
}

#[derive(Deserialize)]
struct CarzoneResponse {
    totalPages: u16,
    results: Vec<CarzoneResult>,
}

pub struct CarZoneIE {}

fn params_from_query(query: &Query, page: u16) -> CarzoneQueryParams {
    CarzoneQueryParams {
        make: query.make.clone(),
        model: query.model.clone(),
        minPrice: query.min_price.clone(),
        maxPrice: query.max_price.clone(),
        minYear: query.min_year.clone(),
        maxYear: query.max_year.clone(),
        minMileage: query.min_kms.clone(),
        maxMileage: query.max_kms.clone(),
        showPoa: "false".to_string(),
        page: page.to_string(),
        size: "30".to_string(),
    }
}

#[async_recursion::async_recursion]
async fn recursive_fetch(
    client: &reqwest::Client,
    query: &Query,
    page: u16,
    mut collected: Vec<CarzoneAd>,
) -> Result<Vec<CarzoneAd>, Error> {
    let params = params_from_query(query, page);
    let res = client
        .get(API_ROOT)
        .query(&params)
        .send()
        .await
        .map_err(|error| Error::new(ErrorKind::Other, error))?
        .json::<CarzoneResponse>()
        .await
        .map_err(|error| Error::new(ErrorKind::Other, error))?;
    let mut current_ads = res.results.into_iter().flat_map(|r| r.items).collect();
    collected.append(&mut current_ads);
    if page < res.totalPages {
        Ok(recursive_fetch(client, query, page + 1, collected).await?)
    } else {
        Ok(collected)
    }
}

#[async_trait::async_trait]
impl Searcher for CarZoneIE {
    async fn search(&self, query: &Query) -> SearchResult {
        let client = reqwest::Client::new();
        let ads = recursive_fetch(&client, query, 1, vec![]).await?;
        Ok(ads.iter().map(Hit::from).collect())
    }
}

impl From<&CarzoneAd> for Price {
    fn from(ad: &CarzoneAd) -> Self {
        ad.summary
            .priceDetail
            .euroPrice
            .map(Price::Eur)
            .or_else(|| ad.summary.priceDetail.gbpPrice.map(Price::Gbp))
            .unwrap_or(Price::Unknown)
    }
}

impl From<&CarzoneAd> for Mileage {
    fn from(ad: &CarzoneAd) -> Self {
        Mileage::Km(ad.summary.vehicle.mileage.mileageKm)
    }
}

impl From<&CarzoneAd> for Hit {
    fn from(ad: &CarzoneAd) -> Self {
        let make = ad.summary.searchDetailSummary.mmv.cleanMake.clone();
        let model = ad.summary.searchDetailSummary.mmv.cleanModel.clone();

        let url = format!(
            "https://www.carzone.ie/used-cars/{}/{}/fpa/{}",
            urlencoding::encode(&make),
            urlencoding::encode(&model),
            ad.summary.publicReference
        );

        Hit {
            mileage: ad.into(),
            year: ad.summary.vehicle.registrationYear,
            search_engine: "carzone.ie".to_string(),
            make,
            model,
            price: ad.into(),
            url,
        }
    }
}
