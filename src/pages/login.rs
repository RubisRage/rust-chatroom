use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    routing::{get, post},
    Router,
};

use crate::database::DatabasePool;

pub fn routes() -> Router<DatabasePool> {
    Router::new()
        .route("/login", get(get::login).post(post::login))
        .route("/signin", post(post::signin))
}

mod templates {
    use super::*;

    #[derive(Template)]
    #[template(path = "login.html")]
    pub struct LoginPage {}
}

mod get {
    use super::templates::*;
    use super::*;

    pub async fn login() -> impl IntoResponse {
        LoginPage {}
    }
}

mod post {
    use axum::{
        debug_handler,
        extract::{Form, Query, State},
        http::StatusCode,
        response::Redirect,
    };
    use axum_htmx::HxLocation;
    use axum_login;
    use serde::Deserialize;

    use crate::{
        auth::{Credentials, DatabaseBackend, User},
        database::DatabasePool,
    };
    type AuthSession = axum_login::AuthSession<DatabaseBackend>;

    #[derive(Deserialize)]
    pub struct NextPage {
        next: Option<String>,
    }

    #[debug_handler]
    pub async fn login(
        mut auth: AuthSession,
        Query(NextPage { next }): Query<NextPage>,
        Form(inputs): Form<Credentials>,
    ) -> Result<(HxLocation, StatusCode), (StatusCode, &'static str)> {
        println!("Inputs: {:?}", inputs);

        let user = match auth.authenticate(inputs).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    &"Invalid username or password",
                ));
            }
            Err(e) => {
                eprintln!("Error authenticating: {:?}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &"Internal server error",
                ));
            }
        };

        if let Err(e) = auth.login(&user).await {
            eprintln!("Error logging in: {:?}", e);
            return Err((StatusCode::UNAUTHORIZED, &"Internal server error"));
        }

        let headers = match next {
            Some(next) => HxLocation(next.parse().unwrap()),
            None => HxLocation("/home".parse().unwrap()),
        };

        Ok((headers, StatusCode::OK))
    }

    use crate::database as db;

    #[debug_handler]
    pub async fn signin(
        mut auth: AuthSession,
        State(pool): State<DatabasePool>,
        Query(NextPage { next }): Query<NextPage>,
        Form(inputs): Form<Credentials>,
    ) -> Result<(HxLocation, StatusCode), (StatusCode, &'static str)> {
        let user = User::new(&inputs.username, "1234");

        if let Err(e) = db::store_user(pool, &user).await {
            eprintln!("Error storing user: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                &"Internal server error",
            ));
        }

        if let Err(e) = auth.login(&user).await {
            eprintln!("Error logging in: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                &"Internal server error",
            ));
        }

        let headers = match next {
            Some(next) => HxLocation(next.parse().unwrap()),
            None => HxLocation("/home".parse().unwrap()),
        };

        Ok((headers, StatusCode::OK))
    }
}
