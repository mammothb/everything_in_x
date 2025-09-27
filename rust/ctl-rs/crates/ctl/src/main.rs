use std::process::ExitCode;

use ctl::main as ctl_main;

#[tokio::main]
async fn main() -> ExitCode {
    ctl_main().await
}
