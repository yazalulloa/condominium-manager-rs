#[cfg(test)]
pub mod mongo_tests {
    use std::error::Error;

    use dotenv::dotenv;
    use mongodb::bson::doc;
    use mongodb::options::FindOptions;
    use wasm_bindgen::UnwrapThrowExt;

    use leptos_start::repository::database;

    #[tokio::test]
    pub async fn list_collection_names() {
        dotenv().ok();
        let db = database().await;
        for collection in db.list_collection_names(None).await.expect_throw("list_database_names") {
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
}