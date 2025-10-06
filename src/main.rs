use std::{env, panic};

use bumpalo::Bump;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
mod lexer;
mod parser;
mod runner;
mod translations;
mod utils;
pub mod validator;
use limit_alloc::Limit;
use runner::Runner;
use std::alloc::System;
use validator::ValidatorContext;
pub use validator::validate_expr;

#[global_allocator]
static A: Limit<System> = Limit::new(256_000_000, System);

#[derive(Parser)]
#[command(
    name = "azcli",
    about = "AzLang ilə yaz, tərtib et, işə sal — bir əmrlə!",
    disable_help_subcommand = true
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// .az faylını işə salır.
    Run {
        /// Məs: output/output
        binary: String,
    },
}

/* const MAX_RAM_MB: u64 = 1024;
 */
/* fn xala_opti_error(msg: &str) {
    eprintln!(
        "{} Cəza gəlir! \"Burda həqiqətən problem var, düzəlməsə sənə şillə vuracam! Xəta: {}\"",
        XALA_OPTI, msg
    );
} */

#[macro_export]
macro_rules! dd {
    ($x:expr) => {
        println!("{:#?}", $x); // daha rahat debug üçün Debug trait istifadə et
        std::process::exit(0);
    };
}
fn main() -> Result<()> {
    color_eyre::install()?;

    panic::set_hook(Box::new(|panic_info| {
        println!("❌ Xüsusi Xəta: Yaddaş limiti keçildi!");
        println!("Əlavə məlumat: {panic_info}");
    }));
    let mut cmd = Cli::command();
    cmd = cmd.help_template(
        "\x1b[36m{before-help}AzCLI — {about}\x1b[0m\n\n\
         \x1b[33mİstifadə:\x1b[0m {usage}\n\n\
         \x1b[32mƏmrlər:\x1b[0m\n{subcommands}\n\n\
         \x1b[35mSeçimlər:\x1b[0m\n{options}\n\n\
         \x1b[31mYardım üçün əlavə suallarınız varsa bizimlə əlaqə saxlayın!\x1b[0m\n\n\
         {after-help}",
    );
    let matches = cmd.get_matches();
    let cli = Cli::from_arg_matches(&matches)?;

    match cli.command {
        Commands::Run { binary } => run(&binary)?,
    }

    Ok(())
}

/* fn emi_validator_error(msg: &str) {
    eprintln!(
        "{EMI_VALIDATOR} Əmi xəbər verir: \"Kodun bura gəlməməli idi, bir az tərbiyə lazımdır! Problem: {msg}\"",
    );
} */
const QARDAS_PARSE: &str = "\x1b[36m[Böyük Qardaş Parserci]:\x1b[0m";
const EMI_VALIDATOR: &str = "\x1b[33m[Dəmir Əmi Validator]:\x1b[0m";
/* const XALA_OPTI: &str = "\x1b[32m[Validə Xala Optimizator]:\x1b[0m";

const SISTER_TRANSP: &str = "\x1b[35m[Kiçik Bacı Tərcüməçi]:\x1b[0m";
 */
fn qardas_parse(msg: &str) {
    println!("{QARDAS_PARSE} {msg}");
}

fn emi_validator(msg: &str) {
    println!("{EMI_VALIDATOR} {msg}");
}

/* fn xala_opti(msg: &str) {
    println!("{XALA_OPTI} {msg}");
}

fn sister_transp(msg: &str) {
    println!("{SISTER_TRANSP} {msg}");
} */

fn qardas_parse_error(msg: &str) {
    eprintln!("{QARDAS_PARSE} Qardaş dedi: \"Dayı, burda iş bitmədi, yenidən bax! Səbəb: {msg}\"",);
}

fn run(input_path: &str) -> Result<()> {
    qardas_parse("Proqramı işə salıram, uğurlar!");
    let stk_code = utils::read_file("program1.az").map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    let full_code =
        utils::read_file_with_imports(input_path).map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;
    let mut tokens = lexer::Lexer::new(&stk_code).tokenize();
    let user_tokens = lexer::Lexer::new(&full_code).tokenize();

    tokens.extend(user_tokens);

    let mut parser = parser::Parser::new(&mut tokens);
    let mut parsed_program = parser.parse().map_err(|e| {
        qardas_parse_error(&format!("Parser xətası: {e}"));
        eyre!("Parser xətası: {e}")
    })?;

    /* Validator */

    let mut validator_ctx = ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validator::validate_expr(expr, &mut validator_ctx, &mut emi_validator)
            .map_err(|e| eyre!("Validator xətası: {e}"))?;
    }

    /* Cleaner */
    drop(validator_ctx);
    let mut bump = Bump::new();
    let mut interpretator_ctx = Runner::new(&mut bump);
    Runner::run(&mut interpretator_ctx, parsed_program);
    /* Intepretator. */

    Ok(())
}
