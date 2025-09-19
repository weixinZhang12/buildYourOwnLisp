use std::{collections::VecDeque, fmt, iter::Peekable};

use thiserror::Error;

 use crate::calc::{add, div, head, max, mi, min, mmod, mul, sub};
#[derive(Debug, Error)]
pub enum TokensError<'a> {
    #[error("Invalid expression expected {expected:?},found {found:?}")]
    InvalidExpression { expected: &'a str, found: &'a str },
    #[error("Invalid expression")]
    Invalid,
}
pub type Num = u32;
///基础token
#[derive(Debug, Clone)]
pub enum Token {
    ///数字类型 1 12 ...
    Number(String),
    ///运算符 '+' '-' '*' ...
    Operator(String),
    ///左括号 '('
    LeftBracket,
    ///右括号 ')'
    RightBracket,
    /// 左大括号 '{'
    LeftBraces,
    ///右大括号 '}'
    RightBraces,
    ///结束符
    Eof,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(v) => write!(f, "{}", v),
            Token::Operator(v) => write!(f, "{}", v),
            Token::LeftBracket => write!(f, "("),
            Token::RightBracket => write!(f, ")"),
            Token::LeftBraces => write!(f, "{{"),
            Token::RightBraces => write!(f, "}}"),
            Token::Eof => write!(f, ""),
        }
    }
}
///用于构建表达式树
#[derive(Debug, Clone)]
pub enum Expr {
    ///数字类型
    Number(Num),
    ///操作符树
    Operator(String, Vec<Expr>),
    #[allow(clippy::enum_variant_names)]
    QExpr(Vec<Expr>),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(v) => write!(f, "{}", v),

            Expr::Operator(op, exprs) => {
                let mut s = Vec::new();
                s.push(op.to_owned());
                // 遍历里面的Expr
                for i in exprs {
                    s.push(i.to_string());
                }
                write!(f, "{}", s.join(" "))
            }
            Expr::QExpr(tokens) => {
                // 如果是一个qexpr
                let mut s = Vec::new();
                s.push("{".to_string());
                // 遍历里面的Expr
                for i in tokens {
                    s.push(i.to_string());
                }
                s.push("}".to_string());

                write!(f, "{}", s.join(" "))
            }
        }
    }
}

impl Expr {
    ///计算数学表达式，返回数字表达式
    pub fn calc(&self) -> Expr {
        match self {
            Expr::Number(v) => Expr::Number(*v),
            Expr::Operator(op, exprs) => Self::switch(op, exprs),
            Expr::QExpr(tokens) => Expr::QExpr(tokens.to_owned()),
        }
    }
    /// 获取计算结果
    pub fn math_result(&self) -> Num {
        match self {
            Expr::Number(v) => v.to_owned(),
            Expr::Operator(op, v) => {
                if let Expr::Number(v) = Self::switch(op, v) {
                    v
                } else {
                    panic!("Invalid")
                }
            }
            Expr::QExpr(exprs) => panic!("Invalid expression,expect a number,but get a qexpr"),
        }
    }

    // 通过迭代器的方式计算值,传入的如果是一个数那么直接返回，如果是一个表达式，通过递归调用直到遇到数字为止
    pub fn switch(op: &str, v: &[Expr]) -> Expr {
        match op {
            "head" =>  head(v).to_owned(),
            "+" => add(v),
            "-"=>sub(v),
            "*"=>mul(v),
            "/"=>div(v),
            "min"=>min(v),
            "max"=>max(v),
            "%"=>mmod(v),
            "^"=>mi(v),
            _ => {
                panic!()
            }
        }
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: VecDeque<Token>,
}

impl Tokens {
    pub fn new(s: String) -> Tokens {
        let mut deq = VecDeque::new();
        let mut iter = s.chars().peekable();
        while let Some(v) = iter.peek() {
            let c = v;
            match c {
                '0'..='9' => {
                    let s = Self::parser_number_string(&mut iter);
                    deq.push_back(Token::Number(s));
                    continue;
                }
                'a'..='z' => {
                    let s = Self::str_parser(&mut iter);
                    deq.push_back(Token::Operator(s));
                }
                '+' | '-' | '*' | '/' | '%' | '^' => {
                    deq.push_back(Token::Operator(c.to_string()));
                }
                '(' => {
                    deq.push_back(Token::LeftBracket);
                }
                ')' => {
                    deq.push_back(Token::RightBracket);
                }
                ' ' => {}
                '{' => {
                    deq.push_back(Token::LeftBraces);
                }
                '}' => {
                    deq.push_back(Token::RightBraces);
                }
                _ => {
                    panic!("Unknown character")
                }
            }
            iter.next();
        }

        Self { tokens: deq }
    }
    // 将连接在一起的字符组成字符串
    pub fn str_parser<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> String {
        let mut s = String::new();
        if let Some(v) = iter.next() {
            s.push(v);
        }
        while let Some(v) = iter.peek() {
            match v {
                'a'..='z' => {
                    s.push(v.to_owned());
                    iter.next();
                }
                _ => {
                    break;
                }
            }
        }
        s
    }
    /// 获取下一个tokens
    pub fn next(&mut self) -> Token {
        self.tokens.pop_front().unwrap_or(Token::Eof)
    }
    ///查看下一个token
    pub fn peek(&self) -> &Token {
        self.tokens.front().unwrap_or(&Token::Eof)
    }

