// Format the output table in LaTeX

use std::fs::read_to_string;

// Formats the beginning of an environment for LaTeX.
// Examples:
//      * begin!("table") => \begin{table}
//      * begin!("tabular", "ccc") => \begin{tabular}{ccc}
macro_rules! begin {
    ($env:expr) => {
        &*format!("\\begin{{{}}}", $env)
    };
    ($env:expr, $param:expr) => {
        &*format!("\\begin{{{}}}{{{}}}", $env, $param)
    };
}

// Formats the end of an environment.
// Example:
//      * end("table") => \end(table)
macro_rules! end {
    ($env:expr) => {
        &*format!("\\end{{{}}}", $env)
    };
}

struct Warning {
    more_rows: bool,
    more_columns: bool,
    less_rows: bool,
    less_columns: bool,
    warning: String,
}

impl Warning {
    pub fn new() -> Warning {
        Warning {
            more_rows: false,
            more_columns: false,
            less_rows: false,
            less_columns: false,
            warning: String::new(),
        }
    }

    pub fn warn_more_rows(&mut self, target: usize, found: usize) {
        if self.more_rows {
            return;
        }
        self.more_rows = true;
        self.warning += format!("Passed row arg ({}). Skipped {} lines from csv.\n", target, found - target).as_str();
    }

    pub fn warn_more_columns(&mut self, target: usize, found: usize) {
        if self.more_columns {
            return;
        }
        self.more_columns = true;
        self.warning += format!("Columns should be {}. Skipped {} columns from at least one row.\n", target, found - target).as_str();
    }

    pub fn warn_less_rows(&mut self, target: usize, found: usize) {
        if self.less_rows {
            return;
        }
        self.less_rows = true;
        self.warning += format!("Passed row arg ({}) but csv has {} lines. Generate {} empty lines.\n", target, found, target - found).as_str();
    }

    pub fn warn_less_columns(&mut self, target: usize, found: usize) {
        if self.less_columns {
            return;
        }
        self.less_columns = true;
        self.warning += format!("Columns should be {}. Filled {} columns in at least one row.\n", target, target - found).as_str();
    }

    pub fn message(&self) -> &String {
        &self.warning
    }

    pub fn has_warnings(&self) -> bool {
        self.more_rows || self.more_columns || self.less_rows || self.less_columns
    }
}

pub fn format_latex_table(rows: usize, columns: usize, col_def: String) -> String {
    assert_eq!(col_def.chars().count(), columns);

    let mut table = String::new();

    // begin table environment. Adding [ht] for here-top positioning in LaTeX.
    table = table + begin!("table") + "[ht]\n";

    // begin tabular environment.
    table = table + "\t" + begin!("tabular", col_def) + "\n";

    let row = " &".repeat(columns - 1);

    // add rows. The last one doesn't have a \\ at the end.
    for _ in 0..(rows - 1) {
        table = table + "\t\t" + &row + " \\\\\n";
    }
    table = table + "\t\t" + &row + " \n";

    // end tabular environment
    table = table + "\t" + end!("tabular") + "\n";

    // end table environment
    table = table + end!("table") + "\n";

    table
}

pub fn format_latex_table_csv(rows: usize, columns: usize, col_def: String, csv_path: String) -> String {
    assert_eq!(col_def.chars().count(), columns);

    let mut table = String::new();

    // begin table environment. Adding [ht] for here-top positioning in LaTeX.
    table = table + begin!("table") + "[ht]\n";

    // begin tabular environment.
    table = table + "\t" + begin!("tabular", col_def) + "\n";

    // Read rows from csv
    let file = read_to_string(csv_path.as_str()).unwrap();
    let mut warning = Warning::new();
    let mut row_counter = 0;

    for line in file.lines() {
        let mut line_break = " \\\\";
        row_counter += 1;
        if row_counter > rows {
            continue; // skip lines, just count
        } else if row_counter == rows {
            line_break = "";
        }

        let col_counter = line.split(',').map(|s| s.trim()).count();

        let row: String;

        if col_counter > columns {
            warning.warn_more_columns(columns, col_counter);
            let mut col_vec: Vec<&str> = line.split(",").collect();
            col_vec.truncate(columns);

            row = col_vec.join(" & ");
        } else if col_counter < columns {
            warning.warn_less_columns(columns, col_counter);

            row = line.replace(',', " & ") + " &".repeat(columns - col_counter).as_str();
        } else {
            row = line.replace(',', " & ");
        }

        // write the line to the output
        table = table + "\t\t" + &row + line_break + "\n";
    }

    if row_counter > rows {
        warning.warn_more_rows(rows, row_counter);
    }

    // Fill potential rows (if csv has less rows that the row argument)
    for r in row_counter..rows {
        warning.warn_less_rows(rows, row_counter);
        let mut line_break = " \\\\";
        if r == rows - 1 {
            line_break = "";
        }

        let row = " &".repeat(columns - 1);

        // write the line to the output
        table = table + "\t\t" + &row +  line_break + "\n";
    }

    // end tabular environment
    table = table + "\t" + end!("tabular") + "\n";

    // end table environment
    table = table + end!("table") + "\n";
    if warning.has_warnings() {
        table = table + "\nWarnings:\n" + warning.message();
    }

    table
}
