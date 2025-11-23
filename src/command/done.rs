use crate::command::common;
use anyhow::{anyhow, Result};

pub fn done() -> Result<()> {
    let git_branch_output = String::from_utf8(common::execute("git", &["branch"])?)?;
    let current_branch = git_branch_output
        .split('\n')
        .find(|line| line.starts_with("*"))
        .ok_or(anyhow!("Failed to find active branch"))?
        .strip_prefix('*')
        .ok_or(anyhow!("Failed to strip active branch prefix"))?
        .trim();

    println!("Active branch: {current_branch}");

    Ok(())
}