    ///读取数字，可用于读取连续数字
    pub fn parser_number_string<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> String {
        let mut s = String::new();
        if let Some(v) = iter.next() {
            s.push(v);
        }
        while let Some(v) = iter.peek() {
            match v {
                '0'..='9' => {
                    s.push(v.to_owned());
                    iter.next();
                }
                _ => {
                    break;
                }
            }
        }
        s
    }
    /// 转换qexpr表达式
    pub fn parser_qexpr(&mut self) -> Expr {
        let mut vec = Vec::new();
        loop {
            match self.next() {
                Token::Number(v) => vec.push(Expr::Number(v.parse().expect("Invalid"))),
                Token::LeftBraces => {
                    let v = self.parser_qexpr();
                    vec.push(v);
                }
                Token::RightBraces => {
                    break;
                }
                Token::Eof => break,
                Token::LeftBracket => vec.push(self.parser()),
                Token::RightBracket => continue,
                Token::Operator(_) => panic!("Invalid"),
            };
        }
        Expr::QExpr(vec)
    }
    // 转换为数学表达式树
    pub fn parser(&mut self) -> Expr {
        // 第一次读取表达式一定是运算符或者括号
        let mut op = match self.next() {
            Token::Number(v) => Expr::Number(v.parse().expect("Invalid expression")),
            Token::Operator(v) => Expr::Operator(v, vec![]),
            Token::LeftBracket => self.parser(),
            Token::RightBracket => {
                panic!("Invalid expression,first token should is operator or left-bracket.")
            }
            Token::Eof => panic!("expression can't be empty."),
            Token::LeftBraces => self.parser_qexpr(),
            Token::RightBraces => panic!("Invalid"),
        };
        // 如果op是一个操作符，目前并不清楚其他情况
        if let Expr::Operator(_, vec) = &mut op {
            loop {
                if let Token::Eof = self.peek() {
                    break;
                } else {
                    let value = match self.peek() {
                        Token::Number(v) => Expr::Number(
                            // 将字符串转换为数字
                            v.parse().expect("Invalid expression,the number is invalid"),
                        ),
                        Token::Operator(v) => {
                            panic!("{}",TokensError::InvalidExpression { expected: "number", found: v.as_str() })
                        }
                        Token::LeftBracket => {
                            // 跳过这个左括号，可以少递归一层
                            self.next();
                            self.parser()
                        }
                        Token::RightBracket => {
                            return op;
                        }
                        Token::Eof => {
                            panic!("{}",TokensError::InvalidExpression { expected: "number", found: "Eof" })
                        }
                        Token::LeftBraces => {
                            self.next();
                            self.parser_qexpr()
                        }
                        Token::RightBraces => break,
                    };
                    
                    vec.push(value);
                    self.next();
                }
            }
        }
        op
    }
}
#[test]
fn test() {
    let mut tokens = Tokens::new("% 2 2".to_string());
    println!("{:?}", tokens);
    let expr = tokens.parser();
    println!("{:?}", expr);
    let v = expr.math_result();
    assert_eq!(0, v);
    println!("{:?}", v);
}
#[test]
fn simple() {
    let mut tokens = Tokens::new("+ 1 2".to_string());
    println!("{:?}", tokens);
    let expr = tokens.parser();
    println!("{:?}", expr);
    let v = expr.math_result();
    assert_eq!(3, v);
    println!("{:?}", v);
}
#[test]
fn bracket() {
    let mut tokens = Tokens::new("(+ 1 2)".to_string());
    println!("{:?}", tokens);
    let expr = tokens.parser();
    println!("{:?}", expr);
    let v = expr.math_result();
    assert_eq!(3, v);
    println!("{:?}", v);
}
#[test]
fn long_number() {
    let mut tokens = Tokens::new("+ 12 2 2".to_string());
    println!("{:?}", tokens);
    let expr = tokens.parser();
    println!("{:?}", expr);
    let v = expr.math_result();
    assert_eq!(16, v);
    println!("{:?}", v);
}
#[test]
fn operator_test() {
    let mut tokens = Tokens::new("^ (% 8 3) 2".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.math_result();
    assert_eq!(4, v);
    println!("{:?}", v);
}

#[test]
fn long_expression() {
    let mut tokens = Tokens::new("+ (% 18 2) (+ 5 2)".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.math_result();
    assert_eq!(7, v);
    println!("{:?}", v);
}
#[test]
fn long_expression2() {
    let mut tokens = Tokens::new("* 22 4 (+ 0 0 0)".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.math_result();
    assert_eq!(0, v);
    println!("{:?}", v);
}
#[test]
fn str_expr() {
    let mut tokens = Tokens::new("max 1 2 3".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.math_result();
    assert_eq!(3, v);
    println!("{:?}", v);
}
#[test]
fn qexpr() {
    let mut tokens = Tokens::new("{1 2 3 {1 2 3}}".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.calc();
    println!("{}", v);
}
#[test]
fn qexpr2() {
    let mut tokens = Tokens::new("head { (1)2 { 3 4 }}".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.calc();
    println!("{}", v);

}
