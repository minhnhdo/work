mod cli;
mod command;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    match Cli::parse() {
        Cli::Start => command::start(),
        Cli::Done => command::done(),
    }
}
