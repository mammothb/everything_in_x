use std::fmt::{Display, Formatter};
use std::io;
use std::path::{Path, PathBuf};

pub struct Cache {
    root: PathBuf,
}

impl Cache {
    pub fn from_path(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    #[must_use]
    pub fn bucket(&self, cache_bucket: CacheBucket) -> PathBuf {
        self.root.join(cache_bucket.to_str())
    }

    /// Initializes the cache.
    ///
    /// # Errors
    ///
    /// * When creating the cache folder fails.
    /// * When retrieving cache's absolute root path fails.
    pub fn init(self) -> Result<Self, io::Error> {
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

    /// Returns the cache entry's parent directory.
    ///
    /// # Panics
    ///
    /// When the cache entry has no parent directory.
    #[inline]
    #[must_use]
    pub fn dir(&self) -> &Path {
        self.0.parent().expect("Cache entry has no parent")
    }

    #[inline]
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.0
    }

    /// Acquires the cache entry.
    ///
    /// # Errors
    ///
    /// When there is an error creating the cache entry's parent directory.
    pub fn get(&self) -> Result<&Path, io::Error> {
        fs_err::create_dir_all(self.dir())?;
        Ok(self.path())
    }
}
