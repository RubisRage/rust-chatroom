use askama::Template;
use axum::{
    extract::{Form, State},
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{database as db, models::ChatRoom};

pub fn routes() -> Router<db::DatabasePool> {
    Router::new().route("/home", get(self::get::home)).route(
        "/create_room",
        get(get::create_room_dialog).post(post::create_room),
    )
}

mod templates {
    use super::*;

    #[derive(Template)]
    #[template(path = "home.html")]
    pub struct Home {
        pub room_cards: Vec<ChatRoomCard>,
    }

    #[derive(Template)]
    #[template(path = "room_card.html")]
    pub struct ChatRoomCard {
        pub name: String,
        pub has_password: bool,
    }

    impl From<ChatRoom> for ChatRoomCard {
        fn from(chat_room: ChatRoom) -> Self {
            ChatRoomCard {
                name: chat_room.name,
                has_password: chat_room.password.is_some(),
            }
        }
    }

    #[derive(Template)]
    #[template(path = "create_room_dialog.html")]
    pub struct CreateRoomDialog {}
}

mod post {
    use super::templates::*;
    use super::*;

    pub async fn create_room(
        State(pool): State<db::DatabasePool>,
        Form(chatroom): Form<ChatRoom>,
    ) -> impl IntoResponse {
        db::store_room(pool, &chatroom).await.unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Hx-Trigger", HeaderValue::from_static("closeModal"));

        let room_card = ChatRoomCard::from(chatroom);

        (headers, room_card)
    }
}

mod get {
    use axum::debug_handler;

    use super::templates::*;
    use super::*;

    #[debug_handler]
    pub async fn home(State(pool): State<db::DatabasePool>) -> Home {
        let room_cards = db::get_rooms(pool)
            .await
            .into_iter()
            .map(ChatRoomCard::from)
            .collect();

        Home { room_cards }
    }

    pub async fn create_room_dialog() -> CreateRoomDialog {
        CreateRoomDialog {}
    }
}
