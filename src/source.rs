//! This module contains source-file-related functionality.

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

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
        let lines = Self::analyze_lines(buf.as_str());
        Ok(SourceFile {
            path: path,
            src: Rc::new(buf),
            lines: Rc::new(lines),
        })
    }

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
}
