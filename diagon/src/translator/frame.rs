use crate::screen::Screen;
use crate::translator::{serialize_option, Example, OptionDescription, Translator, Widget};

#[derive(Debug, Default)]
pub struct Frame;

impl Translator for Frame {
    fn identifier() -> String {
        "Frame".to_string()
    }
    fn name() -> String {
        "Frame".to_string()
    }
    fn description() -> String {
        "Draw a box around the input with (optional) line number".to_string()
    }
    fn options() -> Vec<OptionDescription> {
        vec![
            OptionDescription {
                name: "ascii_only".to_string(),
                values: vec!["false", "true"],
                default_value: "false".to_string(),
                description: "Use the full unicode charset or only ASCII".to_string(),
                r#type: Widget::CheckBox,
            },
            OptionDescription {
                name: "line_number".to_string(),
                values: vec!["false", "true"],
                default_value: "true".to_string(),
                description: "Display the line number.".to_string(),
                r#type: Widget::CheckBox,
            },
        ]
    }
    fn examples() -> Vec<Example> {
        vec![Example {
            title: "1-Hello world".to_string(),
            #[rustfmt::skip]
            input:
            r#"#include <iostream>
using namespace std;

int main() {
    cout << "Hello, world!";
    return 0;
}           
"#.to_string(),
        }]
    }

    fn translate(input: &str, options: &str) -> String {
        let options = serialize_option(options);

        let ascii_only = if let Some(val) = options.get("ascii_only") {
            val == "true"
        } else {
            false
        };

        let line_number = if let Some(val) = options.get("line_number") {
            val == "true"
        } else {
            false
        };

        let lines = input.lines().collect::<Vec<String>>();

        let mut number_len = 0;
        let mut max_number = 1;
        while max_number <= lines.len() {
            max_number *= 10;
            number_len += 1;
        }
        let text_max_width = lines.iter().fold(0, |acc, &str| acc.max(str.len().into()));

        let (width, height, text_x) = if line_number {
            (
                number_len as i32 + text_max_width + 3,
                lines.len() as i32 + 2 + ascii_only as usize,
                number_len as i32 + 2,
            )
        } else {
            (
                number_len + text_max_width + 2,
                lines.len() + 2 + ascii_only as usize,
                1,
            )
        };

        let text_y = if ascii_only { 2 } else { 1 };

        let mut screen = Screen::new(width, height);

        // draw text
        for &line in lines {
            screen.draw_text(text_x, text_y, line);
        }

        // draw line number
        if line_number {
            for y in 0..lines.len() {
                screen.draw_text(1, text_y + 1, (y + 1).to_string().chars().collect())
            }
        }

        // draw box
        if ascii_only {
            screen.draw_horizontal_line(1, width - 2, 0, Some('_'));
            screen.draw_horizontal_line(1, width - 2, height - 1, Some('_'));
            screen.draw_vertical_line(1, height - 1, 0);
            screen.draw_vertical_line(1, height - 1, width - 1);
        } else {
            screen.draw_box(0, 0, width, height);
        }

        // draw the line number separator
        if line_number {
            if ascii_only {
                screen.draw_vertical_line(1, height - 1, number_len as i32 + 1);
            } else {
                screen.draw_pixel(number_len as i32 + 1, 0, '┬');
                screen.draw_vertical_line(1, height - 1, number_len as i32 + 1);
                screen.draw_pixel(number_len as i32 + 1, height - 1, '┴');
            }
        }

        screen.as_string()
    }
}
