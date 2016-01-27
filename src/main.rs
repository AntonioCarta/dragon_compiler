use std::fmt::{self, Debug, Display};
use std::io;

mod lexer;

fn main() {
/*
    let buf = String::new();
    io::stdin().read_to_string(buf);

    let s = "asdasas";
    for c in s.chars() {
        println!("{}", c);
    }
*/
    println!("Mini Compiler.");
    let mut scanner = lexer::Scanner::new();
    let mut tok = scanner.scan();
    while tok != lexer::Token::Eof {
        println!("{:?}", tok);
        tok = scanner.scan();
    }
}
