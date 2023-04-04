use thiserror::Error as ThisError;

mod db;
mod todo;

#[derive(Debug, ThisError)]
pub enum Error{
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
