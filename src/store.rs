use axum::async_trait;
use axum::extract::{FromRef,FromRequestParts};
use axum::http::{request::Parts, StatusCode};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::auth::{RegisterUserAuth, UserAuthId};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

#[async_trait]
impl<S> FromRequestParts<S> for Store
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        Ok(Self{ connection: pool })
    }
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url).await {
                Ok(pool) => pool,
                Err(e) => panic!("couln't establish DB connection: {}", e)
            };

        Store {
            connection: db_pool,
        }
    }

    pub async fn add_user(
        &self,
        new_user: RegisterUserAuth
    ) -> Result<RegisterUserAuth, (StatusCode, String)> {
        match sqlx::query(
            "INSERT INTO users (username, password, name, last_name, surname, picture)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, password, name, last_name, surname, picture"
        )
        .bind(new_user.username)
        .bind(new_user.password)
        .bind(new_user.name)
        .bind(new_user.last_name)
        .bind(new_user.surname)
        .bind(new_user.picture)
        .map(|row: PgRow| RegisterUserAuth {
            id: Some(UserAuthId{id: row.get("id")}),
            username: row.get("username"),
            password: row.get("password"),
            name: row.get("name"),
            last_name: row.get("last_name"),
            surname: row.get("surname"),
            picture: row.get("picture"),
        })
        .fetch_one(&self.connection)
        .await {
            Ok(user) => Ok(user),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<RegisterUserAuth, (StatusCode, String)> {

        match sqlx::query("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .map(|row: PgRow| RegisterUserAuth {
                id: Some(UserAuthId{id: row.get("id")}),
                username: row.get("username"),
                password: row.get("password"),
                name: row.get("name"),
                last_name: row.get("last_name"),
                surname: row.get("surname"),
                picture: row.get("picture")
            })
            .fetch_one(&self.connection).await {
                Ok(user) => Ok(user),
                Err(e) => Err(internal_error(e))
            }
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}