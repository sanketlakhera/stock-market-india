use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;

mod models;
mod routes;
mod services;
mod utils;

use services::{BseService, NseService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    info!("Starting stock market India server...");

    // Initialize services
    let nse_service = web::Data::new(NseService::new());
    let bse_service = web::Data::new(BseService::new());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(nse_service.clone())
            .app_data(bse_service.clone())
            .configure(routes::configure_routes)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
