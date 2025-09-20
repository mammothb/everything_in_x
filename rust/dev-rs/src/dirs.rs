use std::path::PathBuf;

use etcetera::{BaseStrategy, choose_base_strategy};

pub fn user_cache_dir() -> Option<PathBuf> {
    choose_base_strategy()
        .ok()
        .map(|dirs| dirs.cache_dir().join("dev_cli"))
}
