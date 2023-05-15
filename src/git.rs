use git2::{Commit, IndexAddOption, Repository, Signature, Cred, PushOptions};
use crate::utils::GitInfo;

pub fn add_commit_push(repo_path: &str, commit_message: &str, git_info: &GitInfo) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;

    let mut index = repo.index()?;

    // add all changes
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let oid = index.write_tree()?;
    let signature = Signature::now(&git_info.user_name, &git_info.user_email)?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;

    // committing
    repo.commit(Some("HEAD"), &signature, &signature, commit_message, &tree, &[&parent_commit])?;

    let mut remote = repo.find_remote(&git_info.remote_name)?;
    let credentials = utils::get_git_token()
        .map(|token| Cred::userpass_plaintext(&git_info.name, &token))
        .unwrap_or_else(|| Err(git2::Error::from_str("No Git token provided in config file")))?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| credentials);

    let mut options = PushOptions::new();
    options.remote_callbacks(callbacks);

    // pushing
    // repo.find_remote(&git_info.remote_name)?.push(&[format!("refs/heads/{}:refs/heads/{}", git_info.branch_name, git_info.branch_name)], None);
    println!("Pushing changes to remote: {}", &git_info.remote_name);
    remote.push(&[format!("refs/heads/{}:refs/heads/{}", git_info.branch_name, git_info.branch_name)], Some(&options))?;
    // if let Err(e) = repo.find_remote(&git_info.remote_name)?.push(&[format!("refs/heads/{}:refs/heads/{}", git_info.branch_name, git_info.branch_name)], None) {
    //     eprintln!("Failed to push changes: {}", e);
    // }    

    Ok(())
}

fn find_last_commit(repo: &Repository) -> Result<git2::Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}
