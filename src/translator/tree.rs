use crate::translator::{serialize_option, Example, OptionDescription, Translator};
use std::collections::HashMap;
use std::ptr;
use std::ptr::{addr_of_mut, null_mut};

struct Node {
    content: Vec<char>,
    parent: *mut Node,
    children: Vec<*mut Node>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            content: vec![],
            parent: ptr::null_mut(),
            children: vec![],
        }
    }
}

struct Tree {
    root: *mut Node,
}

impl Tree {
    pub fn new() -> Self {
        let node = Box::new(Node::new());
        Self {
            root: Box::leak(node),
        }
    }
}

#[derive(Debug)]
struct Line {
    spaces: i32,
    content: Vec<char>,
    tree: *mut Node,
}

impl Line {
    pub fn new() -> Self {
        Self {
            spaces: 0,
            content: vec![],
            tree: ptr::null_mut(),
        }
    }
}

#[derive(Clone, Copy)]
enum Align {
    Top,
    Center,
    Bottom,
}

struct DisplayTree {
    entrance: usize,
    content: Vec<Vec<char>>,
}

impl DisplayTree {
    pub fn new() -> Self {
        Self {
            entrance: 0,
            content: vec![],
        }
    }
}

pub enum PrintStyle {
    Unicode1,
    Unicode2,
    ASCII1,
    ASCII2,
    ASCII3,
}

// (middle char, end char)
fn get_char_by_print_style(style: PrintStyle) -> (String, String) {
    match style {
        PrintStyle::Unicode1 => (" ├─".to_string(), " └─".to_string()),
        PrintStyle::Unicode2 => (" ├──".to_string(), " └──".to_string()),
        PrintStyle::ASCII1 => (" +-".to_string(), " `-".to_string()),
        PrintStyle::ASCII2 => (" +--".to_string(), " `--".to_string()),
        PrintStyle::ASCII3 => (" |--".to_string(), " `--".to_string()),
    }
}

fn print_tree_by_print_style(print_style: PrintStyle, tree: Tree) -> String {
    let node = tree.root;
    if node.is_null() {
        return "".to_string();
    }
    let (middle, end) = get_char_by_print_style(print_style);
    let mut output = String::new();
    unsafe {
        for child in &(*node).children {
            let content = unsafe { (*(*child)).content.clone() };
            output.push_str((String::from_iter(content) + "\n").as_str());
            output.push_str(print_tree_code(&middle, &end, "", *child).as_str());
        }
    }
    output
}

fn print_tree_code(middle: &str, end: &str, prefix: &str, node: *mut Node) -> String {
    let mut output = String::new();
    if !node.is_null() {
        unsafe {
            let length = (*node).children.len();
            for (i, child) in (*node).children.iter().enumerate() {
                output.push_str(prefix);

                if i == length - 1 {
                    output.push_str(end);
                } else {
                    output.push_str(middle);
                }
                let content = (*(*child)).content.clone();
                output.push_str(String::from_iter(content).as_str());
                output.push('\n');
                if i == length - 1 {
                    output.push_str(
                        print_tree_code(middle, end, (prefix.to_owned() + "    ").as_str(), *child)
                            .as_str(),
                    );
                } else {
                    output.push_str(
                        print_tree_code(middle, end, (prefix.to_owned() + " |  ").as_str(), *child)
                            .as_str(),
                    );
                }
            }
        }
    }
    output
}

