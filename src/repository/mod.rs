use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use once_cell::sync::OnceCell;

static MONGODB: OnceCell<Database> = OnceCell::new();

async fn initialize() {
    if MONGODB.get().is_some() {
        return;
    }

    if let Ok(token) = std::env::var("MONGO_URL") {
        if let Ok(client_options) = ClientOptions::parse(token.as_str()).await {
            if let Ok(client) = Client::with_options(client_options) {
                let _ = MONGODB.set(client.default_database().unwrap());
            }
        }
    }
}

pub async fn database() -> &'static Database {
    initialize().await;
    MONGODB.get().unwrap()
}