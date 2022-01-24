use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub component: String,

    #[clap(long)]
    pub content: String,
}

pub fn interactive_args(args: &mut Args) {}
