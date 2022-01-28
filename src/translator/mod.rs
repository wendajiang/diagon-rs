mod tree;

use crate::translator::Widget::Combobox;
use std::collections::HashMap;

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
struct OptionDescription {
    name: String,
    values: Vec<String>,
    default_value: String,
    description: String,
    r#type: Widget,
}

#[derive(Default, Debug)]
struct Example {
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
    fn options() -> HashMap<String, OptionDescription> {
        HashMap::new()
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

pub type TranslatorPtr = Box<dyn Translator + Send>;
