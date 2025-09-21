use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

use crate::{
    cache::{Cache, CacheBucket, CacheEntry},
    config::LambdaConfig,
};

pub(crate) fn find(config: &LambdaConfig, cache: &Cache) -> Result<String> {
    let LambdaConfig {
        environment,
        suffix,
        ..
    } = config;

    let file = format!("{}{}.json", environment, suffix);
    let cache_entry = cache.entry(CacheBucket::Lambda, file);
    let lambda_names = read_cache(cache_entry)?;

    let mut child = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let stdin = child.stdin.as_mut().context("failed to open stdin")?;
        for name in &lambda_names {
            writeln!(stdin, "{name}")?;
        }
    }

    let output = child.wait_with_output()?;
    let selected = String::from_utf8(output.stdout)?.trim().to_string();

    Ok(selected)
}

fn read_cache(cache_entry: CacheEntry) -> Result<Vec<String>> {
    let cache_path = cache_entry.get()?;
    let content = fs_err::read_to_string(cache_path)?;
    let data: Vec<String> = serde_json::from_str(&content)?;
    Ok(data)
}
