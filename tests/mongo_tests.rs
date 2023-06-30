#[cfg(test)]
pub mod mongo_tests {
    use std::error::Error;
    use std::str::FromStr;
    use std::sync::Once;

    use dotenv::dotenv;
    use mongodb::bson::{doc, Decimal128};
    use mongodb::options::FindOptions;

    use condominum_manager_rs::persistence::db::{database, DB};
    use condominum_manager_rs::persistence::rate_repository::Rate;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            println!("calling initialize");
            let database = database();
        });
    }

    #[tokio::test]
    pub async fn list_collection_names() {
        dotenv().ok();
        let db = database().await;
        for collection in db
            .list_collection_names(None)
            .await
            .expect("error list_collection_names")
        {
            println!("{}", collection);
        }
    }

    #[tokio::test]
    pub async fn last_rate() -> Result<(), Box<dyn Error>> {
        dotenv().ok();
        let db = database().await;

        let collection = db.collection::<Rate>("rates");

        let find_options = FindOptions::builder()
            .sort(doc! { "id" : -1, "date_of_rate" : -1, "created_at" : -1})
            .build();

        let cursor = collection.find(None, find_options).await.unwrap();
        let rate = cursor.deserialize_current().unwrap();
        println!("{}", rate);
        Ok(())
    }

    #[tokio::test]
    pub async fn new_last_rate() {
        dotenv().ok();
        let rate_repo = DB::init().await.unwrap().rates;
        let rate = rate_repo.last_rate().await.unwrap();
        println!("{}", rate)
    }

    #[tokio::test]
    pub async fn list() {
        dotenv().ok();
        let rate_repo = DB::init().await.unwrap().rates;
        let list = rate_repo.list(3, 10).await;
        let size = list.len();
        for rate in list {
            println!("RATE {}", rate)
        }
        println!("LIST SIZE {}", size)
    }

    #[tokio::test]
    pub async fn sum_rates() {
        dotenv().ok();
        let rate_repo = DB::init().await.unwrap().rates;
        let list = rate_repo.list(1, 10).await;
        let size = list.len();
        for rate in list {
            let x = rate.rate.to_string();
            let x1 = x.parse::<f64>().unwrap();
            println!("RATE {}", rate);
            println!("rate f64 {}", x1);

            let decimal128 = Decimal128::from_str(&*x1.to_string()).expect("TODO: panic message");
            println!("rate Decimal128 {}", decimal128);
        }
        println!("LIST SIZE {}", size)
    }
}
