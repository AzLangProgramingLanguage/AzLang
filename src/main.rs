use cli::{Commands, cli};
use compiler::compiler;
use interpreter::interpreter;
use logging::{error, parser_log};

fn main() {
    let command = cli().command;
    parser_log("İşə Başlayıram");
    match command {
        Commands::Run { binary } => {
            let mut output = String::new();
            if let Err(e) = interpreter(&binary, &mut output) {
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
