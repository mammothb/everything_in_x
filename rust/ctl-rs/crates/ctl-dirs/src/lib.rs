use std::path::PathBuf;

use etcetera::{BaseStrategy, choose_base_strategy};

#[must_use]
pub fn user_cache_dir() -> Option<PathBuf> {
    choose_base_strategy()
        .ok()
        .map(|dirs| dirs.cache_dir().join("ctl"))
}

#[must_use]
pub fn user_config_dir() -> Option<PathBuf> {
    choose_base_strategy()
        .ok()
        .map(|dirs| dirs.config_dir().join("ctl"))
}
