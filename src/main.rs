use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

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
    let source = SourceFile::open(opt.source)?;
    let mut lexer = Lexer::new(source.src.as_str());
    while let Some(token) = lexer.advance_token() {
        if token.kind == Whitespace {
            // continue;
        }
        let pos = source.lookup_line_column(token.range.start);
        println!("{:?}:{:?}", token, pos);
    }
    Ok(())
}
