mod args;
mod screen;
mod translator;

use args::*;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("test {} {}!", args.content, args.component);
}
