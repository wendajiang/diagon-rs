use clap::Parser;
use diagon::translator::{Example, OptionDescription};
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

    #[clap(long, default_value = "")]
    pub content: String,

    #[clap(long, default_value = "")]
    pub options: String,

    #[clap(long, short)]
    pub interaction: bool,
}

pub fn interactive_args(
    args: &mut Args,
    options: &fn() -> HashMap<&'static str, OptionDescription>,
    examples: &fn() -> Vec<Example>,
) {
    let mp: HashMap<&'static str, OptionDescription> = options();
    let option_style = if let Some(opt) = mp.get("style") {
        opt.values.clone()
    } else {
        Vec::new()
    };

    // set default content
    let examples: Vec<Example> = examples();
    let example = &examples[0];

    if args.content.is_empty() {
        args.content = example.input.clone();
    }

    if args.interaction {
        let selected_examples = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your example")
            .items(examples.as_slice())
            .default(0)
            .interact();
        args.content = examples[selected_examples.unwrap_or_default()]
            .input
            .clone();

        let selected_style = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your style")
            .items(&option_style)
            .default(0)
            .interact();
        args.options = "style\n".to_string() + option_style[selected_style.unwrap_or_default()];
    }
}
