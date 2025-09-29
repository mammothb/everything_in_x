use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

use crate::commands::ExitStatus;
use crate::process::Process;

pub(crate) async fn up() -> Result<ExitStatus> {
    let env = HashMap::new();
    let shell = Process::new(
        String::from("ls"),
        Some(PathBuf::from("~/code/everything_in_x/rust/ctl-rs")),
        None,
        Some(env),
    );
    let result = shell.run()?;
    println!("{result:?}");
    Ok(ExitStatus::Success)
}
