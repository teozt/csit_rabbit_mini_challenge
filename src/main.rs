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
    CheckInDate: String,
    #[serde(rename = "Check Out Date")]
    CheckOutDate: String,
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


#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FlightResponse {
    pub City: String,
    #[serde(rename = "Departure Date")]
    DepartureDate: String,
    #[serde(rename = "Depature Airline")]
    DepartureAirline: String,
    #[serde(rename = "Depature Price")]
    DepaturePrice: i32,
    #[serde(rename = "Return Date")]
    ReturnDate: String,
    #[serde(rename = "Return Airline")]
    ReturnAirline: String,
    #[serde(rename = "Return Price")]
    ReturnPrice: i32,
}

async fn validate_flight_dates(depature_date: Result<ChronosDateTime<Utc>, chrono::format::ParseError>, return_date: Result<ChronosDateTime<Utc>, chrono::format::ParseError>) -> Option<HttpResponse> {
    let invalid_depature_date: bool;
    let invalid_return_date: bool;

    match depature_date {
        Ok(_) => {
            invalid_depature_date = false;
        },
        Err(..) => {
            invalid_depature_date = true;
        }
    }
    
    match return_date {
        Ok(_) => {
            invalid_return_date = false;
        },
        Err(..) => {
            invalid_return_date = true;
        }
    }

    if invalid_depature_date && invalid_return_date {
        
        return Some(HttpResponse::BadRequest().body("Invalid depature and return date"))
    }
    else if invalid_depature_date || invalid_return_date {
        if invalid_depature_date {
            return Some(HttpResponse::BadRequest().body("Invalid depature date"))
        }
        if invalid_return_date {
            return Some(HttpResponse::BadRequest().body("Invalid return date"))
        }
    }

    return None
}

#[get("/flight")]
async fn flight(flight_collection: web::Data<Collection<Flight>>, query: web::Query<FlightQuery>) -> impl Responder {
    
    let mut modified_departure_date = query.departureDate.clone();
    modified_departure_date.push_str("T00:00:00Z");

    let mut modified_return_date = query.returnDate.clone();
    modified_return_date.push_str("T00:00:00Z");
    
    // Input validation for dates
    let converted_depature_date = modified_departure_date.parse::<ChronosDateTime<Utc>>();
    let converted_return_date = modified_return_date.parse::<ChronosDateTime<Utc>>();
    
    let error_response = validate_flight_dates(converted_depature_date, converted_return_date).await;

    match error_response {
        None => (),
        Some(error_response) => {
            return error_response
        }
    }

    // Processing flight query
    // Implement filter to get flights from singapore to city on a date
    let filter = doc! {"city": &query.destination,
    "date": {"$gte": converted_check_in_date.unwrap(), 
            "$lte": converted_check_out_date.unwrap()}};



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

    let mut responses = Vec::new();

    if filtered_data.len() != 0 {

        let mut price_map = std::collections::HashMap::new();

        for hotel in filtered_data {   
            price_map.entry(hotel.hotelName).and_modify(|price| *price += hotel.price ).or_insert(hotel.price);
        }

        let lowest_hotel = price_map.iter().min_by_key(|entry| entry.1).unwrap();
        let filtered_iter = price_map.iter().filter(|x| { x.1 == lowest_hotel.1 });
        
        
        for filter_hotel in filtered_iter {
            let response = HoteResponse { City: query.destination.clone(), 
                                                        CheckInDate: query.checkInDate.clone(), 
                                                        CheckOutDate: query.checkOutDate.clone(), 
                                                        Hotel: filter_hotel.0.clone(), 
                                                        Price: filter_hotel.1.clone() };
            responses.push(response);
        }
    
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
