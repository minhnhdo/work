use crate::command::common;
use anyhow::Result;

pub fn start() -> Result<()> {
    let branch_output_buf = common::execute("git", &["branch"])?;
    let branch_output = String::from_utf8(branch_output_buf)?;
    println!("{branch_output}");
    Ok(())
}
