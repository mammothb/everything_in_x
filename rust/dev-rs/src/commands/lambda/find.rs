use anyhow::Result;

use crate::{
    cache::{Cache, CacheBucket, CacheEntry},
    config::LambdaConfig,
};

pub(crate) fn find(config: &LambdaConfig, cache: &Cache) -> Result<()> {
    let LambdaConfig {
        environment,
        suffix,
        ..
    } = config;

    let file = format!("{}{}.json", environment, suffix);
    let cache_entry = cache.entry(CacheBucket::Lambda, file);
    let lambda_names = read_cache(cache_entry)?;
    println!("{lambda_names:?}");
    Ok(())
}

fn read_cache(cache_entry: CacheEntry) -> Result<Vec<String>> {
    let cache_path = cache_entry.get()?;
    let content = fs_err::read_to_string(cache_path)?;
    let data: Vec<String> = serde_json::from_str(&content)?;
    Ok(data)
}
