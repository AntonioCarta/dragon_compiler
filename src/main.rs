mod lexer;
mod parser;
mod test;
mod intermediate_code;

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
    while tok.tag != lexer::Tag::Eof {
        println!("{:?}", tok.tag);
        tok = scanner.scan();
    }
}
