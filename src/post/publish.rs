use std::path::Path;

use git2::Error;
use git2::Repository;

pub fn commit_post(post_path: &Path, repo_path: &Path) -> Result<Repository, Error> {
    let repo = Repository::open(repo_path)?;
    let username = repo.config().unwrap().get_str("user.name").unwrap();
    //  println!("{username}");

    // let default_branch = repo.find_remote("origin").unwrap().default_branch().unwrap_or("main")?;

    // repo.index().unwrap().add_path(post_path).unwrap()?;

    // repo.commit(default_branch, "jekyll-poster", "jekyll-poster", message, tree, parents)
    Ok(repo)
}
