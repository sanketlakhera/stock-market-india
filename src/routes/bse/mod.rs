use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

use crate::services::BseService;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_indices)
        .service(get_index_info)
        .service(get_index_stocks)
        .service(get_stock_info)
        .service(get_gainers)
        .service(get_losers)
        .service(get_top_turnover)
        .service(get_stock_chart_data)
        .service(get_stock_info_and_day_chart);
}

#[get("/get_indices")]
async fn get_indices(bse_service: web::Data<BseService>) -> impl Responder {
    match bse_service.get_indices().await {
        Ok(indices) => HttpResponse::Ok().json(indices),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct IndexQuery {
    index_id: String,
}

#[get("/get_index_info")]
async fn get_index_info(
    query: web::Query<IndexQuery>,
    bse_service: web::Data<BseService>,
) -> impl Responder {
    match bse_service.get_index_info(&query.index_id).await {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_index_stocks")]
async fn get_index_stocks(
    query: web::Query<IndexQuery>,
    bse_service: web::Data<BseService>,
) -> impl Responder {
    match bse_service.get_index_stocks(&query.index_id).await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct StockQuery {
    security_code: String,
}

#[get("/get_stock_info")]
async fn get_stock_info(
    query: web::Query<StockQuery>,
    bse_service: web::Data<BseService>,
) -> impl Responder {
    match bse_service.get_stock_info(&query.security_code).await {
        Ok(info) => HttpResponse::Ok().json(info),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_gainers")]
async fn get_gainers(bse_service: web::Data<BseService>) -> impl Responder {
    match bse_service.get_gainers().await {
        Ok(gainers) => HttpResponse::Ok().json(gainers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_losers")]
async fn get_losers(bse_service: web::Data<BseService>) -> impl Responder {
    match bse_service.get_losers().await {
        Ok(losers) => HttpResponse::Ok().json(losers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_top_turnover")]
async fn get_top_turnover(bse_service: web::Data<BseService>) -> impl Responder {
    match bse_service.get_top_turnover().await {
        Ok(stocks) => HttpResponse::Ok().json(stocks),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
struct ChartQuery {
    security_code: String,
    time_frame: String,
}

#[get("/get_stock_chart_data")]
async fn get_stock_chart_data(
    query: web::Query<ChartQuery>,
    bse_service: web::Data<BseService>,
) -> impl Responder {
    match bse_service
        .get_stock_chart_data(&query.security_code, &query.time_frame)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/get_stock_info_and_day_chart")]
async fn get_stock_info_and_day_chart(
    query: web::Query<StockQuery>,
    bse_service: web::Data<BseService>,
) -> impl Responder {
    match bse_service
        .get_stock_info_and_day_chart(&query.security_code)
        .await
    {
        Ok((info, chart)) => HttpResponse::Ok().json(json!({
            "stock_info": info,
            "chart_data": chart
        })),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
