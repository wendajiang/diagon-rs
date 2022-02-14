use crate::translator::{Example, OptionDescription, Translator, Widget};
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
struct Style {
    width: [i32; 3],
    height: [i32; 4],
    charset: [&'static str; 22],
}

static STYLES: Lazy<HashMap<&'static str, Style>> = Lazy::new(|| {
    vec![
        (
            "ascii",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "+", "-", "+", "+", "|", "|", "|", "+", "-", "+", "+", "|", "|", "|", "+", "-",
                    "+", "+", "+", "-", "+", "+",
                ],
            },
        ),
        (
            "ascii rounded",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    ".", "-", "+", ".", "|", "|", "|", "|", "-", "+", "|", "|", "|", "|", "|", "-",
                    "+", "|", "'", "-", "+", "'",
                ],
            },
        ),
        (
            "ascii with header 1",
            Style {
                width: [2, 1, 2],
                height: [1, 1, 1, 1],
                charset: [
                    "|=", "=", "=", "=|", "| ", "|", " |", "|=", "=", "=", "=|", " |", "|", "| ",
                    " +", "-", "+", "+ ", " +", "-", "+", "+ ",
                ],
            },
        ),
        (
            "ascii with header 2",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "=", "=", "=", "=", "|", "|", "|", "=", "=", "=", "=", "|", "|", "|", "+", "-",
                    "+", "+", "+", "-", "+", "+",
                ],
            },
        ),
        (
            "ascii light header",
            Style {
                width: [0, 1, 0],
                height: [0, 1, 0, 0],
                charset: [
                    "", "", "", "", "", " ", "", "", "-", " ", "", "", " ", "", "", "", "", "", "",
                    "", "", "",
                ],
            },
        ),
        (
            "ascii light header/separator",
            Style {
                width: [0, 1, 0],
                height: [0, 1, 0, 0],
                charset: [
                    "", "", "", "", "", "|", "", "", "-", "|", "", "", "|", "", "", "", "", "", "",
                    "", "", "",
                ],
            },
        ),
        (
            "ascii light header/separator/border",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 0, 1],
                charset: [
                    "+", "-", "+", "+", "|", "|", "|", "+", "-", "+", "|", "|", "|", "|", "|", "",
                    "|", "|", "+", "-", "+", "+",
                ],
            },
        ),
        (
            "ascii light separator/border",
            Style {
                width: [1, 1, 1],
                height: [1, 0, 0, 1],
                charset: [
                    "+", "-", "+", "+", "|", "|", "|", "", "", "", "", "|", "|", "|", "|", "", "|",
                    "|", "+", "-", "+", "+",
                ],
            },
        ),
        (
            "ascii light border",
            Style {
                width: [1, 1, 1],
                height: [1, 0, 0, 1],
                charset: [
                    "+", "-", "-", "+", "|", " ", "|", "", "", "", "", "|", " ", "|", "|", "", " ",
                    "|", "+", "-", "-", "+",
                ],
            },
        ),
        (
            "unicode",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "┌", "─", "┬", "┐", "│", "│", "│", "├", "─", "┼", "┤", "│", "│", "│", "├", "─",
                    "┼", "┤", "└", "─", "┴", "┘",
                ],
            },
        ),
        (
            "unicode rounded",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "╭", "─", "┬", "╮", "│", "│", "│", "├", "─", "┼", "┤", "│", "│", "│", "├", "─",
                    "┼", "┤", "╰", "─", "┴", "╯",
                ],
            },
        ),
        (
            "unicode bold",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "┏", "━", "┳", "┓", "┃", "┃", "┃", "┣", "━", "╋", "┫", "┃", "┃", "┃", "┣", "━",
                    "╋", "┫", "┗", "━", "┻", "┛",
                ],
            },
        ),
        (
            "unicode double",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "╔", "═", "╦", "╗", "║", "║", "║", "╠", "═", "╬", "╣", "║", "║", "║", "╠", "═",
                    "╬", "╣", "╚", "═", "╩", "╝",
                ],
            },
        ),
        (
            "unicode with bold header",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "┏", "━", "┳", "┓", "┃", "┃", "┃", "┡", "━", "╇", "┩", "│", "│", "│", "├", "─",
                    "┼", "┤", "└", "─", "┴", "┘",
                ],
            },
        ),
        (
            "unicode with double header",
            Style {
                width: [1, 1, 1],
                height: [1, 1, 1, 1],
                charset: [
                    "╒", "═", "╤", "╕", "│", "│", "│", "╞", "═", "╪", "╡", "│", "│", "│", "├", "─",
                    "┼", "┤", "└", "─", "┴", "┘",
                ],
            },
        ),
        (
            "unicode cells",
            Style {
                width: [1, 2, 1],
                height: [1, 2, 2, 1],
                charset: [
                    "╭",
                    "─",
                    "╮╭",
                    "╮",
                    "│",
                    "││",
                    "│",
                    "╰╭",
                    "──",
                    "╯╰╮╭",
                    "╯╮",
                    "│",
                    "││",
                    "│",
                    "╰╭",
                    "──",
                    "╯╰╮╭",
                    "╯╮",
                    "╰",
                    "─",
                    "╯╰",
                    "╯",
                ],
            },
        ),
        (
            "unicode cells 2",
            Style {
                width: [2, 2, 2],
                height: [2, 2, 2, 2],
                charset: [
                    "╭─│╭",
                    "──",
                    "──╮╭",
                    "─╮╮│",
                    "││",
                    "││",
                    "││",
                    "│╰│╭",
                    "──",
                    "╯╰╮╭",
                    "╯│╮│",
                    "││",
                    "││",
                    "││",
                    "│╰│╭",
                    "──",
                    "╯╰╮╭",
                    "╯│╮│",
                    "│╰╰─",
                    "──",
                    "╯╰──",
                    "╯│─╯",
                ],
            },
        ),
        (
            "conceptual",
            Style {
                width: [1, 2, 1],
                height: [1, 1, 1, 1],
                charset: [
                    " ", "_", "  ", " ", "/", "\\/", "\\", "\\", "_", "/\\", "/", "/", "\\/", "\\",
                    "\\", "_", "/\\", "/", "\\", "_", "/\\", "/",
                ],
            },
        ),
    ]
    .into_iter()
    .collect()
});

