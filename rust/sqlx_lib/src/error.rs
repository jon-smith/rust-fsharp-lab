use thiserror::Error;

use crate::env_var_helper::EnvVarError;

#[derive(Error, Debug)]
// Error type for all of the errors that occur in our library
pub enum Error {
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("env var error")]
    EnvVarError(#[from] EnvVarError),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}
