pub mod utils;
use std::{env, panic};

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use color_eyre::eyre::{eyre, Result};
mod lexer;
mod parser;
pub mod translations;
pub mod validator;
pub use utils::*;
pub use validator::validate_expr;

use limit_alloc::Limit;
use std::alloc::System;

pub mod runner;
pub mod transpiler;
use crate::{transpiler::TranspileContext, validator::ValidatorContext};

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
    /// AzLang kodlarını çevirir.
    Build {
        /// Məs: examples/program.az
        path: String,
    },
    /// .az faylını işə salır.
    Run {
        /// Məs: output/output
        binary: String,
    },
}

const MAX_RAM_MB: u64 = 1024;

/* fn xala_opti_error(msg: &str) {
    eprintln!(
        "{} Cəza gəlir! \"Burda həqiqətən problem var, düzəlməsə sənə şillə vuracam! Xəta: {}\"",
        XALA_OPTI, msg
    );
} */

fn main() -> Result<()> {
    color_eyre::install()?;

    panic::set_hook(Box::new(|panic_info| {
        println!("❌ Xüsusi Xəta: Yaddaş limiti keçildi!");
        println!("Əlavə məlumat: {}", panic_info);
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
        Commands::Build { path } => build(&path)?,
        Commands::Run { binary } => run(&binary)?,
    }

    Ok(())
}

fn emi_validator_error(msg: &str) {
    eprintln!(
        "{EMI_VALIDATOR} Əmi xəbər verir: \"Kodun bura gəlməməli idi, bir az tərbiyə lazımdır! Problem: {msg}\"",
    );
}
const QARDAS_PARSE: &str = "\x1b[36m[Böyük Qardaş Parserci]:\x1b[0m";
const EMI_VALIDATOR: &str = "\x1b[33m[Dəmir Əmi Validator]:\x1b[0m";
const XALA_OPTI: &str = "\x1b[32m[Validə Xala Optimizator]:\x1b[0m";

const SISTER_TRANSP: &str = "\x1b[35m[Kiçik Bacı Tərcüməçi]:\x1b[0m";

fn qardas_parse(msg: &str) {
    println!("{QARDAS_PARSE} {msg}");
}

fn emi_validator(msg: &str) {
    println!("{EMI_VALIDATOR} {msg}");
}

fn xala_opti(msg: &str) {
    println!("{XALA_OPTI} {msg}");
}

fn sister_transp(msg: &str) {
    println!("{SISTER_TRANSP} {msg}");
}

fn qardas_parse_error(msg: &str) {
    eprintln!("{QARDAS_PARSE} Qardaş dedi: \"Dayı, burda iş bitmədi, yenidən bax! Səbəb: {msg}\"",);
}

#[allow(hidden_glob_reexports)]
fn build(input_path: &str) -> Result<()> {
    qardas_parse("Başladım kodu yığmağa, hər kəsə salamlar!");

    let input_code = utils::read_file(input_path).map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    let tokens = lexer::Lexer::new(&input_code).tokenize();
    dbg!(tokens);
    /* println!("Tokens: {:#?}", tokens); */

    // let mut parser = parser::Parser::new(tokens);
    // let mut parsed_program = parser.parse().map_err(|e| {
    //     qardas_parse_error(&format!("Parser xətası: {e}"));
    //     eyre!("Parser xətası: {e}")
    // })?;
    // for expr in parsed_program.expressions.iter_mut() {
    //     validator::validate_expr(expr, &mut validator_ctx, &mut emi_validator).map_err(|e| {
    //         emi_validator_error(&e);
    //         eyre!("Validator xətası: {e}")
    //     })?;

    //     validate_top_level_expr(expr).map_err(|e| {
    //         emi_validator_error(&e);
    //         eyre!("Validator xətası: {e}")
    //     })?;
    // }
    /* println!("Parser {:#?}", parsed_program); */
    // let zig_code =
    //     transpiler::transpile(&parsed_program, &mut ctx, &sister_transp).map_err(|e| {
    //         baci_transp_error(&e);
    //         eyre!("Transpilasiya xətası: {e}")
    //     })?;

    Ok(())
}

fn run(input_path: &str) -> Result<()> {
    qardas_parse("Proqramı işə salıram, uğurlar!");
    let input_code = utils::read_file(input_path).map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    let tokens = lexer::Lexer::new(&input_code).tokenize();
    println!("Tokens: {tokens:#?}");

    let mut parser = parser::Parser::new(tokens);
    let mut parsed_program = parser.parse().map_err(|e| {
        qardas_parse_error(&format!("Parser xətası: {e}"));
        eyre!("Parser xətası: {e}")
    })?;

    let mut validator_ctx = ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validator::validate_expr(expr, &mut validator_ctx, &mut emi_validator)
            .map_err(|e| eyre!("Validator xətası: {e}"))?;
    }

    let mut ctx = TranspileContext::new();
    let zig_code = ctx.transpile(&parsed_program);
    let mut temp_path = env::temp_dir();
    temp_path.push("azlang_output.zig");
    utils::write_file(temp_path.to_str().unwrap(), &zig_code)
        .map_err(|e| eyre!("Zig faylı yazıla bilmədi: {}", e))?;
    if runner::runner(temp_path.to_str().unwrap()).is_err() {
        eprintln!("❌ Proqram işləmədi.");
    }
    dbg!(&zig_code);

    Ok(())
}
