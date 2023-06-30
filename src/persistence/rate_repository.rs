use std::fmt;
use std::fmt::Formatter;

use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::Collection;
use mongodb::options::{FindOneOptions, FindOptions};
use serde::{Deserialize, Serialize};

use crate::domain::domain::{Currency, Source};

#[derive(Clone, Debug)]
pub struct RateRepository {
    pub col: Collection<Rate>,
}

impl RateRepository {
    pub async fn last_rate(&self) -> Result<Rate, Error> {
        let find_options = FindOneOptions::builder()
            .sort(doc! { "id" : -1, "date_of_rate" : -1, "created_at" : -1})
            .build();

        let rate = self
            .col
            .find_one(None, find_options)
            .await
            .ok()
            .expect("Error getting last rate");
        Ok(rate.unwrap())
    }

    pub async fn list(&self, page: i64, page_size: i64) -> Vec<Rate> {
        let find_options = FindOptions::builder()
            .limit(page_size)
            .skip(u64::try_from((page - 1) * page_size).unwrap())
            .sort(doc! { "id" : -1, "date_of_rate" : -1, "created_at" : -1})
            .build();

        let cursor = match self.col.find(None, find_options).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };
        cursor.try_collect().await.unwrap_or_else(|_| vec![])
    }
}


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Rate {
    pub _id: u64,
    pub rate: bson::Decimal128,
    pub from_currency: Currency,
    pub to_currency: Currency,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date_of_rate: DateTime<Utc>,
    pub source: Source,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    //created_at: bson::DateTime,
    pub description: Option<String>,
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let description = self.description.as_ref().map_or("", |n| n);
        write!(f, "Rate {} {} {} {} {} {} {}", self._id, self.rate, self.from_currency.to_string(), self.date_of_rate.to_string(),
               self.source.to_string(), self.created_at.to_string(), description)
    }
}

