use std::path::PathBuf;

#[must_use]
pub fn config_dir() -> Option<PathBuf> {
    Some("/tmp/".into())
}
