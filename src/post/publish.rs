use git2::{Commit, Config, Cred, PushOptions, RemoteCallbacks, Repository, Signature};
use std::path::Path;

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    Ok(repo
        .head()?
        .resolve()?
        .peel_to_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))?)
}

pub fn add_and_commit_post(
    post_path: &Path,
    repo_path: &Path,
    draft: bool,
) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let config = repo.config()?;
    let username = config
        .get_string("user.name")
        .map_err(|e| git2::Error::from_str(&format!("Failed to get user.name: {}", e)))?;
    let email = config
        .get_string("user.email")
        .map_err(|e| git2::Error::from_str(&format!("Failed to get user.email: {}", e)))?;

    let rel_path = post_path
        .strip_prefix(repo_path)
        .expect("relative path is removed");

    let mut index = repo.index()?;
    index.add_path(rel_path)?;
    index.write()?;
    let oid = index.write_tree()?;

    let signature_bot = Signature::now("jekyll-poster", "jekyll-poster")?;
    let signature = Signature::now(&username, &email)?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;

    let commit_msg = format!(
        "Add new post entry{}: {}",
        if draft { " [draft]" } else { "" },
        post_path
            .file_name()
            .expect("Got filename path")
            .to_str()
            .expect("Got filename path string")
    );
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature_bot,
        &commit_msg,
        &tree,
        &[&parent_commit],
    )?;

    // Push
    let config = Config::open_default()?;

    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_, _, _| {
        Cred::credential_helper(&config, "https://github.com", Some(&username)).map_err(|e| {
            git2::Error::from_str(&format!("Failed to call credential.helper: {}", e))
        })
    });
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote.push(
        &["refs/heads/master:refs/heads/master"],
        Some(&mut push_options),
    )?;
    Ok(())
}
