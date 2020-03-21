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

    pub fn analyze_lines(_src: &str) -> Vec<usize> {
        Vec::new()
    }

    /// Returns the index of the line into `lines`.
    pub fn lookup_line_column(&self, _pos: usize) -> (usize, usize) {
        (0, 0)
    }
}
