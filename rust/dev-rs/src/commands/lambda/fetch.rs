use std::path::Path;

use crate::cache::{Cache, CacheBucket};

pub(crate) fn fetch(path: Option<&Path>, cache: Cache) {
    println!("lambda fetch: {:?}", path);
    let cache_entry = cache.entry(CacheBucket::Lambda, format!("{}.toml", "asdf"));
}
