#[cfg(test)]
mod test {
    use parser::{ParseNode, BoolExpr, NumExpr, Loc};
    use lexer;
    use parser;

    fn parse_string(s : String) -> parser::Parser {
        let scanner = lexer::Scanner::new_static(s);
        let mut parser = parser::Parser::new(scanner);
        parser.parse();
        parser
    }

    fn alloc_ide(s : &str) -> Box<NumExpr> {
        Box::new(NumExpr::Loc(Box::new(Loc::Ide(String::from(s)))))
    }

    #[test]
    fn basic_test() {
        let s = "1+3-x*y/543";
        let ast = Box::new(BoolExpr::NumExpr(
            Box::new(NumExpr::Add(
                Box::new(NumExpr::Num(1)),
                Box::new(NumExpr::Sub(
                    Box::new(NumExpr::Num(3)),
                    Box::new(NumExpr::Mul(
                        alloc_ide("x"),
                        Box::new(NumExpr::Div(
                            alloc_ide("y"),
                            Box::new(NumExpr::Num(543))
                        ))
                    ))
                ))
            ))
        ));
        let scanner = lexer::Scanner::new_static(String::from(s));
        let mut parser = parser::Parser::new(scanner);
        let res = parser::BoolExpr::parse(&mut parser);
        assert!(res == ast, "ERRORE ERRORE ERRORE");
    }
}
