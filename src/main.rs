mod app;
mod database;
mod error;
mod models;
mod pages;

use app::App;

#[tokio::main]
async fn main() {
    App::new()
        .await
        .expect("Failed to create app.")
        .serve()
        .await;
}
