use std::iter::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Unknown,
    Open,
    Close,
    Add,
    Substract,
    Multiply,
    Divide,
    Num(isize),
}

impl Token {
    fn lbp(&self) -> usize {
        match *self {
            Token::Open => 80,
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

    fn led<Iter: Iterator<Item = Token>>(
        &self,
        parser: &mut Parser<Iter>,
        lhs: Expression,
    ) -> Result<Expression, String> {
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
    chars: Peekable<std::str::Chars<'a>>,
}

impl TokenIter<'_> {
    pub fn new(str: &str) -> TokenIter {
        let result: TokenIter = TokenIter {
            chars: str.chars().peekable(),
        };
        result
    }

    fn peeking_take_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> String {
        let mut s = String::new();
        while let Some(&ch) = self.chars.peek() {
            if predicate(ch) {
                self.chars.next(); // consume
                s.push(ch);
            } else {
                break;
            }
        }
        s
    }
    fn number(&mut self) -> isize {
        let str = self.peeking_take_while(|c| c <= '9' && c >= '0');
        return str.parse::<isize>().unwrap();
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let oc = self.chars.peek();
            if let Some(c) = oc {
                match c {
                    '0'..='9' => return Some(Token::Num(self.number())),
                    '+' => {
                        self.chars.next();
                        return Some(Token::Add);
                    }
                    '-' => {
                        self.chars.next();
                        return Some(Token::Substract);
                    }
                    '*' => {
                        self.chars.next();
                        return Some(Token::Multiply);
                    }
                    ' ' => { self.chars.next(); },
                    _ => return None,
                }
            } else {
                return None;
            }
        }
    }
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
            tokens: tokens.peekable(),
        }
    }

    fn expression(&mut self, rbp: usize) -> Result<Expression, String> {
        let mut left = self.parse_nud()?;
        while self.next_binds_tighter_than(rbp) {
            left = self.parse_led(left)?;
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
    let tokens = TokenIter::new("123 + 7*3");
    let mut parser = Parser::new(tokens);

    println!("parsed: {:?}", parser.expression(0));
}
