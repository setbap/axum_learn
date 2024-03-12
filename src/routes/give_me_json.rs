use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UselessData {
    name: String,
}
pub async fn give_me_json(Json(data): Json<UselessData>) -> Json<UselessData> {
    Json(data)
}
