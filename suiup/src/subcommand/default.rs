use crate::DefaultOpt;
use crate::{process::config::Config, Process, Result};

use crate::common::{list_all_toolkits, set_symlink, write_setting};
pub(crate) async fn run(process: &Process<'_>, opt: DefaultOpt) -> Result<()> {
    let config = Config::from_env(process)?;

    let toolkit_dir = &config.toolkits_dir;
    let binary_dir = &config.bin;

    let toolkits = list_all_toolkits(&config)?;

    let desc = toolkits.into_iter().find(|t| t.desc() == opt.desc);
    if let Some(desc) = desc {
        let toolkit_dir = toolkit_dir.join(desc.desc());
        set_symlink(&toolkit_dir, binary_dir).await?;
        write_setting(&desc, &binary_dir).await?;
        Ok(())
    } else {
        Err(crate::Error::Custom(
            format!("No such desc: {}", opt.desc).into(),
        ))
    }
}
