use cli::{Commands, cli};
use compiler::compiler;

fn main() {
    let command = cli().command;
    match command {
        Commands::Build { binary } => {
            compiler(&binary).unwrap_or_else(|err| {
                err.display();
                std::process::exit(err.code());
            });
        }
        Commands::Version {} => {
            println!("Version: 0.0.2");
        }
    }
}
