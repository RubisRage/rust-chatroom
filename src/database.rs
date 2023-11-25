use crate::models::ChatRoom;
use sqlx::sqlite::SqlitePool;

pub async fn get_rooms(pool: SqlitePool) -> Vec<ChatRoom> {
    let rooms = sqlx::query_as::<_, ChatRoom>("SELECT name, password FROM chatrooms")
        .fetch_all(&pool)
        .await;

    if let Ok(rooms) = rooms {
        rooms
    } else {
        vec![]
    }
}

pub async fn get_room_by_name(pool: SqlitePool, name: &str) -> ChatRoom {
    let room = sqlx::query_as::<_, ChatRoom>("SELECT name, password FROM chatrooms WHERE name=?")
        .bind(&name)
        .fetch_one(&pool)
        .await;

    if let Ok(room) = room {
        room
    } else {
        ChatRoom {
            name: "Error room".into(),
            password: None,
        }
    }
}

pub async fn store_room(pool: SqlitePool, chatroom: &ChatRoom) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO chatrooms (name, password) VALUES ( ?, ?);")
        .bind(&chatroom.name)
        .bind(&chatroom.password)
        .execute(&pool)
        .await?;

    Ok(())
}
