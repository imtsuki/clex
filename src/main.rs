use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use termcolor::{
    Color::{Red, White},
    ColorChoice, ColorSpec, StandardStream, WriteColor,
};

#[cfg(test)]
#[macro_use]
mod test_utils;

mod lexer;
mod source;
mod stats;
mod token;

use lexer::Lexer;
use source::SourceFile;
use stats::Statistics;
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

    let mut stats = Statistics::default();

    let source = SourceFile::open(opt.source)?;
    let lexer = Lexer::new(source.src.as_str());
    for token in lexer.iter() {
        let (line, column) = source.lookup_line_column(token.char_range.start);
        if let Error(error_kind) = token.kind {
            let mut stderr = StandardStream::stderr(ColorChoice::Auto);
            stderr.set_color(ColorSpec::new().set_fg(Some(White)).set_bold(true))?;
            write!(
                &mut stderr,
                "{}:{}:{}: ",
                source.path.to_str().unwrap(),
                line + 1,
                column + 1,
            )?;
            stderr.set_color(ColorSpec::new().set_fg(Some(Red)).set_bold(true))?;
            write!(&mut stderr, "error: ")?;
            stderr.set_color(ColorSpec::new().set_fg(Some(White)).set_bold(true))?;
            writeln!(&mut stderr, "{:?}", error_kind)?;
            stderr.reset()?;
            source.display_error_hint(&token)?;
        } else {
            println!(
                "{}:{}:{}: {}",
                source.path.to_str().unwrap(),
                line + 1,
                column + 1,
                token,
            );
        }
        stats.track(&token);
    }

    println!(
        "{} error{} generated.",
        stats.errors,
        if let 0 | 1 = stats.errors { "" } else { "s" }
    );

    println!("\nStatistics: \n{}", stats);
    Ok(())
}
