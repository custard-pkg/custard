use std::path::Path;

use eyre::Result;
use git_config::{File, Source};
use git_url::Url;

/// Find the origin remote of the current project.
///
/// Taken from gitoxide.
pub fn find_origin_remote(repo: &Path) -> Result<Option<String>> {
    let non_bare = repo.join(".git").join("config");
    let local = Source::Local;
    let config = File::from_path_no_includes(non_bare.as_path(), local)
        .or_else(|_| File::from_path_no_includes(repo.join("config").as_path(), local))?;
    Ok(config
        .string("remote", Some("origin"), "url")
        .map(|url| Url::from_bytes(url.as_ref()))
        .transpose()?
        .map(|option| option.to_bstring().try_into().unwrap()))
}
