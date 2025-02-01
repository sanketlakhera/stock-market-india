use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::services::NseService;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_market_status)
        .service(get_indices)
        .service(get_quote_info)
        .service(get_multiple_quote_info)
        .service(get_gainers)
        .service(get_losers)
        .service(get_incline_decline)
        .service(get_index_stocks)
        .service(search_stocks)
        .service(get_52_week_high)
        .service(get_52_week_low)
        .service(get_top_value_stocks)
        .service(get_top_volume_stocks);
}

#[get("/get_market_status")]
async fn get_market_status(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_market_status().await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_indices")]
async fn get_indices(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_indices().await {
        Ok(indices) => HttpResponse::Ok().json(indices),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct QuoteQuery {
    symbol: String,
}

#[get("/get_quote_info")]
async fn get_quote_info(
    query: web::Query<QuoteQuery>,
    nse_service: web::Data<NseService>,
) -> impl Responder {
    match nse_service.get_quote_info(&query.symbol).await {
        Ok(quote) => HttpResponse::Ok().json(quote),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct MultipleQuoteQuery {
    symbols: String,
}

#[get("/get_multiple_quote_info")]
async fn get_multiple_quote_info(
    query: web::Query<MultipleQuoteQuery>,
    nse_service: web::Data<NseService>,
) -> impl Responder {
    let symbols: Vec<&str> = query.symbols.split(',').collect();
    let mut quotes = Vec::new();

    for symbol in symbols {
        if let Ok(quote) = nse_service.get_quote_info(symbol.trim()).await {
            quotes.push(quote);
        }
    }

    HttpResponse::Ok().json(quotes)
}

#[get("/get_gainers")]
async fn get_gainers(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_gainers().await {
        Ok(gainers) => HttpResponse::Ok().json(gainers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_losers")]
async fn get_losers(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_losers().await {
        Ok(losers) => HttpResponse::Ok().json(losers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_incline_decline")]
async fn get_incline_decline(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_advance_decline().await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct IndexQuery {
    symbol: String,
}

#[get("/get_index_stocks")]
async fn get_index_stocks(
    query: web::Query<IndexQuery>,
    nse_service: web::Data<NseService>,
) -> impl Responder {
    match nse_service.get_index_stocks(&query.symbol).await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct SearchQuery {
    keyword: String,
}

#[get("/search_stocks")]
async fn search_stocks(
    query: web::Query<SearchQuery>,
    nse_service: web::Data<NseService>,
) -> impl Responder {
    match nse_service.search_stocks(&query.keyword).await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_52_week_high")]
async fn get_52_week_high(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_52_week_high().await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_52_week_low")]
async fn get_52_week_low(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_52_week_low().await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_top_value_stocks")]
async fn get_top_value_stocks(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_top_value_stocks().await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_top_volume_stocks")]
async fn get_top_volume_stocks(nse_service: web::Data<NseService>) -> impl Responder {
    match nse_service.get_top_volume_stocks().await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
