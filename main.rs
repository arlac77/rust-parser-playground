use std::iter::*;
use std::slice::*;

#[derive(Debug)]
pub enum Expression {
    Num(i64),
    Binary(Box<Expression>, Token, Box<Expression>),
}

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Iter<'a, Token>) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    fn expression(&mut self, rbp: u32) -> Result<Expression, String> {
        let mut left = (self.parse_nud())?;
        while self.next_binds_tighter_than(rbp) {
            left = (self.parse_led(left))?;
        }
        Ok(left)
    }

    fn next_binds_tighter_than(&mut self, rbp: u32) -> bool {
        self.tokens.peek().map_or(false, |t| t.lbp() > rbp)
    }

    fn parse_nud(&mut self) -> Result<Expression, String> {
        self.tokens
            .next()
            .map_or(Err("incomplete".to_string()), |t| t.nud())
    }

    fn parse_led(&mut self, expr: Expression) -> Result<Expression, String> {
        self.tokens
            .next()
            .map_or(Err("incomplete".to_string()), |t| t.led(self, expr))
    }
}

#[derive(Clone, Debug)]
pub enum Token {
    Add,
    Substract,
    Multiply,
    Divide,
    Num(i64),
}

impl Token {
    fn lbp(&self) -> u32 {
        match *self {
            Token::Add => 10,
            Token::Substract => 10,
            Token::Multiply => 20,
            Token::Divide => 20,
            _ => 0,
        }
    }

    fn nud(&self) -> Result<Expression, String> {
        match *self {
            Token::Num(i) => Ok(Expression::Num(i)),
            _ => Err("expecting literal".to_string()),
        }
    }

    fn led(&self, parser: &mut Parser, lhs: Expression) -> Result<Expression, String> {
        match *self {
            Token::Add | Token::Substract | Token::Multiply | Token::Divide => {
                let rhs = (parser.expression(self.lbp()))?;
                Ok(Expression::Binary(
                    Box::new(lhs),
                    self.clone(),
                    Box::new(rhs),
                ))
            }

            _ => Err("expecing operator".to_string()),
        }
    }
}

static TOKENS: [Token; 5] = [
    Token::Num(1),
    Token::Add,
    Token::Num(2),
    Token::Multiply,
    Token::Num(5),
];

fn tokens() -> std::slice::Iter<'static, Token> {
    TOKENS.iter()
}

fn main() {
    let mut parser = Parser::new(tokens());
    println!("parsed: {:?}", parser.expression(0));
}
