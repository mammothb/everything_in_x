use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use anyhow::Result;

pub struct Cache {
    root: PathBuf,
}

impl Cache {
    pub fn from_path(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn bucket(&self, cache_bucket: CacheBucket) -> PathBuf {
        self.root.join(cache_bucket.to_str())
    }

    pub fn init(self) -> Result<Self> {
        let root = &self.root;

        fs_err::create_dir_all(root)?;

        Ok(Self {
            root: std::path::absolute(root)?,
        })
    }

    pub fn entry(
        &self,
        cache_bucket: CacheBucket,
        file: impl AsRef<Path>,
    ) -> CacheEntry {
        CacheEntry::new(self.bucket(cache_bucket), file)
    }
}

#[derive(Clone, Copy)]
pub enum CacheBucket {
    /// Lambda names
    Lambda,
}

impl CacheBucket {
    fn to_str(self) -> &'static str {
        match self {
            Self::Lambda => "lambda-v0",
        }
    }
}

impl Display for CacheBucket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

#[derive(Clone, Debug)]
pub struct CacheEntry(PathBuf);

impl CacheEntry {
    pub fn new(dir: impl Into<PathBuf>, file: impl AsRef<Path>) -> Self {
        Self(dir.into().join(file))
    }

    #[inline]
    pub fn dir(&self) -> &Path {
        self.0.parent().expect("Cache entry has no parent")
    }

    #[inline]
    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn get(&self) -> Result<&Path> {
        fs_err::create_dir_all(self.dir())?;
        Ok(self.path())
    }
}
