use crate::pages::{chatroom, home};
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::database::DatabasePool;

pub struct App {
    db: DatabasePool,
}

impl App {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db = DatabasePool::connect("sqlite:chatroom.db").await?;
        sqlx::migrate!().run(&db).await?;

        Ok(Self { db })
    }

    pub async fn serve(&self) {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let app = Router::new()
            .merge(home::routes())
            .merge(chatroom::routes())
            .nest_service("/assets", ServeDir::new("assets"))
            .with_state(DatabasePool::clone(&self.db));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect(&format!(
                "Failed to bind to address, {addr} should be valid."
            ));
    }
}
