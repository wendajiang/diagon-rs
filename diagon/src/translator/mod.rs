mod antlr;
mod sequence;
mod sequence_graph;
mod table;
mod tree;

use crate::translator::table::Table;
use crate::translator::Widget::Combobox;
use once_cell::sync::{Lazy, OnceCell};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
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
    pub values: Vec<&'static str>,
    default_value: String,
    description: String,
    r#type: Widget,
}

impl Eq for OptionDescription {}

impl PartialEq for OptionDescription {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Default, Debug)]
pub struct Example {
    title: String,
    input: String,
}

pub trait Translator {
    fn identifier() -> String {
        String::default()
    }
    fn name() -> String {
        String::default()
    }
    fn description() -> String {
        String::default()
    }
    fn options() -> HashMap<&'static str, OptionDescription> {
        HashMap::new()
    }
    fn examples() -> Vec<Example> {
        Vec::new()
    }

    fn translate(_input: &str, _options: &str) -> String {
        String::default()
    }
}

pub fn serialize_option(options: &str) -> HashMap<String, String> {
    let mut lines_iter = options.lines();
    let mut res = HashMap::new();
    while let Some(label) = lines_iter.next() {
        if let Some(value) = lines_iter.next() {
            res.insert(label.trim_end().to_string(), value.trim_end().to_string());
        }
    }
    res
}

pub type GlobalHashMap = HashMap<
    String,
    (
        fn(&str, &str) -> String,
        fn() -> HashMap<&'static str, OptionDescription>,
        fn() -> Vec<Example>,
    ),
>;

// FIXME how to refactor this global store for dynamically adding support subcommand
// https://isocpp.org/wiki/faq/ctors#static-init-order
// https://docs.rs/once_cell/1.9.0/once_cell/
pub static GLOBAL_FN: Lazy<GlobalHashMap> = Lazy::new(|| {
    let mut res: GlobalHashMap = HashMap::new();
    res.insert(
        Tree::identifier(),
        (Tree::translate, Tree::options, Tree::examples),
    );
    res.insert(
        Table::identifier(),
        (Table::translate, Table::options, Table::examples),
    );
    res
});
