use mongodb::Collection;
use mongodb::{Client, options::ClientOptions};
use mongodb::{bson::DateTime};
use chrono::NaiveDate;
use futures::stream::TryStreamExt;
use serde::{Serialize, Deserialize};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use iso8601_timestamp::Timestamp;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct HotelQuery {
    checkInDate: String,
    checkOutDate: String,
    destination: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct FlightQuery {
    departureDate: Timestamp,
    destination: String,
    returnDate: Timestamp
}

#[get("/flight")]
async fn flight(flight_collection: web::Data<Collection<Flight>>, query: web::Query<FlightQuery>) -> impl Responder {
    
    println!("{:#?}", query);
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hotel")]
async fn hotel(hotel_collection: web::Data<Collection<Hotel>>,  query: web::Query<HotelQuery>) -> impl Responder {
    let curr_destination: String = query.destination.to_lowercase();

    // Input validation for dates
    let converted_check_in_date = NaiveDate::parse_from_str(&query.checkInDate, "%F");
    let converted_check_out_date = NaiveDate::parse_from_str(&query.checkOutDate, "%F");

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

    // for hotel in hotel_data.get_ref() {
    //     if hotel.city.eq_ignore_ascii_case(curr_destination.as_str()) {
    //         println!("{:#?}", hotel);
    //         if NaiveDate::parse_from_str(
    //             hotel.date.try_to_rfc3339_string().unwrap().as_str(),
    //             "%FT%TZ").unwrap() == converted_check_in_date {
    //             println!("{:#?} {}", hotel, converted_check_in_date)
    //         }    
    //     }   
    // }

    HttpResponse::Ok().body("Hello world!")
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
