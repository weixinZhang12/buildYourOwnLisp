#![allow(unused_variables)]
#![allow(dead_code)]

use crate::terminal::Terminal;
mod token;
mod terminal;

fn main() {
    let terminal=Terminal::new();
    terminal.run();
}
