use ansi_term::Color::{Red, White};
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[cfg(test)]
#[macro_use]
mod test_utils;

mod lexer;
mod source;
mod token;

use lexer::Lexer;
use source::SourceFile;
use token::*;

#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Opt {
    #[structopt(parse(from_os_str), help = "The source code file")]
    source: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();

    let source = SourceFile::open(opt.source)?;
    let lexer = Lexer::new(source.src.as_str());
    for token in lexer.iter() {
        let (line, column) = source.lookup_line_column(token.char_range.start);
        if let Error(error_kind) = token.kind {
            println!(
                "{} {} {}",
                White.bold().paint(format!(
                    "{}:{}:{}:",
                    source.path.to_str().unwrap(),
                    line + 1,
                    column + 1,
                )),
                Red.bold().paint("error:"),
                White.bold().paint(format!("{:?}", error_kind))
            );
            source.display_error_hint(&token);
        } else {
            println!(
                "{}:{}:{}: {}",
                source.path.to_str().unwrap(),
                line + 1,
                column + 1,
                token,
            );
        }
    }
    Ok(())
}
