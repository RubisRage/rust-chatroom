use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::database::DatabasePool;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        let username = username.to_string();
        let password = password_auth::generate_hash(password);

        Self { username, password }
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.username.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Clone, Debug)]
pub struct DatabaseBackend {
    db: DatabasePool,
}

impl DatabaseBackend {
    pub async fn new(db: DatabasePool) -> Self {
        Self { db }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl AuthnBackend for DatabaseBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = ?",
            credentials.username
        )
        .fetch_optional(&self.db)
        .await?;

        println!("User: {:?}", user);

        Ok(user.filter(|user| {
            verify_password(credentials.password, &user.password).is_ok()
        }))
    }

    async fn get_user(
        &self,
        username: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = ?",
            username
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }
}
