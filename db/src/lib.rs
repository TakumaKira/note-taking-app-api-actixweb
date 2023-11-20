#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("NotFound")]
    NotFound,

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error)
}
