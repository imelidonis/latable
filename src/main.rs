mod arguments;
mod formatter;

use arguments::LatableArgs;
use arguments::column_def::ColumnDef;
use clap::Parser;

fn main() {
    let args = LatableArgs::parse();

    if args.is_user_defined() {
        match args.col_def_size_validation() {
            Ok(_) => (),
            Err(msg) => {
                eprintln!("error: {}", msg);
                std::process::exit(1);
            }
        }
    }

    let col_def = match args.get_column_def() {
        ColumnDef::Centered => "c".repeat(args.get_columns()),
        ColumnDef::Left => "l".repeat(args.get_columns()),
        ColumnDef::Right => "r".repeat(args.get_columns()),
        ColumnDef::Custom(def) => def,
    };

    println!("LaTeX table generated:\n");
    println!("{}", formatter::format_latex_table(args.get_rows(), args.get_columns(), col_def));
}
