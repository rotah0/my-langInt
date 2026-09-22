use crate::lexer::Token;
use crate::ast::{Expr, Op};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    // 1. Конструктор
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    // 2. Текущий токен
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    } 
    // 3. Забрать текущий токен и сдвинуть позицию
    fn advance(&mut self) -> Option<Token> {
      let tok = self.tokens.get(self.pos).cloned();
       if tok.is_some() {
        self.pos += 1;
       }
       tok
    }
    // 4. Точка входа
    pub fn parse(&mut self) -> Result<Expr, String> {
        let expr = self.parse_expr()?;
        if self.pos < self.tokens.len() {
            return Err(format!("Неожиданный токен после выражения: {:?}", self.tokens[self.pos]));
        }
         Ok(expr)
    }
    // 5.Разбор выражения - пока что только число
fn parse_expr(&mut self) -> Result<Expr, String> {
    let mut left = self.parse_term()?;        

    loop {
        let op = match self.peek() {
            Some(Token::Plus)  => Op::Add,     
            Some(Token::Minus) => Op::Sub,
            _ => break,                        
        };
        self.advance();
        let right = self.parse_term()?;        
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}
   fn parse_factor(&mut self) -> Result<Expr, String> {
    match self.advance() {
        Some(Token::Number(n)) => Ok(Expr::Number(n)),
        Some(other) => Err(format!("Ожидалось число, найдено: {:?}", other)),
        None => Err("Неожиданный конец ввода".to_string()),
    }
}
  fn parse_term(&mut self) -> Result<Expr, String> {
    let mut left = self.parse_factor()?;

    loop {
        let op = match self.peek() {
            Some(Token::Star)  => Op::Mul,
            Some(Token::Slash) => Op::Div,
            _ => break,
        };
        self.advance();
        let right = self.parse_factor()?;
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
    }

    Ok(left)
}
}