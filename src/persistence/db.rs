use std::fmt::Error;

use mongodb::{Client, Collection, Database};
use mongodb::options::ClientOptions;
use once_cell::sync::OnceCell;

use crate::persistence::rate_repository::{Rate, RateRepository};

static MONGODB: OnceCell<Database> = OnceCell::new();

async fn initialize() {
    if MONGODB.get().is_some() {
        let now = chrono::offset::Local::now();

        println!("✅ Database is already initialized {}", now.to_string());
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

pub async fn print_collection_names() {
    let db = database().await;
    for collection in db.list_collection_names(None).await.expect("error list_collection_names") {
        leptos::log!("{}", collection);
    }
}

#[derive(Clone, Debug)]
pub struct DB {
    pub rates: RateRepository,
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        let rates_coll: Collection<Rate> = collection("rates").await;

        let rate_repository = RateRepository { col: rates_coll };

        Ok(Self {
            rates: rate_repository,
        })
    }
}
