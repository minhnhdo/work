mod command;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "work")]
struct Cli {
    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[command(arg_required_else_help = true)]
    Start {
        #[arg(long)]
        remote: Option<String>,
        new_branch: String,
    },
    Done,
}

fn main() -> Result<()> {
    match Cli::parse() {
        Cli {
            sub_command: SubCommand::Start { remote, new_branch },
        } => {
            let remote = remote
                .ok_or_else(|| ())
                .or_else(|_| command::common::get_remote())?;
            let default_branch = command::common::get_default_branch()?;
            command::start(remote, default_branch, new_branch)
        }
        Cli {
            sub_command: SubCommand::Done,
        } => {
            let default_branch = command::common::get_default_branch()?;
            command::done(default_branch)
        }
    }
}
