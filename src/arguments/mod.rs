// The parser logic for Command-Line Arguments

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about = "A simple CLI tool to generate tables for LaTeX.")]
pub struct LatableArgs {
    /// Number of rows
    #[clap(short = 'r', long = "rows")]
    rows: u32,
    
    /// Number of columns
    #[clap(short = 'c', long = "columns")]
    columns: u32,

    /// Column Definition, eg. 'ccc' for 3 centered columns. Default: all columns centered
    /// Accepted options: l (left-align), c (center-align), r (right - align).
    #[clap(
        verbatim_doc_comment,
        hide_default_value = true,
        long = "col-def",
        value_parser = column_def_validation
    )]
    column_def: Option<String>,
}

fn column_def_validation(s: &str) -> Result<String, String> {
    if s.chars().any(|c| c != 'l' && c != 'c' && c != 'r') {
        Err(String::from("Column Definition can only contains: l, c or r]"))
    } else {
        Ok(String::from(s))
    }
}