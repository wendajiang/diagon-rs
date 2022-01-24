use crate::translator::Widget::Combobox;
use std::collections::BTreeMap;

enum Widget {
    Combobox,
    CheckBox,
}

impl Default for Widget {
    fn default() -> Self {
        Combobox
    }
}

#[derive(Default)]
struct OptionDescription {
    name: String,
    values: Vec<String>,
    default_value: String,
    description: String,
    r#type: Widget,
}

#[derive(Default)]
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
    fn options() -> Vec<OptionDescription> {
        Vec::new()
    }
    fn examples() -> Vec<Example> {
        Vec::new()
    }
}

pub fn serialize_option(options: String) -> BTreeMap<String, String> {}

pub type TranslatorPtr = Box<dyn Translator + Send>;
