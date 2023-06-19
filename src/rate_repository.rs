use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::Collection;
use mongodb::options::{FindOneOptions, FindOptions};

use crate::rates::Rate;

#[derive(Clone, Debug)]
pub struct RateRepository {
    pub(crate) col: Collection<Rate>,
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