fn merge_display_tree(align: Align, children: Vec<DisplayTree>, content: Vec<char>) -> DisplayTree {
    let space_to_add = vec![' '; content.len() + 3];
    let mut res = DisplayTree::new();
    if children.is_empty() {
        res.content.push(content);
        res.entrance = 0;
        return res;
    }

    // draw children
    for child in &children {
        for line in &child.content {
            let mut tmp = space_to_add.clone();
            tmp.append(&mut line.clone());
            res.content.push(tmp);
        }
    }

    // draw the current content
    match align {
        Align::Top => res.entrance = 0,
        Align::Center => res.entrance = res.content.len() / 2,
        Align::Bottom => res.entrance = res.content.len() - 1,
    }

    res.content[res.entrance][..content.len()].clone_from_slice(&content[..]);

    // draw vertex
    let mut first_entrance = children[0_usize].entrance;
    let mut last_entrance = 0_usize;
    {
        let mut y = 0_usize;
        for child in &children {
            last_entrance = y + child.entrance;
            y += child.content.len();
        }
    }

    let mut y = 0_usize;
    for child in &children {
        let start = y;

        // draw child vertical connector
        for _ in &child.content {
            if y >= first_entrance && y <= last_entrance {
                res.content[y][content.len() + 1] = '│';
            }
            y += 1;
        }

        // refine connector on child entrance points
        let child_entrance = start + child.entrance;
        let connector = res.content[child_entrance][content.len() + 1];
        if first_entrance == last_entrance {
            res.content[child_entrance][content.len() + 1] = '─';
        } else if child_entrance == first_entrance {
            res.content[child_entrance][content.len() + 1] = '┌';
        } else if child_entrance < last_entrance {
            res.content[child_entrance][content.len() + 1] = '├';
        } else {
            res.content[child_entrance][content.len() + 1] = '└';
        }

        // draw connector to child entrance
        res.content[child_entrance][content.len() + 2] = '─';
    }
    // draw parent entrance to connector
    res.content[res.entrance][content.len() + 0] = '─';
    // refine connector on parent entrance points
    let connector = res.content[res.entrance][content.len() + 1];
    match connector {
        '─' => res.content[res.entrance][content.len() + 1] = '─',
        '┌' => res.content[res.entrance][content.len() + 1] = '┬',
        '├' => res.content[res.entrance][content.len() + 1] = '┼',
        '└' => res.content[res.entrance][content.len() + 1] = '┴',
        '│' => res.content[res.entrance][content.len() + 1] = '┤',
        _ => {
            println!("not recognize char {}", connector);
        }
    }
    res
}

fn make_display_tree(align: Align, node: *mut Node) -> DisplayTree {
    let mut children_tree = Vec::new();
    unsafe {
        for child in &(*node).children {
            children_tree.push(make_display_tree(align, *child));
        }
    }
    unsafe { merge_display_tree(align, children_tree, (*node).content.clone()) }
}

fn print_tree_by_align_style(style: Align, tree: Tree) -> String {
    if tree.root.is_null() {
        return "".to_string();
    }
    let display = make_display_tree(style, tree.root);
    let mut res = String::new();
    for line in display.content {
        let content = String::from_iter(line);
        res.push_str((content + "\n").as_str());
    }
    res
}

impl Tree {
    pub fn translate(input: &str, options_string: String) -> String {
        let options = serialize_option(options_string);

        let style_option = options.get("style").unwrap();

        // parse input string
        let mut print_lines: Vec<Line> = Vec::new();
        let lines = input.lines();
        for line in lines {
            let mut print_line = Line::new();
            for c in line.chars() {
                if !c.is_whitespace() {
                    print_line.content.push(c);
                } else {
                    print_line.spaces += 1;
                }
            }
            print_lines.push(print_line);
        }

        if print_lines.is_empty() {
            return "".to_string();
        }

        // build tree from Vec<Line>
        let tree = Tree::new();

        for i in 0..print_lines.len() {
            let child = Box::new(Node {
                content: print_lines[i].content.clone(),
                parent: ptr::null_mut(),
                children: vec![],
            });
            let child = Box::leak(child);
            print_lines[i].tree = child;
            unsafe {
                child.content = print_lines[i].content.clone();
                for j in (0..i).rev() {
                    if print_lines[j].spaces < print_lines[i].spaces {
                        child.parent = print_lines[j].tree;
                        (*(child.parent)).children.push(child);
                        break;
                    }
                }
                if child.parent.is_null() {
                    child.parent = tree.root;
                    (*(child.parent)).children.push(child);
                }
            }
        }

        match style_option.as_str() {
            "unicode 1" => print_tree_by_print_style(PrintStyle::Unicode1, tree),
            "unicode 2" => print_tree_by_print_style(PrintStyle::Unicode2, tree),
            "ASCII 1" => print_tree_by_print_style(PrintStyle::ASCII1, tree),
            "ASCII 2" => print_tree_by_print_style(PrintStyle::ASCII2, tree),
            "ASCII 3" => print_tree_by_print_style(PrintStyle::ASCII3, tree),
            "unicode right top" => print_tree_by_align_style(Align::Top, tree),
            "unicode right center" => print_tree_by_align_style(Align::Center, tree),
            "unicode right bottom" => print_tree_by_align_style(Align::Bottom, tree),
            _ => print_tree_by_print_style(PrintStyle::Unicode1, tree),
        }
    }
}

