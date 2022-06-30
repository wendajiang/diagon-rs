use diagon::translator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(JsName = "value")]
pub fn translate(translator_name: String, input: String) -> String {
    let maybe_func = translator::GLOBAL_FN.get(translator_name.as_str());

    let (opt_func, options, examples) = maybe_func.unwrap();

    unimplemented!()
}
