use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
#[derive(Parser)]
#[command(
    name = "azcli",
    about = "Write, build and run AzLang code — all in one command!",
    disable_help_subcommand = true
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        /// E.g: output/output
        binary: String,
    },
    Version {},
}

pub fn cli() -> Cli {
    let cmd = Cli::command().help_template(
        "\x1b[36m{before-help}AzCLI — {about}\x1b[0m\n\n\
     \x1b[33mUsage:\x1b[0m {usage}\n\n\
     \x1b[32mCommands:\x1b[0m\n{subcommands}\n\n\
     \x1b[35mOptions:\x1b[0m\n{options}\n\n\
     \x1b[31mFor additional help, feel free to reach out to us!\x1b[0m\n\n\
     {after-help}",
    );
    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches);
    cli.unwrap()
}
