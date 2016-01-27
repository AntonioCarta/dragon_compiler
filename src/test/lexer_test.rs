use lexer::Token;

mod lexer;

fn scan_string(s : String) -> Vec<lexer::Token> {
    let scanner = lexer::Scanner(s);
    let v = vec![];
    while tok != lexer::Token::Eof {
        v.push();
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

fn test_2() {
    let s = "{5<x; 4==2()}";
    let v = scan_string(s);
}

fn test_3() {
    let s = "(1+3);"
}
