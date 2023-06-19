use std::fmt::Error;
use mongodb::{Client, Collection, Database};
use mongodb::options::ClientOptions;

use once_cell::sync::OnceCell;

use crate::rate_repository::RateRepository;
use crate::rates::Rate;


static MONGODB: OnceCell<Database> = OnceCell::new();

async fn initialize() {
    if MONGODB.get().is_some() {
        println!("✅ Database is already initialized");
        return;
    }

    if let Ok(token) = std::env::var("MONGO_URL") {
        if let Ok(client_options) = ClientOptions::parse(token.as_str()).await {
            if let Ok(client) = Client::with_options(client_options) {
                let _ = MONGODB.set(client.default_database().unwrap());
                println!("✅ Database connected successfully");
            }
        }
    }

}

pub async fn database() -> &'static Database {
    initialize().await;
    MONGODB.get().expect("database not initialized")
}

pub async fn collection<T>(col: &str) -> Collection<T> {
    database().await.collection(col)
}

#[derive(Clone, Debug)]
pub struct DB {
    pub rates: RateRepository,
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        let rates_coll: Collection<Rate> = collection("rates").await;

        let rate_repository = RateRepository {
            col: rates_coll,
        };

        Ok(Self {
            rates: rate_repository
        })
    }
}