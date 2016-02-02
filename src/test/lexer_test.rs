use lexer;
use lexer::{Scanner, Token};

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
    let s = "(1+3);";
    let res = scan_string(s);
    let sol = vec![
        Token::LParen,
        Token::Num(1),
        Token::NumOP(Numop::Add),
        Token::Num(3),
        Token::RParen,
        Token::SemiColon,
    ];
    assert!(res == sol);
}
