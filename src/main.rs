use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::{Client, options::ClientOptions};
use mongodb::{bson::DateTime};
use mongodb::options::{Collation, FindOptions, CollationStrength};
use chrono::DateTime as ChronosDateTime;
use chrono::Utc;
use futures::stream::TryStreamExt;
use serde::{Serialize, Deserialize};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};



#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct HotelQuery {
    checkInDate: String,
    checkOutDate: String,
    destination: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct HoteResponse {
    pub City: String,
    #[serde(rename = "Check In Date")]
    checkInDate: String,
    #[serde(rename = "Check Out Date")]
    checkOutDate: String,
    Hotel: String,
    Price: i32
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct FlightQuery {
    departureDate: String,
    destination: String,
    returnDate: String
}

#[get("/flight")]
async fn flight(flight_collection: web::Data<Collection<Flight>>, query: web::Query<FlightQuery>) -> impl Responder {
    
    println!("{:#?}", query);
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hotel")]
async fn hotel(hotel_collection: web::Data<Collection<Hotel>>,  query: web::Query<HotelQuery>) -> impl Responder {

    let mut modifed_check_in_date = query.checkInDate.clone();
    modifed_check_in_date.push_str("T00:00:00Z");

    let mut modified_check_out_date = query.checkOutDate.clone();
    modified_check_out_date.push_str("T00:00:00Z");

    // Input validation for dates
    let converted_check_in_date = modifed_check_in_date.parse::<ChronosDateTime<Utc>>();
    let converted_check_out_date = modified_check_out_date.parse::<ChronosDateTime<Utc>>();

    match converted_check_in_date {
        Ok(_) => {},
        Err(..) => {
            return HttpResponse::BadRequest().body("Invalid check in date")
        }
    }
    
    match converted_check_out_date {
        Ok(_) => {},
        Err(..) => {
            return HttpResponse::BadRequest().body("Invalid check out date")
        }
    }

    // Implement filter to get from City and between the dates
    let filter = doc! {"city": &query.destination,
                                    "date": {"$gte": converted_check_in_date.unwrap(), 
                                            "$lte": converted_check_out_date.unwrap()}};

    let collation = Collation::builder().locale("en").strength(CollationStrength::Primary).build();
    let findoptions = FindOptions::builder().collation(collation).build();

    let cursor = hotel_collection.find(filter, findoptions).await.unwrap();

    let filtered_data = cursor.try_collect::<Vec<Hotel>>().await.unwrap();

    let mut price_map = std::collections::HashMap::new();

    for hotel in filtered_data {   
        price_map.entry(hotel.hotelName).and_modify(|price| *price += hotel.price ).or_insert(hotel.price);
    }

    let lowest_hotel = price_map.iter().min_by_key(|entry| entry.1).unwrap();
    let filtered_iter = price_map.iter().filter(|x| { x.1 == lowest_hotel.1 });
    
    let mut responses = Vec::new();
    for filter_hotel in filtered_iter {
        let response = HoteResponse { City: query.destination.clone(), 
                                                    checkInDate: query.checkInDate.clone(), 
                                                    checkOutDate: query.checkOutDate.clone(), 
                                                    Hotel: filter_hotel.0.clone(), 
                                                    Price: filter_hotel.1.clone() };
        responses.push(response);
    }
    
    
    
    
    HttpResponse::Ok().json(responses)
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flight {
    airline: String,
    airlineid: i32,
    srcairport: String,
    srcairportid: i32,
    destairport: String,
    destairportid: i32,
    codeshare: String,
    stop: i32,
    eq: String,
    airlinename: String,
    srcairportname: String,
    srccity: String,
    destairportname: String,
    destcity: String,
    destcountry: String,
    price: i32,
    date: DateTime
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hotel {
    city: String,
    hotelName: String,
    price: i32,
    date: DateTime
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let client_options = ClientOptions::parse("mongodb+srv://userReadOnly:7ZT817O8ejDfhnBM@minichallenge.q4nve1r.mongodb.net/").await.unwrap();
    let mongodb_client = Client::with_options(client_options).unwrap();

    // Get Database name
    let mongodb_database_name = &mongodb_client.list_database_names(None, None).await.unwrap()[0];

    // Get Collection handle
    let mongodb_db_handle = mongodb_client.database(mongodb_database_name);
    let hotels_collection_handle = mongodb_db_handle.collection::<Hotel>("hotels");
    let flights_collection_handle = mongodb_db_handle.collection::<Flight>("flights");


    // let hotels_cursor = hotels_collection_handle.find(None,None).await.unwrap();
    // let hotel_data = hotels_cursor.try_collect::<Vec<Hotel>>().await.unwrap();

    // let flights_cursor = flights_collection_handle.find(None, None).await.unwrap();
    // let flight_data = flights_cursor.try_collect::<Vec<Flight>>().await.unwrap();
    
    println!("Starting web server after initilializing data");

    // Setup web server
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(hotels_collection_handle.clone()))
            .app_data(web::Data::new(flights_collection_handle.clone()))
            .service(flight)
            .service(hotel)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    Ok(())
}
