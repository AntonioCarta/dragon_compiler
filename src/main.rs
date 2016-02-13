mod lexer;
mod parser;
mod test;
mod symbol_table;
mod code_generator;
mod ast;

fn main() {
    println!("Mini Compiler.");

    let scanner = lexer::Scanner::new_static("{}".to_string());
    let parser = parser::Parser::new(scanner);
    let mut code_gen = code_generator::CodeGenerator::new(parser);
    
    code_gen.generate_code();
}
