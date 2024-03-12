mod give_me_back;
mod give_me_json;
mod hello_world;

use axum::{
    routing::{get, post},
    Router,
};

use self::{give_me_back::give_me_back, give_me_json::give_me_json, hello_world::hello_world};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/give_me_back", post(give_me_back))
        .route("/give_me_json", post(give_me_json))
}
