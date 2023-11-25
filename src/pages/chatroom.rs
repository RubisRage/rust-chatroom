use crate::{database as db, models::ChatRoom};
use askama::Template;
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};

pub fn routes() -> Router<db::DatabasePool> {
    Router::new().route("/join/:name", get(get::join_room))
}

mod templates {
    use super::*;

    #[derive(Template)]
    #[template(path = "room.html")]
    pub struct ChatRoomTemplate {
        pub chatroom: ChatRoom,
    }
}

mod get {
    use super::templates::*;
    use super::*;

    pub async fn join_room(
        State(pool): State<db::DatabasePool>,
        Path(name): Path<String>,
    ) -> ChatRoomTemplate {
        let chatroom = db::get_room_by_name(pool, &name).await;

        ChatRoomTemplate { chatroom }
    }
}
