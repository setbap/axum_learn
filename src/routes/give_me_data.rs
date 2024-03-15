use axum::{Extension, Json};

use super::SharedMessage;

pub async fn give_me_data(Extension(msg): Extension<SharedMessage>) -> Json<SharedMessage> {
    Json(msg)
}
