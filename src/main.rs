pub mod routes;
pub mod models;

use actix_web::{App, HttpServer, web};
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(move || {
        App::new().service(
            web::scope("/api/v1")
                .service(routes::ltp::get_ltp),
        )
    })
        .workers(4)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
        .expect("\nERROR: src/main.rs: server initialization fail\n");

    Ok(())
}
