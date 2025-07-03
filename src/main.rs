extern crate lazy_static;
pub mod context;
pub mod lexer;
pub mod parser;
pub mod runner;
pub mod translations;
pub mod transpiler;
use std::env;
use std::path::PathBuf;
pub mod utils;
pub mod validator;
use crate::{
    context::TranspileContext,
    parser::{Expr, ast::Type},
};
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

/* fn xala_opti_error(msg: &str) {
    eprintln!(
        "{} Cəza gəlir! \"Burda həqiqətən problem var, düzəlməsə sənə şillə vuracam! Xəta: {}\"",
        XALA_OPTI, msg
    );
} */

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
#[allow(hidden_glob_reexports)]
fn build(input_path: &str) -> Result<()> {
    qardas_parse("Başladım kodu yığmağa, hər kəsə salamlar!");

    let input_code = utils::read_file(input_path).map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    let syntax = Syntax::load().map_err(|e| eyre!("Syntax xətası!: {}", e))?;
    let mut ctx = TranspileContext::new();
    let tokens = lexer::Lexer::new(&input_code, &syntax).tokenize();

    /* println!("Tokens: {:#?}", tokens); */

    let mut parser = parser::Parser::new(tokens);
    let mut parsed_program = parser.parse().map_err(|e| {
        qardas_parse_error(&format!("Parser xətası: {}", e));
        eyre!("Parser xətası: {}", e)
    })?;
    qardas_parse("Kodun sintaksisini uğurla anladım, davam edirəm...");
    emi_validator("Kodun qaydalarını yoxlayıram, diqqətlə...");
    let mut validator_ctx = ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validator::validate_expr(expr, &mut validator_ctx, &mut emi_validator).map_err(|e| {
            emi_validator_error(&e);
            eyre!("Validator xətası: {}", e)
        })?;

        validate_top_level_expr(expr).map_err(|e| {
            emi_validator_error(&e);
            eyre!("Validator xətası: {}", e)
        })?;
    }
    /* println!("Parser {:#?}", parsed_program); */
    emi_validator("Validator tapmadı problem, amma yenə diqqətliyəm.");
    xala_opti("Kodun optimizasiyası başladı, görüm nə dərəcədə təmizdir.");
    xala_opti("Optimizasiya tamamlandı! Kod parıldayır, ulduzlar səninlə ⭐");
    let zig_code =
        transpiler::transpile(&parsed_program, &mut ctx, &sister_transp).map_err(|e| {
            baci_transp_error(&e);
            eyre!("Transpilasiya xətası: {}", e)
        })?;

    sister_transp("Hər şey 0-dan 1-ə keçdi. Çevirdim, çatdırdım, indi sən işlə!");
    println!(
        "\x1b[1;34m[Yığım Komandası 👨‍👩‍👧‍👦]:\x1b[0m Kodun bütün ailə üzvləri tərəfindən yoxlanıldı və sevildi. Halaldı sənə!"
    );

    let mut temp_path = env::temp_dir();
    temp_path.push("azlang_output.zig");
    utils::write_file(temp_path.to_str().unwrap(), &zig_code)
        .map_err(|e| eyre!("Zig faylı yazıla bilmədi: {}", e))?;
    if runner::build(temp_path.to_str().unwrap(), input_path).is_err() {
        eprintln!("❌ Proqram işləmədi.");
    }

    Ok(())
}

fn run(input_path: &str) -> Result<()> {
    qardas_parse("Proqramı işə salıram, uğurlar!");

    let input_code = utils::read_file(input_path).map_err(|e| eyre!("Fayl oxunmadı!: {}", e))?;

    let syntax = Syntax::load().map_err(|e| eyre!("Syntax xətası!: {}", e))?;
    let mut ctx = TranspileContext::new();
    let tokens = lexer::Lexer::new(&input_code, &syntax).tokenize();

    /* println!("Tokens: {:#?}", tokens); */

    let mut parser = parser::Parser::new(tokens);
    let mut parsed_program = parser.parse().map_err(|e| {
        qardas_parse_error(&format!("Parser xətası: {}", e));
        eyre!("Parser xətası: {}", e)
    })?;
    qardas_parse("Kodun sintaksisi yoxlandı, icra üçün hazıram.");
    emi_validator("İcra öncəsi yoxlamalar davam edir...");
    let mut validator_ctx = ValidatorContext::new();
    for expr in parsed_program.expressions.iter_mut() {
        validator::validate_expr(expr, &mut validator_ctx, &mut emi_validator).map_err(|e| {
            emi_validator_error(&e);
            eyre!("Validator xətası: {}", e)
        })?;

        validate_top_level_expr(expr).map_err(|e| {
            emi_validator_error(&e);
            eyre!("Validator xətası: {}", e)
        })?;
    }
    /* println!("Parser {:#?}", parsed_program); */
    emi_validator("İcra üçün heç bir problem tapılmadı.");
    xala_opti("Kod işləməyə hazırdır, başlayıram.");
    xala_opti("İcra tamamlandı, nəticələri yoxla!");
    let zig_code =
        transpiler::transpile(&parsed_program, &mut ctx, &sister_transp).map_err(|e| {
            baci_transp_error(&e);
            eyre!("Transpilasiya xətası: {}", e)
        })?;

    sister_transp("Transpilasiya uğurla başa çatdı, proqram işə düşür.");
    println!(
        "\x1b[1;34m[Ailə Komandası 👨‍👩‍👧‍👦]:\x1b[0m Kodun bütün ailə üzvləri tərəfindən yoxlanıldı və sevildi. Halaldı sənə!"
    );
    let mut temp_path = env::temp_dir();
    temp_path.push("azlang_output.zig");
    utils::write_file(temp_path.to_str().unwrap(), &zig_code)
        .map_err(|e| eyre!("Zig faylı yazıla bilmədi: {}", e))?;
    if runner::runner(temp_path.to_str().unwrap()).is_err() {
        eprintln!("❌ Proqram işləmədi.");
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

pub fn validate_top_level_expr(expr: &mut Expr) -> Result<(), String> {
    if let Expr::FunctionCall {
        name,
        return_type: Some(t),
        ..
    } = expr
    {
        if *t != Type::Void {
            return Err(format!(
                "Funksiya '{}' bir dəyər qaytarır ({:?}), amma nəticə istifadə olunmur. Onu dəyişənə mənimsətməlisiniz.",
                name, t
            ));
        }
    }
    Ok(())
}
