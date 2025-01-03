use pklsrs::{codegen::codegen, parser::parse, tokenizer::tokenize};

fn main() {
    let program = stdio();
    println!("// program = {:?}", program);
    let tokens = tokenize(&program);
    println!("// tokens = {:?}", tokens);
    let expr = parse(tokens);
    println!("// expr = {:?}", expr);
    codegen(&expr);
}

fn stdio() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer
}
