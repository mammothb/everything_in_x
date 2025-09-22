use std::path::Path;

use anyhow::{Result, anyhow};
use aws_sdk_cloudformation::Client;
use saphyr::{LoadableYamlNode, Yaml};

use crate::{
    cache::{Cache, CacheBucket, CacheEntry},
    config::{LambdaConfig, LambdaFetchConfig},
};
use dev_rs::types::StackSuffix;

const DEFAULT_STACK_NAMES: [&str; 2] = ["MyStacks-First", "MyStacks-Second"];

pub(crate) async fn fetch(
    config: &LambdaFetchConfig,
    cache: &Cache,
) -> Result<()> {
    let LambdaFetchConfig {
        definition_path,
        config:
            LambdaConfig {
                environment,
                suffix,
                verbose,
            },
    } = config;

    let stack_names: Vec<String> = match definition_path {
        Some(path) => parse_stack_names(path)?,
        None => DEFAULT_STACK_NAMES.iter().map(|&s| s.into()).collect(),
    };
    let stack_names = with_suffix(&stack_names, suffix);
    let data = fetch_all_lambda_names(&stack_names, *verbose).await?;

    let file = format!("{environment}{suffix}.json");
    let cache_entry = cache.entry(CacheBucket::Lambda, file);
    write_cache(cache_entry, &data)?;
    Ok(())
}

async fn fetch_all_lambda_names(
    stack_names: &[String],
    verbose: bool,
) -> Result<Vec<String>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let futures = stack_names
        .iter()
        .map(|stack_name| fetch_lambda_names(&client, stack_name, verbose));
    let results = futures::future::join_all(futures).await;

    results
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map(|names| names.into_iter().flatten().collect())
}

async fn fetch_lambda_names(
    client: &Client,
    stack_name: &str,
    verbose: bool,
) -> Result<Vec<String>> {
    if verbose {
        tracing::info!("Fetching stack '{stack_name}'");
    }
    let response = client
        .list_stack_resources()
        .stack_name(stack_name)
        .send()
        .await?;
    Ok(response
        .stack_resource_summaries()
        .iter()
        .filter(|res| res.resource_type() == Some("AWS::Lambda::Function"))
        .filter_map(|res| res.physical_resource_id().map(str::to_owned))
        .collect())
}

/// Parses the definition file for the stack names
fn parse_stack_names(path: &Path) -> Result<Vec<String>> {
    let content = fs_err::read_to_string(path)?;
    let docs = Yaml::load_from_str(&content)?;
    let stacks = &docs[0]["stacks"]
        .as_vec()
        .ok_or_else(|| anyhow!("'stacks' must be a sequence"))?;
    Ok(stacks
        .iter()
        .filter_map(|stack| stack["name"].as_str().map(str::to_owned))
        .collect())
}

/// Appends the stack suffix to each item in the provided `items`
fn with_suffix(items: &[String], suffix: &StackSuffix) -> Vec<String> {
    let suffix_name = suffix.to_string();
    items
        .iter()
        .map(|item| format!("{item}{suffix_name}"))
        .collect()
}

fn write_cache(cache_entry: CacheEntry, data: &[String]) -> Result<()> {
    let content = serde_json::to_string_pretty(data)?;
    let cache_path = cache_entry.get()?;
    fs_err::write(cache_path, content)?;
    Ok(())
}
