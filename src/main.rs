use std::process;
use xonotlite::{build, exit, link, read_source, save_asm, Args, Generator, Parser, Tokenizer};

#[tokio::main]
async fn main() {
    let args = match Args::read() {
        Some(args) => args,
        None => process::exit(1),
    };

    let source = match read_source(&args.input_file).await {
        Ok(source) => source,
        Err(e) => exit!(e),
    };

    let mut tokenizer = Tokenizer::new(&source);
    let tokens = match tokenizer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => exit!(e),
    };

    let mut parser = Parser::new(tokens);
    let tree = match parser.parse() {
        Ok(tree) => tree,
        Err(e) => exit!(e),
    };

    let mut generator = Generator::new(tree);
    let asm = match generator.generate() {
        Ok(asm) => asm,
        Err(e) => exit!(e),
    };

    match save_asm(&args.input_file, &asm).await {
        Ok(_) => {}
        Err(e) => exit!(e),
    };

    match build(&args.input_file).await {
        Ok(_) => {}
        Err(e) => exit!(e),
    };

    match link(&args).await {
        Ok(_) => {}
        Err(e) => exit!(e),
    };
}
