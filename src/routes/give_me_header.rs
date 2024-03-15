use axum::http::{header::USER_AGENT, HeaderMap};

pub async fn give_me_header(header: HeaderMap) -> String {
    let user_agent = header.get(USER_AGENT).unwrap().to_owned();
    user_agent.to_str().unwrap().to_owned()
}
