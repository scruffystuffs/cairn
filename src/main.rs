mod cli;
mod conn;
mod err;
mod status;
mod task;

use clap::Parser;
use cli::Cli;

use eyre::Result;

use self::cli::SummaryCommand;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Cli::parse();
    if let Some(cmd) = args.command() {
        cmd.run().await
    } else {
        run_summary().await
    }
}

async fn run_summary() -> Result<()> {
    SummaryCommand {}.run().await
}
