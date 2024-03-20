mod routes;
use axum::Router;

use crate::routes::create_routes;

pub async fn run() {
    let app: Router = create_routes();
    let addr = "localhost:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("we are live @ {}", addr);
    axum::serve(listener, app).await.unwrap();
}
