use crate::screen::Screen;
use crate::translator::{Example, OptionDescription, Translator, Widget};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};

enum Direction {
    Left,
    Right,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Dependency {
    from: i32,
    to: i32,
}

impl PartialOrd<Self> for Dependency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dependency {
    fn cmp(&self, other: &Self) -> Ordering {
        if Ordering::Equal == self.from.cmp(&other.from) {
            self.to.cmp(&other.to)
        } else {
            self.from.cmp(&other.from)
        }
    }
}

struct Actor {
    name: Vec<char>,
    dependencies: BTreeSet<Dependency>,

    // computed position
    left: i32,
    right: i32,
    center: i32,
}

impl Actor {
    pub fn draw(&self, screen: &mut Screen, height: i32) {
        screen.draw_boxed_text(self.left, 0, self.name.clone());
        screen.draw_vertical_line(3, height - 4, self.center);
        screen.draw_boxed_text(self.left, height - 3, self.name.clone());
        screen.draw_pixel(self.center, 2, '┬');
        screen.draw_pixel(self.center, height - 3, '┴');
    }
}

struct ActorSpace {
    a: i32,
    b: i32,
    space: i32,
}

struct Message {
    from: Vec<char>,
    to: Vec<char>,
    id: i32,
    messages: Vec<Vec<char>>,

    direction: Direction,

    // computed position
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    width: i32,
    line_left: i32,
    line_right: i32,
    line_top: i32,
    line_bottom: i32,
    is_separated: bool,
    offset: i32,
}

impl Message {
    pub fn draw(&self, screen: &mut Screen) {
        if self.line_top == self.line_bottom {
            screen.draw_horizontal_line(self.line_left, self.line_right, self.line_top, None);
        } else {
            match self.direction {
                Direction::Left => {
                    screen.draw_horizontal_line(
                        self.line_right - self.offset,
                        self.line_right,
                        self.line_top,
                        None,
                    );
                    screen.draw_vertical_line(
                        self.line_top,
                        self.line_bottom,
                        self.line_right - self.offset,
                    );
                    screen.draw_horizontal_line(
                        self.line_left,
                        self.line_right - self.offset,
                        self.line_bottom,
                        None,
                    );
                    screen.draw_pixel(self.line_right - self.offset, self.line_top, '┌');
                    screen.draw_pixel(self.line_right - self.offset, self.line_bottom, '┘');
                }
                Direction::Right => {
                    screen.draw_horizontal_line(
                        self.line_left + self.offset,
                        self.line_left,
                        self.line_top,
                        None,
                    );
                    screen.draw_vertical_line(
                        self.line_top,
                        self.line_bottom,
                        self.line_left + self.offset,
                    );
                    screen.draw_horizontal_line(
                        self.line_left + self.offset,
                        self.line_right,
                        self.line_bottom,
                        None,
                    );
                    screen.draw_pixel(self.line_left + self.offset, self.line_top, '┐');
                    screen.draw_pixel(self.line_left + self.offset, self.line_bottom, '└');
                }
            }
        }

        // Tip of the arrow
        match self.direction {
            Direction::Left => screen.draw_pixel(self.line_left, self.line_bottom, '<'),
            Direction::Right => screen.draw_pixel(self.line_right, self.line_bottom, '>'),
        }

        // The message
        let mut y = self.top;
        for line in &self.messages {
            screen.draw_text(self.left, y, line.clone());
            y += 1;
        }
    }
}

struct MessageDependencies {
    messages: BTreeSet<i32>, // ids
    dependencies: BTreeSet<Dependency>,
}

struct MessageSetWithWeight {
    messages: BTreeSet<i32>,
    weight: usize,
}

impl Eq for MessageSetWithWeight {}

impl PartialEq<Self> for MessageSetWithWeight {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd<Self> for MessageSetWithWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MessageSetWithWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

pub struct Sequence {
    actors: Vec<Actor>,
    messages: Vec<Message>,

    actor_index: BTreeMap<Vec<char>, i32>,
    message_index: BTreeMap<i32, i32>,

    ascii_only: bool,

    pub output: String,
}

impl Translator for Sequence {
    fn identifier() -> String {
        "Sequence".to_string()
    }
    fn name() -> String {
        "Sequence diagram".to_string()
    }
    fn description() -> String {
        "Draw sequence diagram".to_string()
    }
    fn options() -> HashMap<&'static str, OptionDescription> {
        vec![(
            "ascii_only",
            OptionDescription {
                name: "ascii_only".to_string(),
                values: vec!["false", "true"],
                default_value: "false".to_string(),
                description: "Use the full unicode charset or only ASCII".to_string(),
                r#type: Widget::CheckBox,
            },
        )]
        .into_iter()
        .collect()
    }
    fn examples() -> Vec<Example> {
        vec![
            Example {
                title: "1-basic".to_string(),
                input: "Alice -> Bob: Hello Bob!\nAlice <- Bob: Hello Alice!".to_string(),
            },
            Example {
                title: "2-More actors".to_string(),
                #[rustfmt::skip]
                input: "Render -> Browser: BeginNavigation()\n\
Browser -> Network: URLRequest()\n\
Browser <- Network: URLResponse()\n\
Renderer <- Browser: CommitNavigation()\n\
Renderer -> Browser: DidCommitNavigation()".to_string(),
            },
            Example {
                title: "3-Actors order".to_string(),
                #[rustfmt::skip]
                input: "Actor 2 -> Actor 3: message1\n\
Actor 2 -> Actor 3: message 1\n\
Actor 1 -> Actor 2: message 2\n\
\n\
Actor 1:\n\
Actor 2:\n\
Actor 3:".to_string(),
            },
            Example {
                title: "4-Message order".to_string(),
                #[rustfmt::skip]
                input: "2) Actor 2 -> Actor 3: message1\n\
1) Actor 1 -> Actor 2: message 2\n\
\n\
Actor 1:\n\
Actor 2: 1<2\n\
Actor 3:".to_string(),
            },
            Example {
                title: "5-Message crossing".to_string(),
                #[rustfmt::skip]
                input: "1) Renderer -> Browser: Message 1\n\
2) Renderer <- Browser: Message 2\n\
\n\
Renderer: 1<2\n\
Browser: 2<1".to_string(),
            },
        ]
    }

    fn translate(_input: &str, _options: &str) -> String {
        String::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Dependency;
    #[test]
    fn test_assign() {
        let d1 = Dependency { from: 1, to: 2 };

        let d2 = d1.clone();

        assert_eq!(d1, d2);
    }
}
