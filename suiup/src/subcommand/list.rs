use crate::{common::list_all_toolkits, process::config::Config, Process, Result};

pub(crate) fn run(process: &Process<'_>) -> Result<()> {
    let config = Config::from_env(process)?;
    let toolkits = list_all_toolkits(&config)?;
    toolkits.iter().for_each(|t| println!("{}", t.desc()));
    Ok(())
}
