mod routes;
use axum::Router;

use crate::routes::create_routes;

pub async fn run() {
    let app: Router = create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("we are live");
    axum::serve(listener, app).await.unwrap();
}
