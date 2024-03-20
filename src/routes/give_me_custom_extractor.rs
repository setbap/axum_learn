use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AlakiValue {
    name: String,
    age: String,
}
pub async fn give_me_custom_extractor(
    Extension(alakivalue): Extension<AlakiValue>,
) -> Json<AlakiValue> {
    Json(alakivalue)
}
pub async fn alaki_middlaware(mut request: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = request.headers_mut();
    let name = headers
        .get("name")
        .unwrap_or(&HeaderValue::from_static("sina"))
        .to_str()
        .unwrap()
        .to_owned();
    let age = headers
        .get("age")
        .unwrap_or(&HeaderValue::from_static("0"))
        .to_str()
        .unwrap()
        .to_owned();

    let alakiValue = AlakiValue { name, age };
    let ext = request.extensions_mut();
    ext.insert(alakiValue);
    let response = next.run(request).await;
    Ok(response)
}
