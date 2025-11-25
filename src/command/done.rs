use crate::command::common;
use anyhow::Result;

pub fn done<DefaultBranch: AsRef<str>>(default_branch: DefaultBranch) -> Result<()> {
    common::ensure_working_directory_is_clean()?;

    let current_branch_output = String::from_utf8(common::execute(
        "git",
        &["rev-parse", "--abbrev-ref", "HEAD"],
    )?)?;
    let current_branch = current_branch_output.trim();

    common::execute("git", &["checkout", default_branch.as_ref()])?;
    common::execute("git", &["branch", "-D", current_branch])?;

    Ok(())
}
