use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Request Models
#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostCreate {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentCreate {
    pub content: String,
}

// Response Models
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Post {
    pub id: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    pub content: String,
    #[serde(rename = "likeCount")]
    pub like_count: i64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Comment {
    pub id: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    pub post_id: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

// Database row structs
#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostRow {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub like_count: Option<i64>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostCreateRow {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CommentRow {
    pub id: Uuid,
    pub author_id: Uuid,
    pub post_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LoginRow {
    pub id: Uuid,
    pub password_hash: String,
    pub is_admin: bool,
}

// Conversion implementations
impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id.to_string(),
            username: row.username,
            email: row.email,
            bio: row.bio,
            created_at: row.created_at,
        }
    }
}

impl From<PostRow> for Post {
    fn from(row: PostRow) -> Self {
        Self {
            id: row.id.to_string(),
            author_id: row.author_id.to_string(),
            content: row.content,
            like_count: row.like_count.unwrap_or(0),
            created_at: row.created_at,
        }
    }
}

impl From<PostCreateRow> for Post {
    fn from(row: PostCreateRow) -> Self {
        Self {
            id: row.id.to_string(),
            author_id: row.author_id.to_string(),
            content: row.content,
            like_count: 0, // New posts always have 0 likes
            created_at: row.created_at,
        }
    }
}

impl From<CommentRow> for Comment {
    fn from(row: CommentRow) -> Self {
        Self {
            id: row.id.to_string(),
            author_id: row.author_id.to_string(),
            post_id: row.post_id.to_string(),
            content: row.content,
            created_at: row.created_at,
        }
    }
}
