use thiserror::Error;

#[derive(Error, Debug)]
pub enum CairnErr {
    #[error("Cannot determine default database location")]
    NoValidDefault,

    #[cfg(debug)]
    #[error("Running debug binary: DATABASE_URL env var must be set")]
    NoEnvOnDev,

    #[error("Error reported from database: {0}")]
    SqlxErr(#[from] sqlx::Error),

    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Error occurred during database migratioon: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
}
