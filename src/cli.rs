use clap::Parser;

#[derive(Parser)]
pub enum Cli {
    Start,
    Done,
}
