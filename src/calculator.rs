
#[derive(Debug,PartialEq,Eq, PartialOrd,Ord)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
#[derive(Debug,PartialEq,Eq, PartialOrd,Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

pub struct Calculator{}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();// borrowed expression
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for this_char in chars {
            if this_char == '\r' {
            } else {
            match this_char {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (this_char as u32 - 48);
                    },
                    _ => {
                        let this_digit = this_char as u32 - 48;
                        tokens.push(Token::Number(this_digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(this_char);
                },
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(Error::MismatchedParens);
                        }
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Addition)),
                '-' => tokens.push(Token::Op(Operator::Subtraction)),
                '/' => tokens.push(Token::Op(Operator::Division)),
                '*' => tokens.push(Token::Op(Operator::Multiplication)),
                ' ' => {},
                '\n' => {},
                _ => return Err(Error::BadToken(this_char))
                }
            }
        }
            if parens.len() > 0 {
                return Err(Error::MismatchedParens);
            }
            Ok(tokens)
    }    
    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                },
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('('){
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                },
                _ => {}
            }
        }
        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }
        queue

    }
    pub fn evaluate(mut tokens : Vec<Token>) -> Option<f32> {
        tokens.reverse();
        let mut stack: Vec<f32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Op(Operator::Addition) => {
                    let right = stack.pop().unwrap();
                    let left  = stack.pop().unwrap();
                    stack.push(left + right);
                },
                Token::Op(Operator::Subtraction) => {
                    let right = stack.pop().unwrap();
                    let left  = stack.pop().unwrap();
                    stack.push(left - right);
                },
                Token::Op(Operator::Multiplication) => {
                    let right = stack.pop().unwrap();
                    let left  = stack.pop().unwrap();
                    stack.push(left * right);
                },
                Token::Op(Operator::Division) => {
                    let right = stack.pop().unwrap();
                    let left  = stack.pop().unwrap();
                    stack.push(left / right);
                },
                _ => {}
            }
        }
        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
}