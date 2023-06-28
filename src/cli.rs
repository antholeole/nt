use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "test", version = "0.1.0")]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<SubCommand>,

    /// creates a new note at the current time.
    #[clap(flatten)]
    pub main: MainArgs,
}

impl Cli {
    pub fn get_subcommand(self) -> SubCommand {
        self.command.unwrap_or(SubCommand::Main(self.main))
    }
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum SubCommand {
    Main(MainArgs),
    Export,
}

#[derive(Debug, clap::Args, PartialEq)]
#[clap(group(
    clap::ArgGroup::new("commands")
        .required(true)
        .args(&["note"]),
))]
pub struct MainArgs {
    #[clap(short, long, exclusive=true, num_args=1..)]
    pub note: Vec<String>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_allow_main_args_at_top_level() {
        let invocation = vec!["nt", "-n", "hello"];

        let cli: Cli = clap::Parser::parse_from(invocation);

        assert!(match cli.get_subcommand() {
            SubCommand::Main(_) => true,
            _ => false,
        });
    }


    #[test]
    fn should_allow_subcommand() {
        let invocation = vec!["nt", "export"];

        let cli: Cli = clap::Parser::parse_from(invocation);

        assert!(match cli.get_subcommand() {
            SubCommand::Export => true,
            _ => false
        });
    }

    #[test]
    fn should_allow_any_number_of_args_for_dash_n() {
        let invocation = vec!["nt", "-n", "one", "two", "three"];

        let cli: Cli = clap::Parser::parse_from(invocation);

        assert!(match cli.get_subcommand() {
            SubCommand::Main(args) => args.note.len() == 3,
            _ => false
        });
    }
}
