use std::iter::*;

#[derive(Clone, Debug)]
pub enum Token {
    Unknown,
    Add,
    Substract,
    Multiply,
    Divide,
    Num(isize),
}

impl Token {
    fn lbp(&self) -> usize {
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

    fn led<Iter: Iterator<Item = Token>>(&self, parser: &mut Parser<Iter>, lhs: Expression) -> Result<Expression, String> {
        match *self {
            Token::Add | Token::Substract | Token::Multiply | Token::Divide => {
                let rhs = parser.expression(self.lbp())?;
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

pub struct TokenIter<'a> {
    chars: std::str::Chars<'a>
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| match c {
            '1' => Token::Num(1),
            '2' => Token::Num(2),
            '+' => Token::Add,
            '-' => Token::Substract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            _ => Token::Unknown    
        })
    }
}

fn  tokens(str: & str) -> TokenIter {
    TokenIter { chars: str.chars() }
}

#[derive(Debug)]
pub enum Expression {
    Num(isize),
    Binary(Box<Expression>, Token, Box<Expression>),
}


pub struct Parser<Iter: Iterator<Item = Token>> {
    tokens: Peekable<Iter>,
}

impl<Iter: Iterator<Item = Token>> Parser<Iter> {
    fn new(tokens: Iter) -> Self {
        Parser {
            tokens: tokens.peekable()
        }
    }

    fn expression(&mut self, rbp: usize) -> Result<Expression, String> {
        let mut left = self.parse_nud()?;
        while self.next_binds_tighter_than(rbp) {
            left = (self.parse_led(left))?;
        }
        Ok(left)
    }

    fn next_binds_tighter_than(&mut self, rbp: usize) -> bool {
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


fn main() {
    let tokens = tokens("2+2");
    let mut parser = Parser::new(tokens);

    println!("parsed: {:?}", parser.expression(0));
}
