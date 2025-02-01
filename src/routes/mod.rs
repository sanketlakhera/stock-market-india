use actix_web::web;

mod bse;
mod nse;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/nse").configure(nse::configure_routes))
            .service(web::scope("/bse").configure(bse::configure_routes)),
    );
}
