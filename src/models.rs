use serde::{Deserialize, Deserializer};

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct ChatRoom {
    pub name: String,
    #[serde(deserialize_with = "ChatRoom::from_web_password")]
    pub password: Option<String>,
}

impl ChatRoom {
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    fn from_web_password<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| if !s.is_empty() { Some(s) } else { None })
    }

    fn coded_password(&self) -> &str {
        if let Some(password) = &self.password {
            "********"
        } else {
            "None"
        }
    }
}
