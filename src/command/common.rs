use anyhow::{anyhow, Result};
use std::process::Command;

const DEFAULT_BRANCH_MAIN: &str = "main";
const DEFAULT_BRANCH_MASTER: &str = "master";

const DEFAULT_REMOTE_UPSTREAM: &str = "upstream";
const DEFAULT_REMOTE_ORIGIN: &str = "origin";

pub fn execute(program: &str, args: &[&str]) -> Result<Vec<u8>> {
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => Ok(output.stdout),
        Ok(output) => match String::from_utf8(output.stderr) {
            Ok(error) => Err(anyhow!("{program} returned error: {error}")),
            Err(error) => Err(anyhow!(
                "{program} returned error, but failed to convert its stderr to String: {error}"
            )),
        },
        Err(error) => Err(anyhow!(
            "failed to execute {program} with args {args:?}: {error}"
        )),
    }
}

pub fn ensure_working_directory_is_clean() -> Result<()> {
    let git_status_output = String::from_utf8(execute("git", &["status", "--porcelain"])?)?;
    if !git_status_output.trim().is_empty() {
        return Err(anyhow!("working directory is not clean"));
    }
    Ok(())
}

pub fn get_default_branch() -> Result<String> {
    let git_branch_output = String::from_utf8(execute("git", &["branch"])?)?;
    let branches = git_branch_output
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>();
    if branches.contains(&DEFAULT_BRANCH_MASTER) {
        return Ok(DEFAULT_BRANCH_MASTER.to_string());
    }
    if branches.contains(&DEFAULT_BRANCH_MAIN) {
        return Ok(DEFAULT_BRANCH_MAIN.to_string());
    }
    Err(anyhow!("no default branch found"))
}

pub fn get_remote() -> Result<String> {
    let git_remote_output = String::from_utf8(execute("git", &["remote"])?)?;
    let remotes = git_remote_output
        .split('\n')
        .map(|line| line.trim())
        .collect::<Vec<_>>();
    if remotes.contains(&DEFAULT_REMOTE_UPSTREAM) {
        return Ok(DEFAULT_REMOTE_UPSTREAM.to_string());
    }
    if remotes.contains(&DEFAULT_REMOTE_ORIGIN) {
        return Ok(DEFAULT_REMOTE_ORIGIN.to_string());
    }
    if remotes.len() == 1 {
        return Ok(remotes[0].to_string());
    }
    Err(anyhow!(
        "multiple unconventional remotes found, please specify which one to use with --remote"
    ))
}
