use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    Sum(Box<Expr>, Box<Expr>),
    Prod(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Empty,
}

impl Expr {
    pub fn fill(&mut self, new_expr: Expr) {
        match self {
            Expr::Empty => *self = new_expr,
            Expr::Num(_) => panic!("Cannot fill a number"),
            Expr::Neg(expr) => expr.fill(new_expr),
            Expr::Sum(_, expr) => expr.fill(new_expr),
            Expr::Prod(_, expr) => expr.fill(new_expr),
            Expr::Div(_, expr) => expr.fill(new_expr),
        }
    }

    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Neg(expr) => -expr.eval(),
            Expr::Sum(left, right) => left.eval() + right.eval(),
            Expr::Prod(left, right) => left.eval() * right.eval(),
            Expr::Div(left, right) => left.eval() / right.eval(),
            Expr::Empty => panic!("Cannot evaluate an empty expression"),
        }
    }
}

pub fn parse_expr(tokens: &[Token], pos: &mut usize) -> Expr {
    let mut expr = Expr::Empty;

    while *pos < tokens.len() {
        *pos += 1;

        match tokens[*pos - 1] {
            Token::Number(num) => expr.fill(Expr::Num(num)),
            Token::Operator('+') => match expr {
                Expr::Empty => {}
                _ => expr = Expr::Sum(Box::new(expr), Box::new(Expr::Empty)),
            },
            Token::Operator('-') => match expr {
                Expr::Empty => expr = Expr::Neg(Box::new(Expr::Empty)),
                _ => expr = Expr::Sum(Box::new(expr), Box::new(Expr::Neg(Box::new(Expr::Empty)))),
            },
            Token::Operator('*') => match expr {
                Expr::Empty => panic!("Unexpected operator '*'"),
                Expr::Sum(left, right) => {
                    expr = Expr::Sum(left, Box::new(Expr::Prod(right, Box::new(Expr::Empty))))
                }
                _ => expr = Expr::Prod(Box::new(expr), Box::new(Expr::Empty)),
            },
            Token::Operator('/') => match expr {
                Expr::Empty => panic!("Unexpected operator '/'"),
                Expr::Sum(left, right) => {
                    expr = Expr::Sum(left, Box::new(Expr::Div(right, Box::new(Expr::Empty))))
                }
                _ => expr = Expr::Div(Box::new(expr), Box::new(Expr::Empty)),
            },
            Token::Operator(op) => panic!("Unexpected operator '{}'", op),
            Token::Parenthesis(c) => match c {
                '(' => expr.fill(parse_expr(tokens, pos)),
                ')' => return expr,
                _ => panic!("Unexpected parenthesis '{}'", c),
            },
        }
    }

    match expr {
        Expr::Empty => panic!("Empty expression"),
        _ => expr,
    }
}

pub fn parse(input: &str) -> Expr {
    let tokens = crate::lexer::lex(input);

    let mut pos = 0;
    let result_expr = parse_expr(&tokens, &mut pos);

    if pos < tokens.len() {
        panic!("Unexpected token '{:?}'", tokens[pos]);
    }

    result_expr
}
