
use crate::token::{Expr, Num, TokensError};

pub fn head(tokens: &[Expr]) -> &Expr {
    let expr = &tokens[0];
    match &expr {
        // 获取内部的表达式，
        Expr::QExpr(tokens) => &tokens[0],
        _ => unreachable!(),
    }
}
pub fn add(tokens: &[Expr]) -> Expr {
    let mut sum = Expr::Number(0);
    for i in tokens {
        let temp = match i {
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
        };
        match &mut sum {
            Expr::Number(v) => {
                *v += temp;
            }
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    sum
}
pub fn sub(tokens: &[Expr]) -> Expr {
    let mut sum = Expr::Number(0);
    for i in tokens {
        let temp = match i {
            Expr::Number(v) => *v,
            Expr::Operator(op, exprs) => {
                let r = Expr::switch(op, exprs);
                match r {
                    Expr::Number(v) => v,
                    _ => panic!("{}", TokensError::Invalid),
                }
            }
            _ => {
                panic!();
            }
        };
        match &mut sum {
            Expr::Number(v) => {
                *v -= temp;
            }
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    sum
}
pub fn mul(tokens: &[Expr]) -> Expr {
    let mut sum = Expr::Number(0);
    for i in tokens {
        let temp = match i {
            Expr::Number(v) => *v,
            Expr::Operator(op, exprs) => {
                let r = Expr::switch(op, exprs);
                match r {
                    Expr::Number(v) => v,
                    _ => panic!("{}", TokensError::Invalid),
                }
            }
            _ => {
                panic!();
            }
        };
        match &mut sum {
            Expr::Number(v) => {
                *v *= temp;
            }
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    sum
}
pub fn div(tokens: &[Expr]) -> Expr {
    let mut value = Expr::Number(0);
    for i in tokens {
        let temp = match i {
            Expr::Number(v) => *v,
            Expr::Operator(op, exprs) => {
                let r = Expr::switch(op, exprs);
                match r {
                    Expr::Number(v) => v,
                    _ => panic!("{}", TokensError::Invalid),
                }
            }
            _ => {
                panic!();
            }
        };
        match &mut value {
            Expr::Number(v) => {
                *v /= temp;
            }
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    value
}
pub fn min(tokens: &[Expr]) -> Expr {
    let mut value = Expr::Number(Num::MAX);
    for i in tokens {
        let temp = match i {
            Expr::Number(v) => *v,
            Expr::Operator(op, exprs) => {
                let r = Expr::switch(op, exprs);
                match r {
                    Expr::Number(v) => v,
                    _ => panic!("{}", TokensError::Invalid),
                }
            }
            _ => {
                panic!();
            }
        };
        match &mut value {
            Expr::Number(v) => {
                if *v < temp {
                    *v = temp;
                }
            }
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    value
}
pub fn max(tokens: &[Expr]) -> Expr {
    let mut value = Expr::Number(Num::MIN);
    for i in tokens {
        let temp = match i {
            Expr::Number(v) => *v,
            Expr::Operator(op, exprs) => {
                let r = Expr::switch(op, exprs);
                match r {
                    Expr::Number(v) => v,
                    _ => panic!("{}", TokensError::Invalid),
                }
            }
            _ => {
                panic!();
            }
        };
        match &mut value {
            Expr::Number(v) => {
                if *v < temp {
                    *v = temp;
                }
            }
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    value
}
pub fn mmod(tokens: &[Expr]) -> Expr {
    let mut value = Expr::Number(Num::MIN);
    for i in tokens {
        let temp = match i {
            Expr::Number(v) => *v,
            Expr::Operator(op, exprs) => {
                let r = Expr::switch(op, exprs);
                match r {
                    Expr::Number(v) => v,
                    _ => panic!("{}", TokensError::Invalid),
                }
            }
            _ => {
                panic!();
            }
        };
        match &mut value {
            Expr::Number(v) => *v %= temp,
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    value
}
pub fn mi(tokens: &[Expr]) -> Expr {
    let mut value = Expr::Number(Num::MIN);
    for i in tokens {
        let temp = match i {
            Expr::Number(v) => *v,
            Expr::Operator(op, exprs) => {
                let r = Expr::switch(op, exprs);
                match r {
                    Expr::Number(v) => v,
                    _ => panic!("{}", TokensError::Invalid),
                }
            }
            _ => {
                panic!();
            }
        };
        match &mut value {
            Expr::Number(v) => *v = v.pow(temp),
            _ => {
                panic!("{}", TokensError::Invalid)
            }
        }
    }
    value
}
