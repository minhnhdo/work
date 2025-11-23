use crate::command::common;
use anyhow::Result;

pub fn start<Remote: AsRef<str>, DefaultBranch: AsRef<str>, NewBranch: AsRef<str>>(
    remote: Remote,
    default_branch: DefaultBranch,
    new_branch: NewBranch,
) -> Result<()> {
    common::ensure_working_directory_is_clean()?;
    common::execute("git", &["checkout", default_branch.as_ref()])?;
    common::execute("git", &["pull", remote.as_ref(), default_branch.as_ref()])?;
    common::execute("git", &["checkout", "-b", new_branch.as_ref()])?;
    Ok(())
}
