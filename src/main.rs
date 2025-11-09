use cli::{cli, Commands};
use compiler::compiler;
use interpreter::interpreter;
use logging::parser_log;
fn main() {
    let command = cli().command;
    match command {
        Commands::Run { binary } => {
            parser_log(&binary);
            interpreter(&binary);
        }
        Commands::Build { binary } => {
            parser_log(&binary);
            compiler(&binary);
        }
    }
}
