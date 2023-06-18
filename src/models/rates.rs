use std::fmt;
use std::fmt::Formatter;

use mongodb::bson;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rate {
    _id: u64,
    rate: bson::Decimal128,
    from_currency: Currency,
    to_currency: Currency,
    date_of_rate: bson::DateTime,
    source: Source,
    created_at: bson::DateTime,
    description: Option<String>,
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let description = self.description.as_ref().map_or("", |n| n);
        write!(f, "Rate {} {} {} {} {} {} {}", self._id, self.rate, self.from_currency.to_string(), self.date_of_rate.to_string(),
               self.source.to_string(), self.created_at.to_string(), description)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, EnumString, Display)]
enum Currency {
    USD,
    VED,
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, EnumString, Display)]
enum Source {
    BCV,
    PLATFORM,
}