#[cfg(test)]
mod test{
use lexer;
use lexer::{Scanner};

fn scan_string(s : String) -> String {
    let mut scanner = lexer::Scanner::new_static(s);
    let mut v = vec![];
    let mut tok = scanner.scan();
    while tok.tag != lexer::Tag::Eof {
        v.push(tok);
        tok = scanner.scan();
    }

    let mut s = String::new();
    for el in v {
        s.push_str(&el.to_cow_string());
    }
    s
}

#[test]
fn basic_test() {
    let s = "(1+3-x*y/543);"; 
    let res = scan_string(String::from(s));
    assert!(res == String::from(s));

    let s = "{if;while;()[]else;break}";
    let res = scan_string(String::from(s));
    assert!(res == String::from(s));
}
}