impl Translator for Tree {
    fn identifier() -> String {
        "Tree".to_string()
    }
    fn name() -> String {
        "Tree".to_string()
    }
    fn description() -> String {
        "Draw a tree".to_string()
    }
    fn options() -> HashMap<String, OptionDescription> {
        let mut res = HashMap::new();
        res.insert(
            "style".to_string(),
            OptionDescription {
                name: "style".to_string(),
                values: vec![
                    "unicode 1".to_string(),
                    "unicode 2".to_string(),
                    "ASCII 1".to_string(),
                    "ASCII 2".to_string(),
                    "ASCII 3".to_string(),
                    "unicode right top".to_string(),
                    "unicode right center".to_string(),
                    "unicode right bottom".to_string(),
                ],
                default_value: "unicode 1".to_string(),
                description: "The style of the tree.".to_string(),
                r#type: Default::default(),
            },
        );
        res
    }
    fn examples() -> Vec<Example> {
        vec![Example {
            title: "1 - Simple".to_string(),
            #[rustfmt::skip]
            input:
r"Linux
  Android
  Debian
    Ubuntu
      Lubuntu
      Kubuntu
      Xubuntu
      Xubuntu
    Mint
  Centos
  Fedora".to_string(),
        }]
    }
}

#[cfg(test)]
mod tests {
    use crate::translator::tree::{Node, PrintStyle, Tree};
    use crate::translator::Translator;
    use std::cell::{Ref, RefCell};
    use std::ptr;
    use std::ptr::{addr_of, addr_of_mut};
    use std::rc::Rc;

    #[test]
    fn test_tree_options() {
        let res = Tree::options();
        println!("{:?}", res);
    }
    #[test]
    fn test_tree_examples() {
        let res = Tree::examples();
        println!("{:#?}", res);
    }

    #[test]
    fn test_print_example() {
        let examples = Tree::examples();
        for example in examples {
            println!(
                "unicode 1\n{}",
                Tree::translate(example.input.as_str(), "style\nunicode 1".to_string())
            );
            println!(
                "unicode 2\n{}",
                Tree::translate(example.input.as_str(), "style\nunicode 2".to_string())
            );
            println!(
                "ASCII 1\n{}",
                Tree::translate(example.input.as_str(), "style\nASCII 1".to_string())
            );
            println!(
                "ASCII 2\n{}",
                Tree::translate(example.input.as_str(), "style\nASCII 2".to_string())
            );
            println!(
                "ASCII 3\n{}",
                Tree::translate(example.input.as_str(), "style\nASCII 3".to_string())
            );
            println!(
                "unicode right top\n{}",
                Tree::translate(
                    example.input.as_str(),
                    "style\nunicode right top".to_string()
                )
            );
            println!(
                "unicode right center\n{}",
                Tree::translate(
                    example.input.as_str(),
                    "style\nunicode right center".to_string()
                )
            );
            println!(
                "unicode right bottom\n{}",
                Tree::translate(
                    example.input.as_str(),
                    "style\nunicode right bottom".to_string()
                )
            );
        }
    }
}
