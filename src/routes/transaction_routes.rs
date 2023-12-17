use crate::controllers::transaction_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/transactions")
            .route(web::get().to(transaction_controller::get_transactions))
            .route(web::post().to(transaction_controller::add_transaction))
    )
    .service(
        web::resource("/transactions/{id}")
            .route(web::put().to(transaction_controller::update_transaction))
            .route(web::delete().to(transaction_controller::delete_transaction))
    );
}
