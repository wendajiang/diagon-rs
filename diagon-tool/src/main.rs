mod args;

use crate::args::{interactive_args, Args};
use clap::Parser;
use diagon::translator;

fn main() {
    // parse args
    let mut args = Args::parse();
    interactive_args(&mut args);

    let maybe_fn = translator::GLOBAL_FN.get(args.component.as_str());
    if maybe_fn.is_none() {
        println!("not support subcommand {}", args.component.as_str());
        return;
    }

    let (translate, _, _) = maybe_fn.unwrap();

    let output = translate(args.content.as_str(), args.options.as_str());

    println!("{}", output);
}
