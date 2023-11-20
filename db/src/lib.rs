use anyhow::Result;
use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::sqlite::SqlitePool;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("NotFound")]
    NotFound,

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error)
}

#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[async_trait]
pub trait NoteRepository {
    async fn all(&self) -> Result<Vec<Note>>;
    async fn get(&self, id: &str) -> Result<Note>;
    async fn create(&self, note: &NewNote) -> Result<Note>;
    async fn update(&self, id: &str, note: &UpdateNote) -> Result<Note>;
    async fn delete(&self, id: &str) -> Result<Note>;
}

pub struct SqliteNoteRepository {
    pool: SqlitePool,
}

impl SqliteNoteRepository {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;

        Ok(SqliteNoteRepository { pool })
    }
}

#[async_trait]
impl NoteRepository for SqliteNoteRepository {
    async fn all(&self) -> Result<Vec<Note>> {
        let notes = sqlx::query_as!(Note, "SELECT * FROM note")
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    async fn get(&self, id: &str) -> Result<Note> {
        let note = sqlx::query_as!(Note, "SELECT * FROM note WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(note)
    }

    async fn create(&self, note: &NewNote) -> Result<Note> {
        let new_note = sqlx::query_as!(
            Note,
            "INSERT INTO note (id, title, content, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
            note.id, note.title, note.content, note.created_at
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(new_note)
    }

    async fn update(&self, id: &str, note: &UpdateNote) -> Result<Note> {
        let updated_note = sqlx::query_as!(
            Note,
            "UPDATE note SET title = $1, content = $2 WHERE id = $3 RETURNING *",
            note.title, note.content, id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                match e {
                    sqlx::Error::RowNotFound => DbError::NotFound,
                    _ => DbError::SqlxError(e),
                }
            })?;

        Ok(updated_note)
    }

    async fn delete(&self, id: &str) -> Result<Note> {
        let deleted_note = sqlx::query_as!(Note, "DELETE FROM note WHERE id = ? RETURNING *", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                match e {
                    sqlx::Error::RowNotFound => DbError::NotFound,
                    _ => DbError::SqlxError(e),
                }
            })?;

        Ok(deleted_note)
    }
}
