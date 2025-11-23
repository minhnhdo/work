mod cli;
mod command;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let default_branch = command::common::get_default_branch()?;
    match Cli::parse() {
        Cli::Start(cli) => command::start("origin", default_branch, &cli.new_branch),
        Cli::Done => command::done(default_branch),
    }
}
