// The parser logic for Command-Line Arguments

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about = "A simple CLI tool to generate tables for LaTeX.")]
pub struct LatableArgs {
    /// Number of rows
    #[clap(short = 'r', long = "rows")]
    rows: usize,
    
    /// Number of columns
    #[clap(short = 'c', long = "columns")]
    columns: usize,

    /// Column Definition, eg. 'ccc' for 3 centered columns. Default: all columns centered
    /// Accepted options: l (left-aligned), c (center-aligned), r (right-aligned).
    #[clap(
        verbatim_doc_comment,
        hide_default_value = true,
        long = "col-def",
        value_parser = column_def_validation
    )]
    column_def: Option<String>,
}

impl LatableArgs {
    pub fn get_column_def(&self) -> Option<String> {
        self.column_def.clone()
    }

    pub fn get_columns(&self) -> usize {
        self.columns
    }

    #[allow(dead_code)]
    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn has_col_def(&self) -> bool {
        self.column_def.is_some()
    }

    pub fn col_def_size_validation(&self) -> Result<(), String> {
        match &self.column_def {
            // The column definition should contain the same number of columns
            Some(col_def) => if col_def.chars().count() == self.columns {
                Ok(())
            } else {
                Err(format!(
                    "The column definition should contain the same number of columns. Expected: {} found: {}",
                    self.columns,
                    col_def.chars().count()
                ))
            },
            None => Err("Column Definition not found".to_string())
        }
    }
}

fn column_def_validation(s: &str) -> Result<String, String> {
    if s.chars().any(|c| c != 'l' && c != 'c' && c != 'r') {
        Err(String::from("Column Definition can contain only: l, c or r"))
    } else {
        Ok(String::from(s))
    }
}
