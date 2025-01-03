use pklsrs::{codegen::codegen, parser::parse, tokenizer::tokenize};

fn main() {
    let program = stdio();
    let tokens = tokenize(&program);
    let expr = parse(tokens);
    codegen(&expr);
}

fn stdio() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer
}
