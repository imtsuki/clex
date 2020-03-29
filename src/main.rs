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
    let lexer = Lexer::new(source.src.as_str());
    for token in lexer.iter() {
        let pos = source.lookup_line_column(token.char_range.start);
        println!(
            "{} {}:{}:{}",
            token,
            source.path.to_str().unwrap(),
            pos.0,
            pos.1
        );
    }
    Ok(())
}
