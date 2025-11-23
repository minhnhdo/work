mod cli;
mod command;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    match Cli::parse() {
        Cli::Start(cli) => command::start("origin", "main", &cli.new_branch),
        Cli::Done => command::done(),
    }
}
