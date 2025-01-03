use crate::parser::Expr;

pub fn codegen(expr: &Expr) {
    println!(".global _main");
    println!("_main:");
    gen_expr(expr);
    println!("    ret");
}

fn gen_expr(expr: &Expr) {
    println!("// {:?}", expr);

    match expr {
        Expr::Number(n) => {
            println!("    mov x0, {}", n);
        }
        Expr::Add { lhs, rhs } => {
            gen_expr(lhs);
            println!("    mov x1, x0");
            gen_expr(rhs);
            println!("    add x0, x1, x0");
        }
        Expr::Sub { lhs, rhs } => {
            gen_expr(lhs);
            println!("    mov x1, x0");
            gen_expr(rhs);
            println!("    sub x0, x1, x0");
        }
        Expr::Mul { lhs, rhs } => {
            gen_expr(lhs);
            println!("    mov x1, x0");
            gen_expr(rhs);
            println!("    mul x0, x1, x0");
        }
        Expr::Div { lhs, rhs } => {
            gen_expr(lhs);
            println!("    mov x1, x0");
            gen_expr(rhs);
            println!("    sdiv x0, x1, x0");
        }
    }
}
