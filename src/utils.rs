use crate::Cli;

#![allow(unused)]
trait GitUtils {
    fn is_git_repo(path: String) -> bool;
}

impl GitUtils for Cli {
    fn is_git_repo(path: String) -> bool {
        todo!()
    }
}
