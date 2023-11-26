use crate::pages::{chatroom, home, not_found, login};
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{BoxError, Router};
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::{login_required, AuthManagerLayerBuilder};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;

use crate::auth::DatabaseBackend;
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
        // Session layer
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));

        // Auth layer
        let auth_backend =
            DatabaseBackend::new(DatabasePool::clone(&self.db)).await;
        let auth_layer = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::BAD_REQUEST
            }))
            .layer(
                AuthManagerLayerBuilder::new(auth_backend, session_layer)
                    .build(),
            );

        let app = Router::new()
            .merge(home::routes())
            .merge(chatroom::routes())
            .route_layer(login_required!(DatabaseBackend, login_url = "/login"))
            .merge(login::routes())
            .nest_service("/assets", ServeDir::new("assets"))
            .with_state(DatabasePool::clone(&self.db))
            .layer(auth_layer)
            .fallback(not_found::handler);


        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect(&format!(
                "Failed to bind to address, {addr} should be valid."
            ));
    }
}
