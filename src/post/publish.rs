use git2::{Commit, ObjectType, Repository, Signature, Direction};
use std::path::Path;

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

pub fn add_and_commit_post(post_path: &Path, repo_path: &Path) -> Result<(), git2::Error> {
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
    let oid = index.write_tree()?;
    let signature_bot = Signature::now("jekyll-poster", "jekyll-poster")?;
    let signature = Signature::now(&username, &email)?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;

    let commit_msg = format!(
        "Add new post entry: {}",
        post_path.file_name().unwrap().to_str().unwrap()
    );
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature_bot,
        &commit_msg,
        &tree,
        &[&parent_commit],
    )?;

    let mut remote = repo.find_remote("origin")?;
    remote.connect(Direction::Push)?;
    remote.push(&["refs/heads/main:refs/heads/main"], None)?;

    Ok(())
}
