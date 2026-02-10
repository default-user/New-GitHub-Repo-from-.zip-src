use crate::error::Zip2RepoError;
use octocrab::Octocrab;

pub async fn repo_exists(
    gh: &Octocrab,
    owner: &str,
    repo: &str,
) -> Result<bool, Zip2RepoError> {
    let r = gh.repos(owner, repo).get().await;
    match r {
        Ok(_) => Ok(true),
        Err(e) => {
            // 404 => doesn't exist, others => api error
            if e.to_string().contains("404") {
                Ok(false)
            } else {
                Err(Zip2RepoError::GithubApi(e.to_string()))
            }
        }
    }
}
