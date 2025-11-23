use crate::command;
use clap::Parser;

#[derive(Parser)]
pub enum Cli {
    Start(command::start::Cli),
    Done,
}
