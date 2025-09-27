mod lambda;

use std::process::ExitCode;

pub(crate) use lambda::display::UrlType as DisplayUrlType;
pub(crate) use lambda::display::display_url as lambda_display_url;
pub(crate) use lambda::fetch::fetch as lambda_fetch;
pub(crate) use lambda::find::find as lambda_find;

#[derive(Clone, Copy)]
pub enum ExitStatus {
    Success,
    /// The command failed due to an error in the user input
    Failure,
    /// The command failed with an unexpected error
    Error,
    /// The command's exit status is propagated from an external command
    External(u8),
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        match status {
            ExitStatus::Success => Self::from(0),
            ExitStatus::Failure => Self::from(1),
            ExitStatus::Error => Self::from(2),
            ExitStatus::External(code) => Self::from(code),
        }
    }
}
