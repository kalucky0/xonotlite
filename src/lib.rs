mod args;
mod errors;
mod generator;
mod nodes;
mod parser;
mod tokenizer;
mod tokens;

pub use args::Args;
pub use errors::*;
pub use generator::*;
pub use nodes::*;
pub use parser::*;
pub use tokenizer::*;
pub use tokens::*;

use tokio::{fs, io, process::Command};

pub fn read_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args
}

pub async fn read_source(input_file: &str) -> Result<String, io::Error> {
    let source = fs::read_to_string(input_file).await?;
    Ok(source)
}

pub async fn save_asm(input_file: &str, asm: &str) -> Result<(), io::Error> {
    let file_name = input_file.replace(".xon", ".asm");
    fs::write(file_name, asm).await?;
    Ok(())
}

pub async fn build(input_file: &str) -> Result<(), io::Error> {
    Command::new("nasm")
        .arg("-fmacho64")
        .arg(input_file.replace(".xon", ".asm"))
        .output()
        .await?;

    Ok(())
}

pub async fn link(args: &Args) -> Result<(), io::Error> {
    Command::new("cc")
        .arg(args.input_file.replace(".xon", ".o"))
        .arg("-o")
        .arg(args.output_file.clone())
        .arg("-arch")
        .arg(args.arch.clone())
        .output()
        .await?;

    Ok(())
}
