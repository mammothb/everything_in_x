pub(crate) mod commands;

use anyhow::Result;

use crate::commands::ExitStatus;

pub async fn run() -> Result<ExitStatus> {
    Ok(ExitStatus::Success)
}
