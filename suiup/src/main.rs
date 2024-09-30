use anyhow::Result;
use clap::Parser;
use suiup::{Process, CLI};

#[tokio::main]
async fn main() -> Result<()> {
    let process = Process::os();
    let cli = CLI::parse();
    cli.run(&process).await?;

    Ok(())
}
