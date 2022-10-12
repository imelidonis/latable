mod arguments;

use arguments::LatableArgs;
use clap::Parser;

fn main() {
    let args = LatableArgs::parse();

    println!("{:?}", args);
}
