// Format the output table in LaTeX

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