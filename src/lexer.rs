#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    Parenthesis(char),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut num = 0 as f64;

                while let Some('0'..='9') = chars.peek() {
                    num = num * 10.0 + chars.next().unwrap().to_digit(10).unwrap() as f64;
                }

                if chars.peek().map_or(false, |&c| c == '.') {
                    chars.next();
                    let mut pos = 0;

                    while let Some('0'..='9') = chars.peek() {
                        let digit = chars.next().unwrap().to_digit(10).unwrap() as f64;
                        num = num + digit / ((10 as f64).powi(pos + 1));

                        pos += 1;
                    }
                }

                tokens.push(Token::Number(num));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Operator(chars.next().unwrap()));
            }
            '(' | ')' => {
                tokens.push(Token::Parenthesis(chars.next().unwrap()));
            }
            ' ' => {
                chars.next();
            }
            _ => panic!("Unexpected character '{}'", c),
        }
    }

    tokens
}
