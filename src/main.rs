mod database;
mod error;
mod models;
mod templates;

use database as db;

use axum::{
    extract::{Form, Path, State, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::{HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};

use sqlx::sqlite::SqlitePool;
type DatabasePool = SqlitePool;

use std::net::SocketAddr;
use tower_http::services::ServeDir;

use models::ChatRoom;
use templates::{
    ChatRoomCard, ChatRoomTemplate, CreateRoomDialog, Home, Index,
};

async fn home(State(pool): State<DatabasePool>) -> Home {
    let room_cards = db::get_rooms(pool)
        .await
        .into_iter()
        .map(ChatRoomCard::from)
        .collect();

    Home { room_cards }
}

async fn join_handler(
    State(pool): State<DatabasePool>,
    Path(name): Path<String>,
) -> ChatRoomTemplate {
    let chatroom = db::get_room_by_name(pool, &name).await;

    ChatRoomTemplate { chatroom }
}

async fn create_room_dialog() -> CreateRoomDialog {
    CreateRoomDialog {}
}

async fn create_room(
    State(pool): State<DatabasePool>,
    Form(chatroom): Form<ChatRoom>,
) -> impl IntoResponse {
    db::store_room(pool, &chatroom).await.unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Hx-Trigger", HeaderValue::from_static("closeModal"));

    let room_card = ChatRoomCard::from(chatroom);

    (headers, room_card)
}

async fn index() -> Index {
    Index {}
}

#[tokio::main]
async fn main() {
    let database_pool = DatabasePool::connect("sqlite:chatroom.db")
        .await
        .expect("Failed to connect to database, database file 'chatroom.db' should exist.");

    sqlx::migrate!()
        .run(&database_pool)
        .await
        .expect("Failed to migrate");

    let app = Router::new()
        .route("/home", get(home))
        .route("/join/:name", get(join_handler))
        .route("/create_room", get(create_room_dialog).post(create_room))
        .route("/", get(index))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(database_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect(&format!(
            "Failed to bind to address, {addr} should be valid."
        ));
}
