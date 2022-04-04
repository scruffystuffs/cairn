use clap::{Args, Parser, Subcommand};
use derive_getters::Getters;
use eyre::Result;

use crate::conn::default_conn_pool;
use crate::status::TaskStatus;
use crate::task::{add_task, delete_all_tasks, fetch_summary_tasks, NewTask};

#[derive(Parser, Debug, Clone, Getters)]
#[clap(version, author, long_about = None)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum Command {
    Add(AddCommand),
    Clear(ClearCommand),
    Summary(SummaryCommand),
}

impl Command {
    pub(crate) async fn run(&self) -> Result<()> {
        match self {
            Command::Add(cmd) => cmd.run().await,
            Command::Clear(cmd) => cmd.run().await,
            Command::Summary(cmd) => cmd.run().await,
        }
    }
}

#[derive(Args, Clone, Debug)]
pub(crate) struct AddCommand {
    summary: String,
    #[clap(long, takes_value = true, default_value = "Active")]
    status: TaskStatus,
}

impl AddCommand {
    async fn run(&self) -> Result<()> {
        let task = NewTask::builder()
            .status(self.status)
            .summary(self.summary.clone())
            .build();
        let pool = default_conn_pool().await?;
        add_task(&task, &pool).await?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub struct ClearCommand {}
impl ClearCommand {
    pub async fn run(&self) -> Result<()> {
        let pool = default_conn_pool().await?;
        let deleted_count = delete_all_tasks(&pool).await?;
        eprintln!("Deleted {} tasks", deleted_count);
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
pub(crate) struct SummaryCommand {}
impl SummaryCommand {
    pub async fn run(&self) -> Result<()> {
        let pool = default_conn_pool().await?;
        let tasks = fetch_summary_tasks(&pool).await?;
        for task in tasks {
            println!("{}", task.oneline());
        }
        Ok(())
    }
}
