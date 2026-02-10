use thiserror::Error;

#[derive(Debug, Error)]
pub enum Zip2RepoError {
    #[error("usage/config error: {0}")]
    Usage(String),

    #[error("permission denied: {0}")]
    Permission(String),

    #[error("github api error: {0}")]
    GithubApi(String),

    #[error("git push error: {0}")]
    GitPush(String),

    #[error("zip validation failed: {0}")]
    ZipValidation(String),

    #[error("network fetch failed: {0}")]
    Network(String),

    #[error("io error: {0}")]
    Io(String),
}
