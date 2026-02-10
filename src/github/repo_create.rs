use crate::error::Zip2RepoError;
use octocrab::models::Repository;
use octocrab::Octocrab;
use serde_json::json;

pub async fn create_repo_user(
    gh: &Octocrab,
    repo: &str,
    private: bool,
) -> Result<Repository, Zip2RepoError> {
    let body = json!({
        "name": repo,
        "private": private,
        "auto_init": false,
    });
    gh.post::<_, Repository>("/user/repos", Some(&body))
        .await
        .map_err(|e| Zip2RepoError::Permission(e.to_string()))
}

pub async fn create_repo_org(
    gh: &Octocrab,
    org: &str,
    repo: &str,
    private: bool,
) -> Result<Repository, Zip2RepoError> {
    let route = format!("/orgs/{org}/repos");
    let body = json!({
        "name": repo,
        "private": private,
        "auto_init": false,
    });
    gh.post::<_, Repository>(&route, Some(&body))
        .await
        .map_err(|e| Zip2RepoError::Permission(e.to_string()))
}
