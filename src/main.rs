extern crate lazy_static;
pub mod context;
pub mod lexer;
pub mod parser;
pub mod runner;
pub mod translations;
pub mod transpiler;
pub mod utils;
pub mod validator;
use crate::context::TranspileContext;
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
pub use runner::*;
pub use translations::syntax::Syntax;
pub use transpiler::*;
pub use utils::*;
pub use validator::*;
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
    /// .AzLang kodlarını compile edib işə salır
    Build {
        /// Məs: examples/program.az
        path: String,
    },
    /// Compile edilmiş output faylını işə sal
    Run {
        /// Məs: output/output
        binary: String,
    },
}

const QARDAS_PARSE: &str = "\x1b[36m[Böyük Qardaş Parserci]:\x1b[0m";
const EMI_VALIDATOR: &str = "\x1b[33m[Dəmir Əmi Validator]:\x1b[0m";
const XALA_OPTI: &str = "\x1b[32m[Validə Xala Optimizator]:\x1b[0m";
const SISTER_TRANSP: &str = "\x1b[35m[Kiçik Bacı Tərcüməçi]:\x1b[0m";

fn qardas_parse(msg: &str) {
    println!("{} {}", QARDAS_PARSE, msg);
}

fn emi_validator(msg: &str) {
    println!("{} {}", EMI_VALIDATOR, msg);
}

fn xala_opti(msg: &str) {
    println!("{} {}", XALA_OPTI, msg);
}

fn sister_transp(msg: &str) {
    println!("{} {}", SISTER_TRANSP, msg);
}

fn qardas_parse_error(msg: &str) {
    eprintln!(
        "{} Qardaş dedi: \"Dayı, burda iş bitmədi, yenidən bax! Səbəb: {}\"",
        QARDAS_PARSE, msg
    );
}

fn emi_validator_error(msg: &str) {
    eprintln!(
        "{} Əmi xəbər verir: \"Kodun bura gəlməməli idi, bir az tərbiyə lazımdır! Problem: {}\"",
        EMI_VALIDATOR, msg
    );
}

fn xala_opti_error(msg: &str) {
    eprintln!(
        "{} Cəza gəlir! \"Burda həqiqətən problem var, düzəlməsə sənə şillə vuracam! Xəta: {}\"",
        XALA_OPTI, msg
    );
}

fn baci_transp_error(msg: &str) {
    eprintln!(
        "{} Transpilov qardaş: \"Yolda problem çıxdı, sabah səni zig-də gözləyirəm! Detal: {}\"",
        SISTER_TRANSP, msg
    );
}

fn main() -> Result<()> {
    color_eyre::install()?;

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

fn build(input_path: &str) -> Result<()> {
    let input_code = utils::read_file(input_path).map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    let syntax = Syntax::load().map_err(|e| eyre!("Syntax xətası!: {}", e))?;
    let mut ctx = TranspileContext::new();
    let tokens = lexer::Lexer::new(&input_code, &syntax).tokenize();

    println!("Tokens: {:#?}", tokens);

    let mut parser = parser::Parser::new(tokens);
    let parsed_program = parser.parse(&mut ctx).map_err(|e| {
        qardas_parse_error(&format!("Parser xətası: {}", e));
        eyre!("Parser xətası: {}", e)
    })?;

    println!("Parsed program: {:#?}", parsed_program);
    emi_validator("Yaxşı-yaxşı, sənin işini indi yoxlayıram!");
    for expr in &parsed_program.expressions {
        validator::validate_expr(expr, &mut ctx, &mut emi_validator).map_err(|e| {
            emi_validator_error(&e);
            eyre!("Validator xətası: {}", e)
        })?;
    }
    emi_validator("Əla, Dəmir Əmi razı qaldı. Kod təmizdi!");
    let mut transpiler_ctx = ctx.clone();
    let zig_code = transpiler::transpile(&parsed_program, &mut transpiler_ctx, &sister_transp)
        .map_err(|e| {
            baci_transp_error(&e);
            eyre!("Transpilasiya xətası: {}", e)
        })?;
    println!("Zig code: {}", zig_code);

    utils::write_file("output/output.zig", &zig_code)
        .map_err(|e| eyre!("Zig faylı yazıla bilmədi: {}", e))?;
    if runner::compile_and_run("output/output.zig", "output/output").is_err() {
        eprintln!("❌ Proqram işləmədi.");
    }
    Ok(())
}

fn run(binary: &str) -> Result<()> {
    use std::path::Path;
    use std::process::Command;

    let binary_path = Path::new(binary);
    if !binary_path.exists() {
        return Err(eyre!("Fayl mövcud deyil: {}", binary));
    }

    let status = Command::new(binary_path).status()?;
    if !status.success() {
        eprintln!("⚠️ Proqram icrası zamanı xəta.");
    }

    Ok(())
}

/*
qardas_parse("Gəlin, kodu yığışdırıram, hamıya salam deyirəm!");
qardas_parse("Əla! Kodu didik-didik etdim, amma başa düşdüm!");
emi_validator("Gəlim yoxlayım görüm kodun harasında fırıldaq var.");
emi_validator("Heç bir problem tapmadım... Amma tapacağım günü gözlə!");
xala_opti("Kod əlimə keçdi. İndi gör necə parıldayacaq");

xala_opti("Əla,Afərin! Səhv yoxdu, məndən sənə beş ulduz ⭐");
sister_transp("Hər şey 0-dan 1-ə keçdi. Çevirdim, çatdırdım, indi sən işlə!");
println!(
    "\x1b[1;34m[Ailə Komandası 👨‍👩‍👧‍👦]:\x1b[0m Kodun bütün ailə üzvləri tərəfindən yoxlanıldı və sevildi. Halaldı sənə!"
);
 */
