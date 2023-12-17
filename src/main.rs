mod routes;
mod models;
mod controllers;

use actix_web::{App, HttpServer};
use routes::transaction_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .configure(transaction_routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
