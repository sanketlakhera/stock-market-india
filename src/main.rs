use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;

mod models;
// mod routes;
// mod services;
// mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv().ok();
    env_logger::init();

    info!("Starting stock market India server...");

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // Configure routes here
            .service(
                web::scope("/api")
                    // NSE routes
                    .service(web::scope("/nse"))
                    // BSE routes
                    .service(web::scope("/bse")),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
