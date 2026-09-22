#[derive(Debug)]
pub enum Expr {
    Number(i32), // вариант 1: это число
    Binary { // вариант 2: это операция 
        op: Op, // какая операция(+, -, *, /)
        left: Box<Expr>,// левое подвыражение в куче box
        right: Box<Expr>,// правое подвыражение в куче box
    },
}

#[derive(Debug)]
pub enum Op { // виды операции
    Add,
    Sub,
    Mul,
    Div,
}