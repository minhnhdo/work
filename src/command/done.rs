use crate::command::common;
use anyhow::Result;

pub fn done<Remote: AsRef<str>, DefaultBranch: AsRef<str>>(
    remote: Remote,
    default_branch: DefaultBranch,
) -> Result<()> {
    common::ensure_working_directory_is_clean()?;

    let current_branch_output = String::from_utf8(common::execute(
        "git",
        &["rev-parse", "--abbrev-ref", "HEAD"],
    )?)?;
    let current_branch = current_branch_output.trim();

    common::execute("git", &["checkout", default_branch.as_ref()])?;
    common::execute("git", &["pull", remote.as_ref(), default_branch.as_ref()])?;
    common::execute("git", &["branch", "-D", current_branch])?;

    Ok(())
}
