use crate::ast::{Expr, Op};

pub fn eval(expr: &Expr) -> Result<i32, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Binary {op, left, right} => {
            let l = eval(left)?;
            let r = eval(right)?;
            let result = match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => {
                    if r == 0 {
                        return Err("Деление на ноль".to_string());
                    }
                    l / r
                }
            };
            Ok(result)
        }
    }
}