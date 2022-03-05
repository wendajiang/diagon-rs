use crate::screen::Screen;
use crate::translator::{serialize_option, Example, OptionDescription, Translator, Widget};
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Style {
    width: [i32; 3],
    height: [i32; 4],
    charset: [&'static str; 22],
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: [1, 1, 1],
            height: [1, 1, 1, 1],
            charset: [
                "┌", "─", "┬", "┐", "│", "│", "│", "├", "─", "┼", "┤", "│", "│", "│", "├", "─",
                "┼", "┤", "└", "─", "┴", "┘",
            ],
        }
    }
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
Javascript,CSS,HTML".to_string(),
        }]
    }
    fn translate(input: &str, options: &str) -> String {
        let options = serialize_option(options);

        // Style
        let style_option = if let Some(str) = options.get("style") {
            str.clone()
        } else {
            "unicode"
        };
        let style = if let Some(st) = STYLES.get(style_option) {
            st.clone()
        } else {
            Style::default()
        };

        // Separator.
        let separator = if let Some(sep) = options.get("separator") {
            sep.clone()
        } else {
            ","
        };

        // Parse data.
        let mut data: Vec<Vec<String>> = input
            .lines()
            .map(|line| line.split(separator).map(|str| str.to_string()).collect())
            .collect();

        // Compute row/line count
        let row_count = data.len();
        let mut column_count = 0;
        for line in &data {
            column_count = column_count.max(line.len());
        }

        // Uniformize the number of cells per lines
        data.iter_mut()
            .map(|v| v.resize(column_count, String::default()));

        // Compute row_width
        let mut column_width: Vec<usize> = vec![0; column_count];
        for line in &data {
            for (i, _) in line.iter().enumerate() {
                column_width[i] = column_width[i].max(line[i].len());
            }
        }

        // compute sum_row_width
        let column_width_global: usize = column_width.iter().sum();

        // compute screen dimension
        let width = style.width[0]
            + style.width[1] * ((column_count - 1) as i32)
            + style.width[2]
            + column_width_global as i32;
        let height = style.height[0]
            + style.height[1]
            + style.height[2] * (row_count - 2) as i32
            + style.height[3]
            + row_count as i32;

        let mut screen = Screen::new(width, height);
        // Draw Table

        let mut long_y = 0;
        for y in 0..row_count {
            let last_row = (y == row_count - 1);
            let mut long_x = 0;

            let cell_top = long_y + style.height[2.min(y)];
            let cell_bottom = cell_top + 1;

            for x in 0..column_count {
                let last_column = (x == column_count - 1);

                let top_char = if y == 0 {
                    1
                } else if y == 1 {
                    8
                } else {
                    15
                };
                let left_char = if y == 0 {
                    if x == 0 {
                        4
                    } else {
                        5
                    }
                } else if x == 0 {
                    11
                } else {
                    12
                };
                let right_char = if y == 0 { 6 } else { 13 };
                let bottom_char = 19;
                let top_left_char = if y == 0 {
                    if x == 0 {
                        0
                    } else {
                        2
                    }
                } else if y == 1 {
                    if x == 0 {
                        7
                    } else {
                        9
                    }
                } else if x == 0 {
                    14
                } else {
                    16
                };
                let top_right_char = if y == 0 {
                    3
                } else if y == 1 {
                    10
                } else {
                    17
                };
                let bottom_left_char = if x == 0 { 18 } else { 20 };
                let bottom_right_char = 21;
                let cell_left = long_x + style.width[1.min(x)];
                let cell_right = cell_left + column_width[x] as i32;

                // println!(
                //     "{} {} {} {} {} {} {} {}",
                //     top_char,
                //     left_char,
                //     right_char,
                //     bottom_char,
                //     top_left_char,
                //     top_right_char,
                //     bottom_left_char,
                //     bottom_right_char
                // );

                // draw top
                {
                    let mut i = 0;
                    for yy in long_y..cell_top {
                        for xx in cell_left..cell_right {
                            screen.draw_pixel(
                                xx,
                                yy,
                                style.charset[top_char].chars().nth(i).unwrap(),
                            );
                        }
                        i += 1;
                    }
                }

                // draw down
                if last_row {
                    let mut i = 0;
                    for yy in cell_bottom..height {
                        for xx in cell_left..cell_right {
                            screen.draw_pixel(
                                xx,
                                yy,
                                style.charset[bottom_char].chars().nth(i).unwrap(),
                            );
                        }
                        i += 1;
                    }
                }

                // draw left
                for yy in cell_top..cell_bottom {
                    screen.draw_text(long_x, yy, style.charset[left_char].chars().collect());
                }

                // draw right
                if last_column {
                    for yy in cell_top..cell_bottom {
                        screen.draw_text(
                            cell_right,
                            yy,
                            style.charset[right_char].chars().collect(),
                        );
                    }
                }

                // draw left/top
                {
                    let mut i = 0;
                    for yy in long_y..cell_top {
                        for xx in long_x..cell_left {
                            screen.draw_pixel(
                                xx,
                                yy,
                                style.charset[top_left_char].chars().nth(i).unwrap(),
                            );
                            i += 1;
                        }
                    }
                }

                // draw right/top
                if last_column {
                    let mut i = 0;
                    for yy in long_y..cell_top {
                        for xx in cell_right..width {
                            screen.draw_pixel(
                                xx,
                                yy,
                                style.charset[top_right_char].chars().nth(i).unwrap(),
                            );
                            i += 1;
                        }
                    }
                }

                // draw left/bottom
                if last_row {
                    let mut i = 0;
                    for yy in cell_bottom..height {
                        for xx in long_x..cell_left {
                            screen.draw_pixel(
                                xx,
                                yy,
                                style.charset[bottom_left_char].chars().nth(i).unwrap(),
                            );
                            i += 1;
                        }
                    }
                }

                // draw right/bottom
                if last_row && last_column {
                    let mut i = 0;
                    for yy in cell_bottom..height {
                        for xx in cell_right..width {
                            screen.draw_pixel(
                                xx,
                                yy,
                                style.charset[bottom_right_char].chars().nth(i).unwrap(),
                            );
                            i += 1;
                        }
                    }
                }

                // draw text
                screen.draw_text(cell_left, cell_top, data[y][x].chars().collect());
                long_x = cell_right;
            }

            long_y = cell_bottom;
        }

        screen.as_string()
    }
}
