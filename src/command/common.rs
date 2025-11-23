use anyhow::{anyhow, Result};
use std::process::Command;

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
