use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let antlr_files = vec![("sequence")];

    let additional_args = vec![Some("-visitor"), None, None, None, None];
    let antlr_path =
        "/Users/david/learncppcode/diagon-rs/diagon/assets/antlr4-4.8-2-SNAPSHOT-complete.jar";

    for (grammar, arg) in antlr_files.into_iter().zip(additional_args) {
        // ignoring error because we do not need to run anything when deploying to crates.io
        let _ = gen_for_grammar(grammar, antlr_path, arg);
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/antlr4-4.8-2-SNAPSHOT-complete.jar");
}

fn gen_for_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let input = PathBuf::from("src/translator/antlr".to_string());
    let file_name = grammar_file_name.to_owned() + ".g4";
    println!("cargo:rerun-if-changed=src/translator/antlr/{}", file_name);

    let c = Command::new("java")
        .current_dir(input)
        .arg("-jar")
        .arg(antlr_path)
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg("codegen")
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;

    println!("{:?}", c);

    Ok(())
}
