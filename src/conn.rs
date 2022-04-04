use directories_next::ProjectDirs;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

use crate::err::CairnErr;

const DB_FILE_NAME: &str = "cairn.db";

pub async fn default_conn_pool() -> Result<SqlitePool, CairnErr> {
    let uri = default_db_uri().await?;
    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect(&uri)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

#[cfg(not(debug))]
async fn default_db_uri() -> Result<String, CairnErr> {
    use tokio::fs::create_dir_all;

    let path = ProjectDirs::from("", "", "cairn")
        .ok_or(CairnErr::NoValidDefault)?
        .data_dir()
        .join(DB_FILE_NAME);
    create_dir_all(
        &path
            .parent()
            .expect("cairn attempted to use filesystem root as database dir"),
    )
    .await?;
    Ok(format!("sqlite://{}?mode=rwc", path.display()))
}
