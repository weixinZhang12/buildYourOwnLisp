use std::collections::VecDeque;
#[derive(Debug)]
pub enum Token {
    Number(char),
    Operator(char),
    LeftBracket(char),
    RightBracket(char),
    Eof,
}
#[derive(Debug)]
pub enum Expr {
    Number(u32),
    Operator(char, Vec<Expr>),
}

pub struct Tokens {
    tokens: VecDeque<Token>,
}

impl Tokens {
    pub fn new(s: String) -> Tokens {
        let mut deq = VecDeque::new();
        for i in s.chars() {
            match i {
                '0'..='9' => {
                    deq.push_back(Token::Number(i));
                }
                '+' | '-' | '*' | '/' => {
                    deq.push_back(Token::Operator(i));
                }
                '(' => {
                    deq.push_back(Token::LeftBracket('('));
                }
                ')' => {
                    deq.push_back(Token::RightBracket(')'));
                }
                ' ' => {}
                _ => {
                    panic!("Unknown character")
                }
            }
        }
        Self { tokens: deq }
    }
    pub fn next(&mut self) -> Token {
        self.tokens.pop_front().unwrap_or(Token::Eof)
    }
    pub fn peek(&self) -> &Token {
        self.tokens.front().unwrap_or(&Token::Eof)
    }
    pub fn check(&self) {
        let mut last_token = None;
        for token in &self.tokens {
            if last_token.is_none() {
                last_token = Some(token);
                continue;
            }
            last_token = Some(token)
        }
    }
    pub fn parser(&mut self) -> Expr {
        let mut op = match self.next() {
            Token::Number(v) => panic!("Invalid expression,first token should is operator."),
            Token::Operator(v) => Expr::Operator(v, vec![]),
            Token::LeftBracket(_) => panic!("Invalid expression,first token should is operator."),
            Token::RightBracket(_) => panic!("Invalid expression,first token should is operator."),
            Token::Eof => panic!("expression can't be empty."),
        };
        let left = match self.peek() {
            Token::Number(v) => Expr::Number(v.to_digit(10).expect("Invalid expression")),
            Token::Operator(v) => {
                panic!("Invalid expression,expecting a number,but get one '{}'.", v)
            }
            Token::LeftBracket(v) => {
                self.next();
                Self::parser(self)
            }
            Token::RightBracket(v) => {
                panic!("Invalid expression,expecting a number,but get one '{}'.", v)
            }
            Token::Eof => panic!("Invalid expression,expecting a number,but get one 'Eof'."),
        };
        self.next();
        let right = match self.peek() {
            Token::Number(v) => Expr::Number(v.to_digit(10).expect("Invalid expression")),
            Token::Operator(v) => {
                panic!("Invalid expression,expecting a number,but get one '{}'.", v)
            }
            Token::LeftBracket(v) => {
                self.next();
                Self::parser(self)
            }
            Token::RightBracket(v) => {
                panic!("Invalid expression,expecting a number,but get one '{}'.", v)
            }
            Token::Eof => panic!("Invalid expression,expecting a number,but get one 'Eof'."),
        };
        self.next();
        if let Expr::Operator(_, v) = &mut op {
            v.push(left);
            v.push(right);
        }
        op
    }

    pub fn to_string(&self) -> String {
        format!("{:#?}", self.tokens)
    }
}

///Operator('+', [Operator('+', [Number(1), Number(2)]), Operator('+', [Number(1), Number(2)])])

pub fn calc(deq: &Expr)->u32 {
    match deq {
        Expr::Number(v) =>v.to_owned() ,
        Expr::Operator(op, exprs) => {
            let left=match &exprs[0] {
                Expr::Number(v) => v.to_owned(),
                Expr::Operator(v, exprs) =>eval_op(calc(&exprs[0]), op.to_owned(), calc(&exprs[1])),
            };
            let right=match &exprs[1] {
                Expr::Number(v) => v.to_owned(),
                Expr::Operator(v, exprs) =>eval_op(calc(&exprs[0]), op.to_owned(), calc(&exprs[1])),
            };
            eval_op(left, op.to_owned(), right)
        }
    }
}
pub fn eval() {}
pub fn eval_op(x: u32, op: char, y: u32) -> u32 {
    match op {
        '+' => x + y,
        '-' => x - y,
        '/' => x / y,
        '*' => x * y,
        _ => panic!("Unknown character"),
    }
}
// #[test]
// fn test() {
//     let mut tokens = Tokens::new("+ 1 2".to_string());
//     let expr=tokens.parser();
//     println!("{:?}", expr);
//     println!("{:?}", calc(&expr));
// }

#[test]
fn long_expression() {
    let mut tokens = Tokens::new("+ (+ 1 2) (+ 1 2)".to_string());
    let expr=tokens.parser();
    println!("{:?}", expr);
    println!("{:?}", calc(&expr));

}
