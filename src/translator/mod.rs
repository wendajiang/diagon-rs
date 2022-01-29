mod tree;

use crate::translator::Widget::Combobox;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use tree::Tree;

#[derive(Debug)]
enum Widget {
    Combobox,
    CheckBox,
}

impl Default for Widget {
    fn default() -> Self {
        Combobox
    }
}

#[derive(Default, Debug)]
pub struct OptionDescription {
    name: String,
    values: Vec<String>,
    default_value: String,
    description: String,
    r#type: Widget,
}

#[derive(Default, Debug)]
pub struct Example {
    title: String,
    input: String,
}

pub trait Translator {
    fn translate(_input: &str, _options: String) -> String {
        String::default()
    }
    fn identifier() -> String {
        String::default()
    }
    fn name() -> String {
        String::default()
    }
    fn description() -> String {
        String::default()
    }
    fn options() -> Vec<OptionDescription> {
        Vec::new()
    }
    fn examples() -> Vec<Example> {
        Vec::new()
    }
}

pub fn serialize_option(options: String) -> HashMap<String, String> {
    let mut lines_iter = options.lines();
    let mut res = HashMap::new();
    while let Some(label) = lines_iter.next() {
        if let Some(value) = lines_iter.next() {
            res.insert(label.to_string(), value.to_string());
        }
    }
    res
}

pub type GlobalHashMap = HashMap<
    String,
    (
        fn(&str, String) -> String,
        fn() -> Vec<OptionDescription>,
        fn() -> Vec<Example>,
    ),
>;

// FIXME how to refactor this global store for dynamically adding support subcommand
pub fn global_fn() -> &'static GlobalHashMap {
    static GLOBAL_FN: OnceCell<GlobalHashMap> = OnceCell::new();

    GLOBAL_FN.get_or_init(|| {
        let mut res: GlobalHashMap = HashMap::new();
        res.insert(
            Tree::identifier(),
            (Tree::translate, Tree::options, Tree::examples),
        );

        res
    })
}
