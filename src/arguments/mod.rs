// The parser logic for Command-Line Arguments

pub mod column_def;

use clap::Parser;
use column_def::ColumnDef;

#[derive(Debug, Parser)]
#[clap(author, version, about = "A simple CLI tool to generate tables for LaTeX.")]
pub struct LatableArgs {
    /// Number of rows
    #[clap(short = 'r', long = "rows", value_parser = dimension_check)]
    rows: usize,

    /// Number of columns
    #[clap(short = 'c', long = "columns", value_parser = dimension_check)]
    columns: usize,

    // FIXME: better doc
    /// Definition for each column. Provide either a rule for all columns or a
    /// sequence with l (left-aligned) | c (center-aligned) | r (right-aligned)
    /// for each column.
    /// [possible values: centered, left, right, <sequence>]
    #[clap(
        value_enum,
        verbatim_doc_comment,
        long = "col-def",
        value_parser = column_def_check,
        default_value_t = ColumnDef::Centered
    )]
    column_def: ColumnDef,
}

impl LatableArgs {
    pub fn get_column_def(&self) -> ColumnDef {
        self.column_def.clone()
    }

    pub fn get_columns(&self) -> usize {
        self.columns
    }

    #[allow(dead_code)]
    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn is_user_defined(&self) -> bool {
        matches!(self.column_def, ColumnDef::Custom(_))
    }

    pub fn col_def_size_validation(&self) -> Result<(), String> {
        match &self.column_def {
            ColumnDef::Centered | ColumnDef::Left | ColumnDef::Right => {
                Ok(())
            },
            // The column definition should contain the same number of columns
            ColumnDef::Custom(col_def) => {
                if col_def.chars().count() == self.columns {
                    Ok(())
                } else {
                    Err(format!(
                        "The column definition should contain the same number of columns. Expected: {} found: {}",
                        self.columns,
                        col_def.chars().count()
                    ))
                }
            }
        }
    }
}

fn column_def_check(s: &str) -> Result<ColumnDef, String> {
    match &s[..] {
        "centered" => {
            Ok(ColumnDef::Centered)
        },
        "left" => {
            Ok(ColumnDef::Left)
        },
        "right" => {
            Ok(ColumnDef::Right)
        },
        def if def.chars().all(|c| c == 'l' || c == 'c' || c == 'r' ) => {
            Ok(ColumnDef::Custom(String::from(def)))
        },
        _ => Err(String::from("possible values: centered, left, right, <sequence of 'l', 'c' and 'r'>"))
    }
}

fn dimension_check(s: &str) -> Result<usize, String> {
    let dim: usize = s
        .parse()
        .map_err(|_| format!("'{}' isn't a valid dimension", s))?;

    if dim > 0 {
        Ok(dim)
    } else {
        Err(String::from("Should have at least 1 cell"))
    }
}
