use std::path::{Component, Path, PathBuf};

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

pub trait HomeDirExt {
    /// Expands `~` at the front of the path.
    ///
    /// # Errors
    ///
    /// * Unable to find the user's home directory.
    /// * The path begins with `~user`
    fn expand_home(&self) -> Result<PathBuf, Error>;
}

impl HomeDirExt for Path {
    fn expand_home(&self) -> Result<PathBuf, Error> {
        let mut path = PathBuf::new();
        let mut components = self.components();

        match components.next() {
            Some(Component::Normal(component)) => match component.to_str() {
                Some("~") => {
                    path.push(
                        choose_base_strategy()
                            .map_err(|_| Error::NotFound)?
                            .home_dir(),
                    );
                }
                Some(s) if s.starts_with('~') => {
                    return Err(Error::NotSupported);
                }
                Some(s) => path.push(s),
                None => path.push(component),
            },
            Some(component) => path.push(component),
            None => return Ok(path),
        }

        for component in components {
            path.push(component);
        }

        Ok(path)
    }
}

impl<T> HomeDirExt for T
where
    T: AsRef<Path>,
{
    fn expand_home(&self) -> Result<PathBuf, Error> {
        self.as_ref().expand_home()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User's home directory not found")]
    NotFound,
    #[error("`~user` prefixed paths are not supported")]
    NotSupported,
}
