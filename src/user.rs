use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::database::DatabasePool;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub username: String,
    password: String,
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

#[derive(Clone)]
struct DatabaseBackend {
    db: DatabasePool,
}

impl DatabaseBackend {
    pub async fn new(db: DatabasePool) -> Self {
        Self { db }
    }
}

#[derive(Clone)]
struct Credentials {
    username: String,
    password: String,
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
            creadentials.username
        );

        todo!();
    }

    async fn get_user() {
        todo!();
    }
}
