use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

// todo subcommand pattern, e.g. diagon-rs Tree --examples --options
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    pub component: String,

    #[clap(long)]
    pub content: String,

    #[clap(long, default_value = "style\nunicode 1")]
    pub options: String,

    #[clap(long, short)]
    pub interaction: bool,
}

pub fn interactive_args(args: &mut Args) {
    if args.interaction {
        let select_style = vec![
            "unicode 1",
            "unicode 2",
            "ASCII 1",
            "ASCII 2",
            "ASCII 3",
            "unicode right top",
            "unicode right center",
            "unicode right bottom",
        ];
        let selected_style = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your style")
            .items(&select_style)
            .default(0)
            .interact();
        args.options = "style\n".to_string() + select_style[selected_style.unwrap_or_default()];
    }
}
