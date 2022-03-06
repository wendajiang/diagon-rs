use clap::Parser;
use diagon::translator::{Example, OptionDescription};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

// todo subcommand pattern, e.g. diagon-rs Tree --examples --options
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long = "command",
    possible_values =
    ["Tree", "Table", "Math"])]
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
    options: &fn() -> Vec<OptionDescription>,
    examples: &fn() -> Vec<Example>,
) {
    // set default content
    let examples: Vec<Example> = examples();
    let options: Vec<OptionDescription> = options();

    let mut option_style = "";
    for op in &options {
        if op.name == *"style" {
            option_style = op.values[0];
        }
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

        for option in options {
            let selected_option = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!("Pick your {}", option.name))
                .items(&option.values)
                .default(0)
                .interact();
            args.options +=
                (option.name + "\n" + option.values[selected_option.unwrap_or_default()] + "\n")
                    .as_str();
        }
        args.options.pop();
    } else {
        let example = &examples[0];

        if args.content.is_empty() {
            args.content = example.input.clone();
        }

        if args.options.is_empty() {
            args.options = "style\n".to_string() + option_style;
        }
    }
}
