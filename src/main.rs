use mongodb::{Client, options::ClientOptions};
use mongodb::{bson::doc, options::FindOptions};
use futures::stream::{StreamExt, TryStreamExt};


// fn get_document(client: &Client, db_name: &str, coll_name: &str) {
//     let db = client.database(db_name);
//     let coll = db.collection(coll_name);

//     let filter = doc! {"name": "John"};

//     let result = coll.find_one(Some(filter), None).await.unwrap();
//     match result {
//         Some(doc) => println!("{}", doc),
//         None => println!("No document found"),
//     }
// }


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let client_options = ClientOptions::parse("mongodb+srv://userReadOnly:7ZT817O8ejDfhnBM@minichallenge.q4nve1r.mongodb.net/").await.unwrap();
    let mongodb_client = Client::with_options(client_options).unwrap();

    // Get Database name
    let mongodb_database_name = &mongodb_client.list_database_names(None, None).await.unwrap()[0];

    // Get Collection handle
    let mongodb_db_handle = mongodb_client.database(mongodb_database_name);
    let hotels_collection_handle = mongodb_db_handle.collection::<bson::Document>("hotels");
    let flights_collection_handle = mongodb_db_handle.collection::<bson::Document>("flights");

    let mut hotel_cursor = hotels_collection_handle.find(None,None).await.unwrap();
    while let data = hotel_cursor.next().await {
        // println!("{}", hotel);
        data.unwrap().whatisthis();
    }
    




    // Query database sample
    //let filter = doc! {""}

    Ok(())
}
