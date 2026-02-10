mod cli;
mod error;
mod exit_codes;
mod logging;

mod auth;
mod github;
mod scaffold;
mod zip;

use clap::Parser;
use cli::Cli;
use error::Zip2RepoError;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    logging::init(cli.verbose);

    let code = match run(cli).await {
        Ok(code) => code,
        Err((e, code)) => {
            eprintln!("{e}");
            code
        }
    };
    std::process::exit(code);
}

async fn run(cli: Cli) -> Result<i32, (String, i32)> {
    use exit_codes::*;

    if !(cli.private ^ cli.public) {
        return Err((
            "Must set exactly one of --private or --public".into(),
            FAIL_CONFIG_OR_USAGE,
        ));
    }
    let private = cli.private;

    let token = auth::acquire_token(cli.auth).map_err(|e| map_err(e))?;

    // 1) load zip bytes
    let bytes = zip::extract::load_zip_bytes(&cli.zip)
        .await
        .map_err(|e| map_err(e))?;

    // 2) open + validate
    let archive = zip::extract::open_zip(bytes.clone()).map_err(|e| map_err(e))?;
    zip::validate::validate_zip(archive).map_err(|e| map_err(e))?;

    // 3) extract
    let zip2 = zip::extract::open_zip(bytes).map_err(|e| map_err(e))?;
    let (_td, root) = zip::extract::extract_to_temp(zip2).map_err(|e| map_err(e))?;

    // 4) detect stack + scaffold
    let effective_stack = scaffold::detect::resolve_stack(&cli.stack, &root);
    scaffold::write::scaffold_minimum(&root, &effective_stack).map_err(|e| map_err(e))?;

    if cli.dry_run {
        println!(
            "DRY RUN OK: would create/push {}/{} (private={}) from extracted root {:?}",
            cli.owner, cli.repo, private, root
        );
        return Ok(SUCCESS_ARTIFACTS_ONLY);
    }

    // 5) github preflight + create if needed
    let gh = github::api::client(&token);

    let exists = github::preflight::repo_exists(&gh, &cli.owner, &cli.repo)
        .await
        .map_err(|e| map_err(e))?;

    if !exists {
        // Attempt org create first; fall back to user create
        let created = github::repo_create::create_repo_org(&gh, &cli.owner, &cli.repo, private)
            .await
            .or_else(|_| {
                // Block on async: use a nested approach via tokio
                // Actually we're already in async context, so just return the future
                // We need to handle this differently since or_else is sync for Result
                Err(Zip2RepoError::Permission("org create failed".into()))
            });

        if created.is_err() {
            github::repo_create::create_repo_user(&gh, &cli.repo, private)
                .await
                .map_err(|e| (e.to_string(), FAIL_PERMISSION))?;
        }
    }

    // 6) git init + commit + push
    let repo = github::push::init_and_commit(&root, "Initial import via zip2repo")
        .map_err(|e| (e.to_string(), FAIL_IO))?;
    github::push::add_remote_and_push(&repo, &token, &cli.owner, &cli.repo)
        .map_err(|e| (e.to_string(), FAIL_GIT_PUSH))?;

    // 7) settings
    if cli.apply_settings {
        let check_names = vec!["build", "test"];
        let result =
            github::settings::apply_and_verify(&gh, &cli.owner, &cli.repo, &check_names)
                .await
                .map_err(|e| map_err(e))?;

        if result.verified {
            return Ok(SUCCESS_FULL);
        }
        // Not fully verified => artifacts only
        return Ok(SUCCESS_ARTIFACTS_ONLY);
    }

    Ok(SUCCESS_ARTIFACTS_ONLY)
}

fn map_err(e: Zip2RepoError) -> (String, i32) {
    use exit_codes::*;
    match e {
        Zip2RepoError::Usage(s) => (s, FAIL_CONFIG_OR_USAGE),
        Zip2RepoError::Permission(s) => (s, FAIL_PERMISSION),
        Zip2RepoError::GithubApi(s) => (s, FAIL_GITHUB_API),
        Zip2RepoError::GitPush(s) => (s, FAIL_GIT_PUSH),
        Zip2RepoError::ZipValidation(s) => (s, FAIL_ZIP_VALIDATION),
        Zip2RepoError::Network(s) => (s, FAIL_NETWORK_FETCH),
        Zip2RepoError::Io(s) => (s, FAIL_IO),
    }
}
