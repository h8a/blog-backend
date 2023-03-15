use axum::async_trait;
use axum::extract::{FromRef,FromRequestParts};
use axum::http::{request::Parts, StatusCode};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::{Row, types::Uuid};

use crate::types::auth::{RegisterUserAuth, UserAuthId};
use crate::types::media::{Media, MediaId};
use crate::types::post::{Post, PostId, ReferencesPosts, ReferencesPostsId, CommentsPosts, CommentsPostsId};

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

    pub async fn add_file(
        &self,
        name: &str,
        name_generated: Uuid,
        content_type: &str,
        path: &str,
        user_id: i32
    ) -> Result<Media, (StatusCode, String)> {
        match sqlx::query(
            "INSERT INTO media (name, name_generated, content_type, path, user_id) VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, name_generated, content_type, path, user_id"
        )
        .bind(name)
        .bind(name_generated)
        .bind(content_type)
        .bind(path)
        .bind(user_id)
        .map(|row: PgRow| Media {
            id: Some(MediaId{id: row.get("id")}),
            name: row.get("name"),
            name_generated: row.get("name_generated"),
            content_type: row.get("content_type"),
            path: row.get("path"),
            user_id: row.get("user_id")
        })
        .fetch_one(&self.connection)
        .await {
            Ok(media) => Ok(media),
            Err(e) => {
                println!("ERROR MEDIA: {:?}", e);
                Err(internal_error(e))
            }
        }
    }

    pub async fn get_media_by_name_generated(&self, name_generated: &str) -> Result<Media, (StatusCode, String)> {
        match sqlx::query("SELECT * FROM media WHERE name_generated = $1")
            .bind(Uuid::parse_str(&name_generated).unwrap())
            .map(|row: PgRow| Media {
                id: Some(MediaId { id: row.get("id") }),
                name: row.get("name"),
                name_generated: row.get("name_generated"),
                content_type: row.get("content_type"),
                path: row.get("path"),
                user_id: row.get("user_id")
            })
            .fetch_one(&self.connection).await {
                Ok(media) => Ok(media),
                Err(e) => Err(internal_error(e))
            }
    }

    pub async fn create_posts(
        &self,
        title: &str,
        body: &str,
        slug: &str,
        user_id: i32
    ) -> Result<Post, (StatusCode, String)> {
        match sqlx::query("INSERT INTO posts (title, body, slug, user_id) VALUES ($1, $2, $3, $4)
        RETURNING id, title, body, slug, created_on, user_id")
        .bind(title)
        .bind(body)
        .bind(slug)
        .bind(user_id)
        .map(|row: PgRow| Post {
            id: Some(PostId{id: row.get("id")}),
            title: row.get("title"),
            body: row.get("body"),
            slug: Some(row.get("slug")),
            created_on: Some(row.get("created_on")),
            user_id: Some(row.get("user_id"))
        })
        .fetch_one(&self.connection).await {
            Ok(post) => Ok(post),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn get_post(
        &self,
        id: i32
    ) -> Result<Post, (StatusCode, String)> {
        match sqlx::query("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| Post {
            id: Some(PostId { id: row.get("id") }),
            title: row.get("title"),
            body: row.get("body"),
            slug: Some(row.get("slug")),
            created_on: Some(row.get("created_on")),
            user_id: Some(row.get("user_id"))
        })
        .fetch_one(&self.connection).await {
            Ok(post) => Ok(post),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn update_posts(
        &self,
        id: i32,
        title: &str,
        body: &str,
        slug: &str
    ) -> Result<Post, (StatusCode, String)> {
        match sqlx::query("UPDATE posts SET title = $1, body = $2, slug = $3 WHERE id = $4
        RETURNING id, title, body, slug, created_on, user_id")
        .bind(title)
        .bind(body)
        .bind(slug)
        .bind(id)
        .map(|row: PgRow| Post {
            id: Some(PostId { id: row.get("id") }),
            title: row.get("title"),
            body: row.get("body"),
            slug: Some(row.get("slug")),
            created_on: Some(row.get("created_on")),
            user_id: Some(row.get("user_id"))
        })
        .fetch_one(&self.connection).await {
            Ok(post) => Ok(post),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn delete_posts(
        &self,
        id: i32
    ) -> Result<bool, (StatusCode, String)> {
        match sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&self.connection).await {
            Ok(_) => Ok(true),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn list_posts(
        &self,
        start: i32,
        limit: i32
    ) -> Result<Vec<Post>, (StatusCode, String)> {
        match sqlx::query("SELECT * FROM posts ORDER BY created_on DESC OFFSET $1 LIMIT $2")
        .bind(start)
        .bind(limit)
        .map(|row: PgRow| Post {
            id: Some(PostId { id: row.get("id") }),
            title: row.get("title"),
            body: row.get("body"),
            slug: Some(row.get("slug")),
            created_on: Some(row.get("created_on")),
            user_id: Some(row.get("user_id"))
        })
        .fetch_all(&self.connection).await {
            Ok(post) => Ok(post),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn create_references_posts(
        &self,
        name: &str,
        url: &str,
        post_id: i32,
        user_id: i32
    ) -> Result<ReferencesPosts, (StatusCode, String)> {
        match sqlx::query("INSERT INTO posts_references (name, url, post_id, user_id) VALUES ($1, $2, $3, $4)
        RETURNING id, name, url, created_on, post_id, user_id")
        .bind(name)
        .bind(url)
        .bind(post_id)
        .bind(user_id)
        .map(|row: PgRow| ReferencesPosts {
            id: Some(ReferencesPostsId{ id: row.get("id") }),
            name: row.get("name"),
            url: row.get("url"),
            created_on: Some(row.get("created_on")),
            post_id: Some(row.get("post_id")),
            user_id: Some(row.get("user_id"))
        })
        .fetch_one(&self.connection).await {
            Ok(reference) => Ok(reference),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn update_references_posts(
        &self,
        id: i32,
        title: &str,
        url: &str
    ) -> Result<ReferencesPosts, (StatusCode, String)> {
        match sqlx::query("UPDATE posts_references SET name = $1, url = $2 WHERE id = $3
        RETURNING id, name, url, created_on, post_id, user_id")
        .bind(title)
        .bind(url)
        .bind(id)
        .map(|row: PgRow| ReferencesPosts {
            id: Some(ReferencesPostsId{ id: row.get("id") }),
            name: row.get("name"),
            url: row.get("url"),
            created_on: Some(row.get("created_on")),
            post_id: row.get("post_id"),
            user_id: Some(row.get("user_id"))
        })
        .fetch_one(&self.connection).await {
            Ok(reference) => Ok(reference),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn delete_references_posts(
        &self,
        id: i32
    ) -> Result<bool, (StatusCode, String)> {
        match sqlx::query("DELETE FROM posts_references WHERE id = $1")
        .bind(id)
        .execute(&self.connection).await {
            Ok(_) => Ok(true),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn lists_references_posts(
        &self,
        post_id: i32
    ) -> Result<Vec<ReferencesPosts>, (StatusCode, String)> {
        match sqlx::query("SELECT * FROM posts_references WHERE post_id = $1")
        .bind(post_id)
        .map(|row: PgRow| ReferencesPosts {
            id: Some(ReferencesPostsId { id: row.get("id") }),
            name: row.get("name"),
            url: row.get("url"),
            created_on: Some(row.get("created_on")),
            post_id: Some(row.get("post_id")),
            user_id: Some(row.get("user_id"))
        })
        .fetch_all(&self.connection).await {
            Ok(references) => Ok(references),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn create_comments_posts(
        &self,
        comment: &str,
        nickname: &str,
        email: &str,
        post_id: i32,
        parent_id: Option<i32>
    ) -> Result<CommentsPosts, (StatusCode, String)> {
        let parent = if parent_id.is_some() { parent_id } else { None };

        let query;
        if parent.is_some() {
            query = "INSERT INTO posts_comments (comment, nickname, email, post_id, parent_id) VALUES ($1, $2, $3, $4, $5)
            RETURNING id, comment, nickname, email, post_id, parent_id, created_on";
        } else {
            query = "INSERT INTO posts_comments (comment, nickname, email, post_id, parent_id) VALUES ($1, $2, $3, $4, null)
            RETURNING id, comment, nickname, email, post_id, parent_id, created_on";
        }

        match sqlx::query(query)
        .bind(comment)
        .bind(nickname)
        .bind(email)
        .bind(post_id)
        .bind(parent)
        .map(|row: PgRow| CommentsPosts {
            id: Some(CommentsPostsId{ id: row.get("id") }),
            comment: row.get("comment"),
            created_on: Some(row.get("created_on")),
            nickname: row.get("nickname"),
            email: row.get("email"),
            post_id: row.get("post_id"),
            parent_id: row.get("parent_id")
        })
        .fetch_one(&self.connection).await {
            Ok(comment) => Ok(comment),
            Err(e) => Err(internal_error(e))
        }
    }

    pub async fn delete_comments_posts(
        &self,
        id: i32
    ) -> Result<bool, (StatusCode, String)> {
        match sqlx::query("DELETE FROM posts_comments WHERE id = $1")
        .bind(id)
        .execute(&self.connection).await {
            Ok(_) => Ok(true),
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