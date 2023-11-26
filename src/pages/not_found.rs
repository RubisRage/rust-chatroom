use askama::Template;
use askama_axum::IntoResponse;
use axum::http::StatusCode;

pub async fn handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NotFoundTemplate {})
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate {}
