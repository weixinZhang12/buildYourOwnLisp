#![allow(unused_variables)]
#![allow(dead_code)]

use crate::terminal::Terminal;
mod token;
mod terminal;
mod utils;
mod ac;
mod varmap;
mod calc;

fn main() {
    let terminal=Terminal::new();
    terminal.run();
}
