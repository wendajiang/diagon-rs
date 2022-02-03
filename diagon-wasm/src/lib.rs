use diagon_rs::translator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn translate(translator_name: String, input: String) -> String {
    let maybe_func = translator::GLOBAL_FN.get(args.component.as_str());
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
