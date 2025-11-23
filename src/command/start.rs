use crate::command::common;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub new_branch: String,
}

pub fn start(upstream: &str, default_branch: &str, new_branch: &str) -> Result<()> {
    common::ensure_working_directory_is_clean()?;
    common::execute("git", &["checkout", default_branch])?;
    common::execute("git", &["pull", upstream, default_branch])?;
    common::execute("git", &["checkout", "-b", new_branch])?;
    Ok(())
}
