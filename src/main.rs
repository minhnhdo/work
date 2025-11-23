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
        #[arg(long, help = "remote to use, defaults to upstream or origin")]
        remote: Option<String>,
        #[arg(long, help = "default branch to use, defaults to main or master")]
        default_branch: Option<String>,
        new_branch: String,
    },
    Done {
        #[arg(long, help = "default branch to use, defaults to main or master")]
        default_branch: Option<String>,
    },
}

fn main() -> Result<()> {
    match Cli::parse() {
        Cli {
            sub_command:
                SubCommand::Start {
                    remote,
                    default_branch,
                    new_branch,
                },
        } => {
            let remote = remote
                .ok_or_else(|| ())
                .or_else(|_| command::common::get_remote())?;
            let default_branch = default_branch
                .ok_or_else(|| ())
                .or_else(|_| command::common::get_default_branch())?;
            command::start(remote, default_branch, new_branch)
        }
        Cli {
            sub_command: SubCommand::Done { default_branch },
        } => {
            let default_branch = default_branch
                .ok_or_else(|| ())
                .or_else(|_| command::common::get_default_branch())?;
            command::done(default_branch)
        }
    }
}
