mod lexer;

fn main() {
    println!("Hello, world!");
    let scanner = lexer::Scanner::new();
    scanner.scan();
}
