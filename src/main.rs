use std::io::Result;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};

use crate::client::rest::RestClient;

pub mod client;
pub mod constants;
mod errors;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        // rest api client will be used for making kraken api calls
        let client = Arc::new(RestClient::new());

        App::new()
            .service(web::scope("/api/v1").service(routes::last_trade_price::get_ltp))
            .app_data(Data::new(client))
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
    .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
