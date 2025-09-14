use std::io::Write;

use crate::token::Tokens;

pub struct Terminal {}

impl Terminal {
    // 创面一个Terminal管理器
    pub fn new() -> Self {
        Self {}
    }
    pub fn print<S: AsRef<str>>(s: S) {
        print!("lispy>{}", s.as_ref());
        std::io::stdout().flush().expect("Faild");
    }
    pub fn println<S: AsRef<str>>(s: S) {
        println!("lispy>{}", s.as_ref())
    }
    pub fn run(&self) {
        loop {
            Terminal::print("");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed read user input");
            let ss = input.trim();
            let mut tokens = Tokens::new(ss.to_string());
            let expr = tokens.parser();
            Terminal::println(format!("calc result is '{}'", expr.calc()));
        }
    }
}
