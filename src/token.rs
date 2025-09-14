use std::{collections::VecDeque, iter::Peekable};

use thiserror::Error;
#[derive(Debug, Error)]
enum TokensError<'a> {
    #[error("Invalid expression expected {expected:?},found {found:?}")]
    InvalidExpression { expected: &'a str, found: &'a str},
}
type Num = u32;
#[derive(Debug)]
pub enum Token {
    ///数字类型
    Number(String),
    ///运算符
    Operator(char),
    ///左括号
    LeftBracket(char),
    ///右括号
    RightBracket(char),
    ///结束符
    Eof,
}
///用于构建表达式树
#[derive(Debug)]
pub enum MathExpr {
    ///数字类型
    Number(Num),
    ///操作符树
    Operator(char, Vec<MathExpr>),
}

pub enum QExpr {
    Number(Num),
    Operator(String, Vec<QExpr>),
}

impl MathExpr {
    ///计算数学表达式，返回数字
    pub fn calc(&self) -> Num {
        match self {
            // 如果表达式就是一个数字直接返回就可以了
            MathExpr::Number(v) => v.to_owned(),
            MathExpr::Operator(op, exprs) => {
                let mut _iter = exprs.iter();
                // 先获取一个基础数字，如果运算符后面不跟数字，代表表达式错误
                let base = _iter.next().expect("Invalid expression");
                let mut _res = 0;
                // 尝试获取一个数字
                _res = match base {
                    MathExpr::Number(v) => v.to_owned(),
                    MathExpr::Operator(op, exprs) => {
                        let mut iter = exprs.iter();
                        // 获取第一个值,第一个值不可能没有
                        _res = iter.next().expect("Invalid expression").result();
                        for expr in iter {
                            let value = expr.calc();
                            _res = eval_op(_res, op.to_owned(), value);
                        }
                        _res
                    }
                };
                for i in _iter {
                    _res = eval_op(_res, op.to_owned(), i.result())
                }
                _res
                // 将第一个数字与其他数字计算
            }
        }
    }
    /// 获取计算结果
    pub fn result(&self) -> Num {
        match self {
            MathExpr::Number(v) => v.to_owned(),
            MathExpr::Operator(op, v) => Self::calc_by_iter(op.to_owned(), v.iter()),
        }
    }
    // 通过迭代器的方式计算值,传入的如果是一个数那么直接返回，如果是一个表达式，通过递归调用直到遇到数字为止
    pub fn calc_by_iter<'a, I: Iterator<Item = &'a MathExpr>>(op: char, mut v: I) -> Num {
        let mut first = match v.next().expect("Invalid expression") {
            MathExpr::Number(v) => v.to_owned(),
            MathExpr::Operator(op, math_exprs) => {
                Self::calc_by_iter(op.to_owned(), math_exprs.iter())
            }
        };
        for i in v {
            let value = match i {
                MathExpr::Number(v) => *v,
                MathExpr::Operator(op, math_exprs) => Self::calc_by_iter(*op, math_exprs.iter()),
            };
            first = eval_op(first, op, value)
        }
        first
    }
}

pub enum Expr {
    MathExpr,
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
                '+' | '-' | '*' | '/' | '%' | '^' => {
                    deq.push_back(Token::Operator(c.to_owned()));
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
            iter.next();
        }

        Self { tokens: deq }
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
    // 转换为表达式树
    pub fn parser(&mut self) -> MathExpr {
        // 第一次读取表达式一定是运算符或者括号
        let mut op = match self.next() {
            Token::Number(v) => MathExpr::Number(v.parse().expect("Invalid expression")),
            // 如果是操作符就创建一颗树
            Token::Operator(v) => MathExpr::Operator(v, vec![]),
            Token::LeftBracket(_) => self.parser(),
            // 第一个表达式不可能为右括号
            Token::RightBracket(_) => panic!("Invalid expression,first token should is operator or left-bracket."),
            // 第一个表达式不可能为空，数学表达式只应该返回数字
            Token::Eof => panic!("expression can't be empty."),
        };
        if let MathExpr::Operator(_, v) = &mut op {
            loop {
                if let Token::Eof = self.peek() {
                    break;
                } else {
                    let value = match self.peek() {
                        Token::Number(v) => {
                            MathExpr::Number(v.parse().expect("Invalid expression,the number is invalid"))
                        }
                        Token::Operator(v) => {
                            panic!("Invalid expression,expecting a number,but get one '{}'.", v)
                        }
                        Token::LeftBracket(v) => {
                            // 跳过这个左括号，可以少递归一层
                            self.next();
                            Self::parser(self)
                        }
                        Token::RightBracket(v) => {
                            return op;
                        }
                        Token::Eof => {
                            panic!("Invalid expression,expecting a number,but get one 'Eof'.")
                        }
                    };
                    v.push(value);
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
pub fn calc<I: Iterator<Item = MathExpr>>(op: char, v: &mut I) -> Num {
    let mut first = match v.next().expect("Invalid expression") {
        MathExpr::Number(v) => v.to_owned(),
        MathExpr::Operator(op, math_exprs) => calc(op.to_owned(), &mut math_exprs.into_iter()),
    };
    for i in v {
        let value = match i {
            MathExpr::Number(v) => v,
            MathExpr::Operator(op, math_exprs) => calc(op, &mut math_exprs.into_iter()),
        };
        first = eval_op(first, op, value)
    }
    first
}
pub fn eval_op(x: Num, op: char, y: Num) -> Num {
    match op {
        '+' => x + y,
        '-' => x - y,
        '/' => x / y,
        '*' => x * y,
        '%' => x % y,
        '^' => x.pow(y),
        _ => panic!("Unknown character"),
    }
}
#[test]
fn test() {
    let mut tokens = Tokens::new("+ 1 2".to_string());
    println!("{:?}", tokens);
    let expr = tokens.parser();
    println!("{:?}", expr);
    let v = expr.result();
    assert_eq!(3, v);
    println!("{:?}", v);
}
#[test]
fn bracket() {
    let mut tokens = Tokens::new("(+ 1 2)".to_string());
    println!("{:?}", tokens);
    let expr = tokens.parser();
    println!("{:?}", expr);
    let v = expr.result();
    assert_eq!(3, v);
    println!("{:?}", v);
}
#[test]
fn long_number() {
    let mut tokens = Tokens::new("+ 12 2 2".to_string());
    println!("{:?}", tokens);
    let expr = tokens.parser();
    println!("{:?}", expr);
    let v = expr.result();
    assert_eq!(16, v);
    println!("{:?}", v);
}
#[test]
fn operator_test() {
    let mut tokens = Tokens::new("^ (% 8 3) 2".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.result();
    assert_eq!(4, v);
    println!("{:?}", v);
}

#[test]
fn long_expression() {
    let mut tokens = Tokens::new("+ (% 18 2) (+ 5 2)".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.result();
    assert_eq!(7, v);
    println!("{:?}", v);
}
#[test]
fn long_expression2() {
    let mut tokens = Tokens::new("* 22 4 (+ 0 0 0)".to_string());
    println!("tokens: {:?}", tokens);
    let expr = tokens.parser();
    println!("expr: {:?}", expr);
    let v = expr.result();
    assert_eq!(0, v);
    println!("{:?}", v);
}
