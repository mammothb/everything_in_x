use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

use crate::commands::ExitStatus;
use ctl_dirs::HomeDirExt;

pub(crate) struct Process {
    cmd: String,
    chdir: Option<PathBuf>,
    executable: Option<String>,
    env: Option<HashMap<String, String>>,
}

impl Process {
    pub(crate) fn new(
        cmd: String,
        chdir: Option<PathBuf>,
        executable: Option<String>,
        env: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            cmd,
            chdir,
            executable,
            env,
        }
    }

    pub(crate) fn run(&self) -> Result<ExitStatus> {
        let shell = self.executable.as_deref().unwrap_or("/bin/sh");
        let mut process = duct::cmd(shell, vec!["-c", &self.cmd]);
        if let Some(ref dir) = self.chdir {
            process = process.dir(dir.expand_home()?);
        }
        if let Some(ref vars) = self.env {
            for (key, val) in vars {
                process = process.env(key, val);
            }
        }
        let output = process.unchecked().run()?;
        match output.status.code() {
            Some(0) => Ok(ExitStatus::Success),
            Some(code) => Ok(ExitStatus::External(u8::try_from(code)?)),
            None => Ok(ExitStatus::Error),
        }
    }
}
