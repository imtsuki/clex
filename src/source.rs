//! This module contains source-file-related functionality.

use crate::token::*;
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use termcolor::{Color::Green, ColorChoice, ColorSpec, StandardStream, WriteColor};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone)]
pub struct SourceFile {
    /// The path of the file
    pub path: PathBuf,
    /// The complete source code
    pub src: Rc<String>,
    /// Locations of line beginnings
    pub lines: Rc<Vec<usize>>,
}

impl SourceFile {
    pub fn open(path: PathBuf) -> Result<Self> {
        let mut buf = String::new();
        File::open(&path)?.read_to_string(&mut buf)?;

        // Remove UTF-8 BOM, if any.
        if buf.starts_with("\u{feff}") {
            buf.drain(..3);
        }
        // Append \n if necessary.
        if !buf.ends_with("\n") {
            buf.push_str("\n");
        }
        let lines = Self::analyze_lines(buf.as_str());
        Ok(SourceFile {
            path: path,
            src: Rc::new(buf),
            lines: Rc::new(lines),
        })
    }

    /// Find out all line breaks.
    pub fn analyze_lines(src: &str) -> Vec<usize> {
        let mut lines = vec![0];
        lines.extend(src.chars().enumerate().filter_map(|(i, c)| {
            if c == '\n' {
                Some(i + 1)
            } else {
                None
            }
        }));
        lines
    }

    /// Returns the line and column position corresponding to the given `char_pos`.
    pub fn lookup_line_column(&self, char_pos: usize) -> (usize, usize) {
        let line = self
            .lines
            .binary_search(&char_pos)
            .map_or_else(|e| e - 1, |t| t);
        let column = char_pos - self.lines[line];
        (line, column)
    }

    pub fn get_line(&self, line: usize) -> String {
        let start = self.lines[line];
        let end = self.lines[line + 1];
        self.src.chars().skip(start).take(end - start - 1).collect()
    }

    pub fn display_error_hint(&self, token: &Token) -> Result<()> {
        if let Error(error_kind) = token.kind {
            let mut stderr = StandardStream::stderr(ColorChoice::Auto);

            let (line, column) = self.lookup_line_column(token.char_range.start);
            let line_src = self.get_line(line);
            writeln!(&mut stderr, "{}", line_src)?;

            let leading_spaces = line_src.chars().take(column).fold(0, |acc, c| {
                acc + UnicodeWidthChar::width(c).unwrap_or_default()
            });
            write!(&mut stderr, "{: <1$}", "", leading_spaces)?;

            let token_display_width =
                UnicodeWidthStr::width(&self.src.as_str()[token.byte_range.clone()]);

            stderr.set_color(ColorSpec::new().set_fg(Some(Green)).set_bold(true))?;
            writeln!(
                &mut stderr,
                "{} {:?}",
                std::iter::repeat('^')
                    .take(token_display_width)
                    .collect::<String>(),
                error_kind
            )?;

            stderr.reset()?;
        }
        Ok(())
    }
}
