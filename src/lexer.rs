#[derive(Debug, Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c.is_ascii_digit() {
            let mut num_str = String::new();
            while i < chars.len() && chars[i].is_ascii_digit() {
                num_str.push(chars[i]);
                i += 1;
            }
            let value: i32 = num_str.parse().unwrap();
            tokens.push(Token::Number(value));
        } else if c == '+' {
            tokens.push(Token::Plus);
            i += 1;
        } else if c == '-' {
            tokens.push(Token::Minus);
            i += 1;
        } else if c == '*' {
            tokens.push(Token::Star);
            i += 1;
        } else if c == '/' {
            tokens.push(Token::Slash);
            i += 1;
        } else if c == '(' {
            tokens.push(Token::LParen);
            i += 1;
        } else if c == ')' {
            tokens.push(Token::RParen);
            i += 1;
        } else {
            return Err(format!("Неизвестный символ: '{}'", c));
        }
    }
    Ok(tokens)
}