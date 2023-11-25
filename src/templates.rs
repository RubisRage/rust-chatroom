use crate::models::ChatRoom;
use askama::Template;

#[derive(Template)]
#[template(path = "room_card.html", ext = "html")]
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
#[template(path = "room.html")]
pub struct ChatRoomTemplate {
    pub chatroom: ChatRoom,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {
    pub room_cards: Vec<ChatRoomCard>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "message.html")]
pub struct Message {
    sender: String,
    content: String,
}

#[derive(Template)]
#[template(path = "create_room_dialog.html")]
pub struct CreateRoomDialog {}
