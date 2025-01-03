use crate::parser::Expr;

pub fn codegen(expr: &Expr) {
    println!(".global _main");
    println!("_main:");
    gen_expr(expr);
    println!("    ldr x0, [sp], #16 // pop x0");
    println!("    ret");
}

fn gen_expr(expr: &Expr) {
    println!("// {:?}", expr);

    match expr {
        Expr::Number(n) => {
            println!("    mov x0, {}", n);
            println!("    str x0, [sp, #-16]! // push x0");
        }
        Expr::Add { lhs, rhs } => {
            gen_expr(lhs);
            gen_expr(rhs);
            println!("    ldr x2, [sp], #16 // pop x2");
            println!("    ldr x1, [sp], #16 // pop x1");
            println!("    add x0, x1, x2");
            println!("    str x0, [sp, #-16]! // push x0");
        }
        Expr::Sub { lhs, rhs } => {
            gen_expr(lhs);
            gen_expr(rhs);
            println!("    ldr x2, [sp], #16 // pop x2");
            println!("    ldr x1, [sp], #16 // pop x1");
            println!("    sub x0, x1, x2");
            println!("    str x0, [sp, #-16]! // push x0");
        }
        Expr::Mul { lhs, rhs } => {
            gen_expr(lhs);
            gen_expr(rhs);
            println!("    ldr x2, [sp], #16 // pop x2");
            println!("    ldr x1, [sp], #16 // pop x1");
            println!("    mul x0, x1, x2");
            println!("    str x0, [sp, #-16]! // push x0");
        }
        Expr::Div { lhs, rhs } => {
            gen_expr(lhs);
            gen_expr(rhs);
            println!("    ldr x2, [sp], #16 // pop x2");
            println!("    ldr x1, [sp], #16 // pop x1");
            println!("    sdiv x0, x1, x2");
            println!("    str x0, [sp, #-16]! // push x0");
        }
    }
}
