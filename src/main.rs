mod arguments;

use arguments::LatableArgs;
use clap::Parser;

fn main() {
    let args = LatableArgs::parse();

    if args.has_col_def() {
        match args.col_def_size_validation() {
            Ok(_) => (),
            Err(msg) => {
                eprintln!("error: {}", msg);
                std::process::exit(1);
            }
        }
    }

    let col_def = args.get_column_def().unwrap_or_else(
        // if column definition wasn't provided, all columns are centered.
        || "c".repeat(args.get_columns())
    );

    println!("{:?}", args);
    println!("{}", col_def);
}
