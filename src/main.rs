use cli::{Commands, cli};
use compiler::compiler;
use logging::{error, parser_log};

fn main() {
    let command = cli().command;
    parser_log("İşə Başlayıram");
    match command {
        Commands::Run { binary } => {
             if let Err(e) = interpreter::interpreter_file(&binary) {
                 error(e.to_string().as_str());
             }
        }
        Commands::Repl => {
            if let Err(e) = interpreter::interpreter_run_repl() {
                error(e.to_string().as_str());
            }
        }
        Commands::Build { binary } => {
            if let Err(e) = compiler(&binary) {
                error(e.to_string().as_str());
            }
        }
    }
}
