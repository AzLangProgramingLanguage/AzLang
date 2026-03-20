use cli::{Commands, cli};
use compiler::compiler;

fn main() {
    let command = cli().command;
    match command {
        Commands::Run { binary } => {
            interpreter::interpreter_file(&binary);
        }
        Commands::Repl => {
            interpreter::interpreter_run_repl();
        }
        Commands::Build { binary } => {
            compiler(&binary);
        }
    }
}
