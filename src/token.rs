use std::{collections::VecDeque, iter::Peekable};

use thiserror::Error;
#[derive(Debug, Error)]
enum TokensError<'a> {
    #[error("Invalid expression expected {expected:?},found {found:?}")]
    InvalidExpression { expected: &'a str, found: &'a str },
}
type Num = u32;
#[derive(Debug)]
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
///用于构建表达式树
#[derive(Debug)]
pub enum Expr {
    ///数字类型
    Number(Num),
    ///操作符树
    Operator(String, Vec<Expr>),
    QExpr(Vec<Token>),
}

impl Expr {
    ///计算数学表达式，返回数字
    pub fn calc(&self) -> Num {
        match self {
            Expr::Number(v) => v.to_owned(),
            Expr::Operator(op, exprs) => {
                let mut _iter = exprs.iter();
                // 先获取一个基础数字，如果运算符后面不跟数字，代表表达式错误
                let base = _iter.next().expect("Invalid expression");
                let mut _res = 0;
                // 尝试获取一个数字
                _res = match base {
                    Expr::Number(v) => v.to_owned(),
                    Expr::Operator(op, exprs) => {
                        let mut iter = exprs.iter();
                        // 获取第一个值,第一个值不可能没有
                        _res = iter.next().expect("Invalid expression").math_result();
                        for expr in iter {
                            let value = expr.calc();
                            _res = eval_op(_res, op.to_owned(), value);
                        }
                        _res
                    }
                    Expr::QExpr(exprs) => panic!("Invalid"),
                };
                for i in _iter {
                    _res = eval_op(_res, op.to_owned(), i.math_result())
                }
                _res
                // 将第一个数字与其他数字计算
            }
            Expr::QExpr(exprs) => panic!("Invalid"),
        }
    }
    /// 获取计算结果
    pub fn math_result(&self) -> Num {
        match self {
            Expr::Number(v) => v.to_owned(),
            Expr::Operator(op, v) => Self::calc_by_iter(op.to_owned(), v.iter()),
            Expr::QExpr(exprs) => panic!("Invalid expression,expect a number,but get a qexpr"),
        }
    }
    pub fn qexpr_result(&self) -> String {
        match self {
            Expr::Number(v) => panic!("Invalid"),
            Expr::Operator(_, exprs) => panic!("Invalid"),
            Expr::QExpr(tokens) => {
                let mut s = String::new();
                for i in tokens {
                    let c = match i {
                        Token::Number(v) => v.as_str(),
                        Token::Operator(v) => v.as_str(),
                        Token::LeftBracket => "(",
                        Token::RightBracket => ")",
                        Token::LeftBraces => "{",
                        Token::RightBraces => "}",
                        Token::Eof => "",
                    };
                    s.push_str(c);
                    s.push(' ');
                }
                s=s[0..s.len()-1].to_string();
                s
            }
        }
    }
    // 通过迭代器的方式计算值,传入的如果是一个数那么直接返回，如果是一个表达式，通过递归调用直到遇到数字为止
    pub fn calc_by_iter<'a, I: Iterator<Item = &'a Expr>>(op: String, mut v: I) -> Num {
        let mut first = match v.next().expect("Invalid expression") {
            Expr::Number(v) => v.to_owned(),
            Expr::Operator(op, math_exprs) => Self::calc_by_iter(op.to_owned(), math_exprs.iter()),
            Expr::QExpr(exprs) => panic!("Invalid"),
        };
        for expr in v {
            let value = match expr {
                Expr::Number(v) => *v,
                Expr::Operator(op, math_exprs) => {
                    Self::calc_by_iter(op.to_owned(), math_exprs.iter())
                }
                Expr::QExpr(exprs) => panic!("Invalid"),
            };
            first = eval_op(first, op.to_owned(), value)
        }
        first
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
            // iter.next();
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
            // iter.next();
        }
        s
    }
    pub fn parser_qexpr(&mut self) -> Expr {
        let mut vec = Vec::new();
        vec.push(Token::LeftBraces);
        loop {
            match self.next() {
                Token::Number(v) => vec.push(Token::Number(v)),
                Token::Operator(v) => vec.push(Token::Operator(v)),
                Token::LeftBracket => vec.push(Token::LeftBracket),
                Token::RightBracket => vec.push(Token::RightBracket),
                Token::LeftBraces => {
                    let v=self.parser_qexpr();
                    if let Expr::QExpr(v)=v{
                        for i in v{
                            vec.push(i);
                        }
                    }
                }
                Token::RightBraces => {
                    vec.push(Token::RightBraces);
                    break;
                }
                // Token::Eof => panic!("Invalid expression,your expreesion seem has some problems"),
                Token::Eof => break,
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
        if let Expr::Operator(_, vec) = &mut op {
            loop {
                if let Token::Eof = self.peek() {
                    break;
                } else {
                    let value = match self.peek() {
                        Token::Number(v) => Expr::Number(
                            v.parse().expect("Invalid expression,the number is invalid"),
                        ),
                        Token::Operator(v) => {
                            panic!("Invalid expression,expecting a number,but get one '{}'.", v)
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
                            panic!("Invalid expression,expecting a number,but get one 'Eof'.")
                        }
                        Token::LeftBraces => self.parser_qexpr(),
                        Token::RightBraces => break,
                    };
                    vec.push(value);
                    self.next();
                }
            }
        }
        op
    }
    #[allow(clippy::inherent_to_string)]
    ///转换为字符串
    pub fn to_string(&self) -> String {
        format!("{:#?}", self.tokens)
    }
}
// 计算单个表达式树的值
pub fn calc<I: Iterator<Item = Expr>>(op: String, v: &mut I) -> Num {
    let mut first = match v.next().expect("Invalid expression") {
        Expr::Number(v) => v.to_owned(),
        Expr::Operator(op, math_exprs) => calc(op.to_owned(), &mut math_exprs.into_iter()),
        Expr::QExpr(exprs) => panic!("Invalid"),
    };
    for expr in v {
        let value = match expr {
            Expr::Number(v) => v,
            Expr::Operator(op, math_exprs) => calc(op, &mut math_exprs.into_iter()),
            Expr::QExpr(exprs) => panic!("Invalid"),
        };
        first = eval_op(first, op.to_owned(), value)
    }
    first
}
pub fn eval_op(x: Num, op: String, y: Num) -> Num {
    match op.as_str() {
        "+" => x + y,
        "-" => x - y,
        "/" => x / y,
        "*" => x * y,
        "%" => x % y,
        "^" => x.pow(y),
        "min" => x.min(y),
        "max" => x.max(y),
        _ => panic!("Unknown character"),
    }
}
#[test]
fn test() {
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
    let v = expr.qexpr_result();
    println!("{:?}", v);
}
#[test]
fn qexpr2() {
    let mut tokens = Tokens::new("+ 1 { 1 2}".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.calc();
    println!("{:?}", v);
}
