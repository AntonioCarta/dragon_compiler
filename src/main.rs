use lexer::Token;
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


fn scan_string(s : String) -> Vec<lexer::Token> {
    let mut scanner = lexer::Scanner::new_static(s);
    let mut v = vec![];
    let mut tok = scanner.scan();
    while tok != lexer::Token::Eof {
        v.push(tok);
        tok = scanner.scan();
    }
    v
}

#[test]
fn test_1() {
    let mut s = String::new();
    s.push_str("(1+3);");
    let res = scan_string(s);
    let sol = vec![
        Token::LParen,
        Token::Num(1),
        Token::NumOP(lexer::Numop::Add),
        Token::Num(3),
        Token::RParen,
        Token::SemiColon
    ];
    assert!(res == sol);
}
