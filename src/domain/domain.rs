
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
pub enum Currency {
    USD,
    VED,
}


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, EnumString, Display)]
pub enum Source {
    BCV,
    PLATFORM,
}