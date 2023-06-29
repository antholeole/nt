use anyhow::{anyhow, Result};
use cli::{Cli, SubCommand, MainArgs};
use export::export;
use tokio;

mod cli;
mod note;
mod db;
mod export;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = clap::Parser::parse();
    let command = args.get_subcommand();


    match command {
        SubCommand::Export(args) => export(args).await,
        SubCommand::Main(main_args) => parse_main(main_args).await
    }
}

async fn parse_main(main_args: MainArgs) -> Result<()> {
    if main_args.note.len() > 0 {
        note::create_note(main_args.note).await
    } else {
        // this should be unreachable
        Err(anyhow!("should have one main subcommand"))
    }
}
