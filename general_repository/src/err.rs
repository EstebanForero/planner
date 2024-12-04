use thiserror::Error;
pub type Result<T> = core::result::Result<T, RepositoryError>;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Internal DB error: {0}")]
    DatabaseError(#[from] sqlx::error::Error)
}
