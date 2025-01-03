use crate::parser::Expr;

pub fn codegen(expr: &Expr) {
    println!(".global _main");
    println!("_main:");
    gen_expr(expr);
    println!("    ret");
}

fn gen_expr(expr: &Expr) {
    match expr {
        Expr::Number(n) => {
            println!("    mov x0, {}", n);
        }
    }
}
