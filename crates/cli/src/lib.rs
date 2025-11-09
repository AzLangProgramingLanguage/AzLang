use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
#[derive(Parser)]
#[command(
    name = "azcli",
    about = "AzLang ilə yaz, tərtib et, işə sal — bir əmrlə!",
    disable_help_subcommand = true
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// .az faylını işə salır.
    Run {
        /// Məs: output/output
        binary: String,
    },

    Build {
        /// Məs: output/output
        binary: String,
    },
}

pub fn cli() -> Cli {
    let cmd = Cli::command().help_template(
        "\x1b[36m{before-help}AzCLI — {about}\x1b[0m\n\n\
     \x1b[33mİstifadə:\x1b[0m {usage}\n\n\
     \x1b[32mƏmrlər:\x1b[0m\n{subcommands}\n\n\
     \x1b[35mSeçimlər:\x1b[0m\n{options}\n\n\
     \x1b[31mYardım üçün əlavə suallarınız varsa bizimlə əlaqə saxlayın!\x1b[0m\n\n\
     {after-help}",
    );
    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches);
    cli.unwrap()
}
