use clap::Parser;
use diagon::translator::OptionDescription;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::collections::HashMap;

// todo subcommand pattern, e.g. diagon-rs Tree --examples --options
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long = "command",
    possible_values =
    ["Tree", "Table"])]
    pub component: String,

    #[clap(long)]
    pub content: String,

    #[clap(long, default_value = "style\nunicode 1")]
    pub options: String,

    #[clap(long, short)]
    pub interaction: bool,
}

pub fn interactive_args(
    args: &mut Args,
    options: &fn() -> HashMap<&'static str, OptionDescription>,
) {
    let mp: HashMap<&'static str, OptionDescription> = options();
    let option_style = if let Some(opt) = mp.get("style") {
        opt.values.clone()
    } else {
        Vec::new()
    };
    if args.interaction {
        let selected_style = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your style")
            .items(&option_style)
            .default(0)
            .interact();
        args.options = "style\n".to_string() + option_style[selected_style.unwrap_or_default()];
    }
}
