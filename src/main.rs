use cli::{Commands, cli};
//use compiler::compiler;

fn main() {
    let command = cli().command;
    match command {
        Commands::Run { binary } => {
            interpreter::interpreter_file(&binary).unwrap_or_else(|err| {
                err.display();
                std::process::exit(err.code());
            });
        }
        Commands::Repl => {
            interpreter::interpreter_run_repl().unwrap_or_else(|err| {
                err.display();
                std::process::exit(err.code());
            });
        }
        Commands::Build { binary } => { /* compiler(&binary).unwrap_or_else(|err| { err.display(); std::process::exit(err.code()); }); */
        }
    }
}
