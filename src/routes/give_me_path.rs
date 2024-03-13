use axum::extract::Path;

pub async fn give_me_path(Path(learn): Path<String>) -> String {
    learn
}
