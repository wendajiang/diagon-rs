mod args;

use crate::args::{interactive_args, Args};
use clap::Parser;
use diagon::translator;

fn main() {
    // parse args
    let mut args = Args::parse();

    let maybe_fn = translator::GLOBAL_FN.get(args.component.as_str());
    if maybe_fn.is_none() {
        println!("not support subcommand {}", args.component.as_str());
        return;
    }

    let (translate, options, examples) = maybe_fn.unwrap();

    interactive_args(&mut args, options, examples);

    let output = translate(args.content.as_str(), args.options.as_str());

    println!("{}", output);
}
