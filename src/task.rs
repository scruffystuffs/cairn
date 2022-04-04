use derive_getters::Getters;
use sqlx::types::chrono::{DateTime, Local, NaiveDateTime};
use sqlx::SqlitePool;
use typed_builder::TypedBuilder;

use crate::err::CairnErr;
use crate::status::TaskStatus;

#[derive(TypedBuilder, Debug, Clone)]
pub struct NewTask {
    summary: String,
    status: TaskStatus,
    #[builder(default)]
    details: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Getters)]
pub struct Task {
    /// This ID is for database interactions only.  We never reveal it to the user.
    db_id: i64,
    /// ID for users to uniquely reference tasks.
    public_id: i64,
    /// One-liner of the task.  Unique for text searches.
    summary: String,
    /// Task status, shown to the user, and determines whther tasks get shown or not.
    status: TaskStatus,
    /// Optional task details.  Opaque text blob.
    details: Option<String>,
    /// Next time the user should receive an alert for this task. UTC ISO format.
    alarm_time: Option<NaiveDateTime>,
    /// When the task was created.  UTC ISO format
    created_time: Option<NaiveDateTime>,
}

impl Task {
    pub fn oneline(&self) -> String {
        format!("{} {}", self.status, self.summary)
    }
}

pub async fn add_task(new: &NewTask, pool: &SqlitePool) -> Result<Task, CairnErr> {
    let created_time: DateTime<Local> = Local::now();
    let public_id = created_time.timestamp();
    let insert_id = sqlx::query_file!(
        "sql/insert-new-task.sql",
        public_id,
        new.summary,
        new.status,
        new.details,
        created_time
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(
        sqlx::query_file_as!(Task, "sql/fetch-task-by-id.sql", insert_id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn fetch_summary_tasks(pool: &SqlitePool) -> Result<Vec<Task>, CairnErr> {
    Ok(sqlx::query_file_as!(Task, "sql/fetch-summary-tasks.sql")
        .fetch_all(pool)
        .await?)
}

pub async fn delete_all_tasks(pool: &SqlitePool) -> Result<u64, CairnErr> {
    let count = sqlx::query_file!("sql/truncate-tasks.sql")
        .execute(pool)
        .await?
        .rows_affected();
    Ok(count)
}
