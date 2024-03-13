use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QP {
    name: String,
    age: i32,
}
pub async fn give_me_query(Query(qp): Query<QP>) -> Json<QP> {
    Json(qp)
}
