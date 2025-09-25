pub(crate) mod commands;

use std::process::ExitCode;

use tracing_subscriber::FmtSubscriber;

use crate::commands::ExitStatus;
use ctl::run;

#[allow(clippy::print_stderr)]
#[tokio::main]
async fn main() -> ExitCode {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        eprintln!("Tracing setup error: {error}");
        return ExitStatus::Failure.into();
    }
    let result = run().await;
    match result {
        Ok(code) => code.into(),
        Err(err) => {
            eprintln!("Error: {err}");
            ExitStatus::Success.into()
        }
    }
}
