use crate::command::common;
use anyhow::{anyhow, Result};

const CURRENT_BRANCH_PREFIX: char = '*';

pub fn done(default_branch: &str) -> Result<()> {
    common::ensure_working_directory_is_clean()?;

    let git_branch_output = String::from_utf8(common::execute("git", &["branch"])?)?;
    let current_branch = git_branch_output
        .split('\n')
        .find(|line| line.starts_with(CURRENT_BRANCH_PREFIX))
        .ok_or(anyhow!("failed to find current branch"))?
        .strip_prefix(CURRENT_BRANCH_PREFIX)
        .ok_or(anyhow!("failed to strip current branch prefix"))?
        .trim();

    common::execute("git", &["checkout", default_branch])?;
    common::execute("git", &["branch", "-D", current_branch])?;

    Ok(())
}
