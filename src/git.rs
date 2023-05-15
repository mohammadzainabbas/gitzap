use git2::{Repository, Signature};
use crate::utils::GitInfo;

pub fn add_commit_push(repo_path: &str, commit_message: &str, git_info: &GitInfo) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;

    let mut index = repo.index()?;

    // add all changes
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let oid = index.write_tree()?;
    let signature = Signature::now(&
