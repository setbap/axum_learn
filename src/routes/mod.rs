mod give_me_back;
mod give_me_custom_extractor;
mod give_me_data;
mod give_me_header;
mod give_me_json;
mod give_me_path;
mod give_me_query;
mod hello_world;

use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;

use self::{
    give_me_back::give_me_back,
    give_me_custom_extractor::{alaki_middlaware, give_me_custom_extractor},
    give_me_data::give_me_data,
    give_me_header::give_me_header,
    give_me_json::give_me_json,
    give_me_path::give_me_path,
    give_me_query::give_me_query,
    hello_world::hello_world,
};

#[derive(Clone, Serialize)]
pub struct SharedMessage {
    pub msg: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::PUT])
        .allow_origin(tower_http::cors::Any);

    let msg = SharedMessage {
        msg: "hello".to_owned(),
    };

    Router::new()
        .route("/", get(hello_world))
        .route("/give_me_back", post(give_me_back))
        .route("/give_me_json", post(give_me_json))
        .route("/give_me_path/:learn", post(give_me_path))
        .route("/give_me_query", get(give_me_query))
        .route("/give_me_header", get(give_me_header))
        .route("/give_me_data", get(give_me_data))
        .route("/give_me_custom_extractor", get(give_me_custom_extractor))
        .layer(cors)
        .layer(middleware::from_fn(alaki_middlaware))
        .layer(Extension(msg))
}
