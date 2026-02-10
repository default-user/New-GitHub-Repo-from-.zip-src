use crate::error::Zip2RepoError;
use git2::{Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature};
use std::path::Path;

pub fn init_and_commit(repo_root: &Path, message: &str) -> Result<Repository, Zip2RepoError> {
    let repo = Repository::init(repo_root).map_err(|e| Zip2RepoError::Io(e.to_string()))?;

    {
        let mut idx = repo
            .index()
            .map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        idx.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
            .map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        idx.write()
            .map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        let tree_id = idx
            .write_tree()
            .map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        let tree = repo
            .find_tree(tree_id)
            .map_err(|e| Zip2RepoError::Io(e.to_string()))?;

        let sig = Signature::now("zip2repo", "zip2repo@local")
            .map_err(|e| Zip2RepoError::Io(e.to_string()))?;
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])
            .map_err(|e| Zip2RepoError::Io(e.to_string()))?;
    }

    Ok(repo)
}

pub fn add_remote_and_push(
    repo: &Repository,
    token: &str,
    owner: &str,
    name: &str,
) -> Result<(), Zip2RepoError> {
    let url = format!("https://github.com/{owner}/{name}.git");
    if repo.find_remote("origin").is_err() {
        repo.remote("origin", &url)
            .map_err(|e| Zip2RepoError::GitPush(e.to_string()))?;
    }

    let mut cb = RemoteCallbacks::new();
    let tok = token.to_string();
    cb.credentials(move |_url, _username, _allowed| {
        Cred::userpass_plaintext("x-access-token", &tok)
    });

    let mut push_opts = PushOptions::new();
    push_opts.remote_callbacks(cb);

    let mut remote = repo
        .find_remote("origin")
        .map_err(|e| Zip2RepoError::GitPush(e.to_string()))?;

    // Try pushing master as main first, fall back to master:master
    let push_result =
        remote.push(&["refs/heads/master:refs/heads/main"], Some(&mut push_opts));

    if let Err(first_err) = push_result {
        // Rebuild callbacks for retry (consumed by first attempt)
        let mut cb2 = RemoteCallbacks::new();
        let tok2 = token.to_string();
        cb2.credentials(move |_url, _username, _allowed| {
            Cred::userpass_plaintext("x-access-token", &tok2)
        });
        let mut push_opts2 = PushOptions::new();
        push_opts2.remote_callbacks(cb2);

        remote
            .push(
                &["refs/heads/master:refs/heads/master"],
                Some(&mut push_opts2),
            )
            .map_err(|e| {
                Zip2RepoError::GitPush(format!(
                    "push as main failed: {first_err}; push as master also failed: {e}"
                ))
            })?;
    }

    Ok(())
}
