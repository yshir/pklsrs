fn main() {
    let program = stdio();

    println!(".global _main");
    println!("_main:");
    println!("    mov x0, {}", program);
    println!("    ret");
}

fn stdio() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer
}
