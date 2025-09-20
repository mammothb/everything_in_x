use anyhow::Result;

use crate::{
    cache::{Cache, CacheBucket, CacheEntry},
    config::{LambdaConfig, LambdaFetchConfig},
};

pub(crate) fn fetch(config: LambdaFetchConfig, cache: Cache) -> Result<()> {
    let LambdaFetchConfig {
        definition_path,
        config: LambdaConfig {
            environment,
            suffix,
        },
    } = config;
    let file = format!("{}{}.json", environment, suffix);
    println!("lambda fetch: {:?}", definition_path);
    let cache_entry = cache.entry(CacheBucket::Lambda, file);
    let data = vec![String::from("asdf"), String::from("qwer")];
    write_cache(cache_entry, data)?;
    Ok(())
}

fn write_cache(cache_entry: CacheEntry, data: Vec<String>) -> Result<()> {
    let content = serde_json::to_string_pretty(&data)?;
    fs_err::create_dir_all(cache_entry.dir())?;
    fs_err::write(cache_entry.path(), content)?;
    Ok(())
}
