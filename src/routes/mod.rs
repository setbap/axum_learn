mod give_me_back;
mod give_me_header;
mod give_me_json;
mod give_me_path;
mod give_me_query;
mod hello_world;

use axum::{
    routing::{get, post},
    Router,
};

use self::{
    give_me_back::give_me_back, give_me_header::give_me_header, give_me_json::give_me_json,
    give_me_path::give_me_path, give_me_query::give_me_query, hello_world::hello_world,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/give_me_back", post(give_me_back))
        .route("/give_me_json", post(give_me_json))
        .route("/give_me_path/:learn", post(give_me_path))
        .route("/give_me_query", get(give_me_query))
        .route("/give_me_header", get(give_me_header))
}
