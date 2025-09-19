use std::ffi::NulError;

use crate::token::{Expr, Num, TokensError};
pub fn extra_num(expr: &Expr) -> Num {
    match expr {
        Expr::Number(v) => *v,
        // 如果相加的是一个表达式
        Expr::Operator(op, exprs) => {
            let r = Expr::switch(op, exprs);
            match r {
                Expr::Number(v) => v,
                _ => panic!("{}", TokensError::Invalid),
            }
        }
        _ => {
            panic!("{}", TokensError::Invalid)
        }
    }
}
pub fn calc<F: Fn(Num, Num) -> Num>(tokens: &[Expr], init: Num, op_func: F) -> Num {
    let mut result = init;
    for i in tokens.iter() {
        let v = extra_num(i);
        result = op_func(result, v)
    }
    result
}
pub fn head(tokens: &[Expr]) -> &Expr {
    let expr = &tokens[0];
    match &expr {
        // 获取内部的表达式，
        Expr::QExpr(tokens) => &tokens[0],
        _ => unreachable!(),
    }
}
pub fn add(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(tokens, 0, |acc, x| acc + x))
}
pub fn sub(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(tokens, 0, |acc, x| acc - x))
}
pub fn mul(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(tokens, 1, |acc, x| acc * x))
}
pub fn div(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(tokens, 1, |acc, x| acc / x))
}
pub fn min(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(
        tokens,
        Num::MAX,
        |acc, x| {
            if acc > x { x } else { acc }
        },
    ))
}
pub fn max(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(
        tokens,
        Num::MIN,
        |acc, x| {
            if acc < x { x } else { acc }
        },
    ))
}
pub fn mmod(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(&tokens[1..], extra_num(&tokens[0]), |acc, x| acc % x))
}
pub fn mi(tokens: &[Expr]) -> Expr {
    Expr::Number(calc(&tokens[1..], extra_num(&tokens[0]), |acc, x| {
        acc.pow(x)
    }))
}
