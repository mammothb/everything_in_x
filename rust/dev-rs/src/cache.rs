use std::path::PathBuf;

pub struct Cache {
    root: PathBuf,
}

impl Cache {
    /// A persistent cache directory at `root`.
    pub fn from_path(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
}
