use lexer;
use lexer::{Scanner, Token, Tag};

fn scan_string(s : String) -> Vec<lexer::Token> {
    let mut scanner = lexer::Scanner::new_static(s);
    let mut v = vec![];
    let mut tok = scanner.scan();
    while tok.tag != lexer::Tag::Eof {
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
        Tag::LParen,
        Tag::Num,
        Tag::NumOP,
        Tag::Num,
        Tag::RParen,
        Tag::SemiColon,
    ];
    //assert!(res == sol);
}
