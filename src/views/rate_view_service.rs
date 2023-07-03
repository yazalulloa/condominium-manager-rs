use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

use crate::domain::domain::{Currency, Source};

cfg_if! {
    if #[cfg(feature = "ssr")] {

use crate::persistence::db::DB;
use crate::persistence::rate_repository::Rate;

pub async fn get_view_item_rate(page: i64, page_size: i64) -> Vec<ViewItemRate> {
            println!("get_view_item_rate");
    let repo = DB::init().await.unwrap().rates;
    repo.list(page, page_size).await.into_iter().map(|rate| to_view_item(rate)).collect::<Vec<ViewItemRate>>()
}

fn to_view_item(rate: Rate) -> ViewItemRate {
    ViewItemRate {
        _id: rate._id,
        rate: rate.rate.to_string().parse::<f64>().unwrap(),
        from_currency: rate.from_currency,
        to_currency: rate.to_currency,
        date_of_rate: rate.date_of_rate.to_string(),
        source: rate.source,
        created_at: rate.created_at.to_string(),
    }
}

    }

}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ViewItemRate {
    pub _id: u64,
    pub rate: f64,
    pub from_currency: Currency,
    pub to_currency: Currency,
    pub date_of_rate: String,
    pub source: Source,
    pub created_at: String,
}