pub struct Table {}

impl Translator for Table {
    fn identifier() -> String {
        "Table".to_string()
    }
    fn name() -> String {
        "Table".to_string()
    }
    fn description() -> String {
        "Draw table".to_string()
    }
    fn options() -> HashMap<&'static str, OptionDescription> {
        vec![(
            "style",
            OptionDescription {
                name: "style".to_string(),
                values: vec![
                    "unicode",
                    "unicode rounded",
                    "unicode bold",
                    "unicode double",
                    "unicode with bold header",
                    "unicode with double header",
                    "unicode cells",
                    "unicode cells 2",
                    "ascii",
                    "ascii rounded",
                    "ascii with header 1",
                    "ascii with header 2",
                    "ascii light header",
                    "ascii light header/separator",
                    "ascii light header/separator/border",
                    "ascii light separator/border",
                    "ascii light border",
                    "conceptual",
                ],
                default_value: "unicode".to_string(),
                description: "The style of the table.".to_string(),
                r#type: Widget::Combobox,
            },
        )]
        .into_iter()
        .collect()
    }
    fn examples() -> Vec<Example> {
        vec![Example {
            title: "1-simple".to_string(),
            #[rustfmt::skip]
                input: "Column 1,Column 2,Column 3\n\
C++,Web,Assembly\n\
Javascript,CSS,HTM".to_string(),
        }]
    }
    fn translate(_input: &str, _options: &str) -> String {
        String::default()
    }
}
