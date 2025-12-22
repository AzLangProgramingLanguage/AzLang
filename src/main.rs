use cli::{Commands, cli};
use compiler::compiler;
use interpreter::interpreter;
use logging::{error, parser_log};

fn main() {
    let mut sabuhi: &'static str = "Salam"; //5
    sabuhi = "Aleykume"; //8
    let asdas = "Salam".to_string().push('c');

    let command = cli().command;

    parser_log("İşə Başlayıram");
    match command {
        Commands::Run { binary } => {
            if let Err(e) = interpreter(&binary) {
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
