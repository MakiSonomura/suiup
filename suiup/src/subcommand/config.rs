use crate::process::config::Config;
use crate::Process;
use crate::Result;
pub(crate) fn run(process: &Process<'_>) -> Result<()> {
    let config = Config::from_env(process)?;
    let s = serde_json::to_string_pretty(&config)?;
    println!("{}", s);
    Ok(())
}
