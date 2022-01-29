mod args;
mod screen;
mod translator;

use crate::translator::global_fn;
use args::*;
use clap::Parser;

enum SubCommand {
    Tree,
}

fn main() {
    let args = Args::parse();
    let global_call = global_fn();
    let maybe_fn = global_call.get("Tree");
    if maybe_fn.is_none() {
        println!("not support subcommand tree");
        return;
    }

    let (translate, options, examples) = maybe_fn.unwrap();

    println!("test {} {}!", args.content, args.component);
}
