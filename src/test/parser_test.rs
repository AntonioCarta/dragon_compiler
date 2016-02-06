#[cfg(test)]
mod test {
    use parser::{ParseNode, BoolExpr as B, NumExpr as N, Program, Type,
                 Loc, Statement as S, Decl as D, Relop, Block};
    use lexer;
    use parser;
        
    type SAst = Box<S>;
    type BAst = Box<B>;
    type NAst = Box<N>;
    
    fn parse_string(s : &str) -> Option<Box<Program>> {
        let scanner = lexer::Scanner::new_static(String::from(s));
        let mut parser = parser::Parser::new(scanner);
        parser.parse();
        parser.ast_root
    }
    
    fn parse_stmt(s : &str) -> Box<S> {
        let scanner = lexer::Scanner::new_static(String::from(s));
        let mut parser = parser::Parser::new(scanner);
        let res = S::parse(&mut parser);
        res
    }
      
    fn parse_bool(s : &str) -> BAst {
        let scanner = lexer::Scanner::new_static(String::from(s));
        let mut parser = parser::Parser::new(scanner);
        let res = B::parse(&mut parser);
        res
    }
    
    fn parse_expr(s : &str) -> NAst {
        let scanner = lexer::Scanner::new_static(String::from(s));
        let mut parser = parser::Parser::new(scanner);
        let res = N::parse(&mut parser);
        res
    }
    
    fn parse_decl(s : &str) -> Box<D> {
        let scanner = lexer::Scanner::new_static(String::from(s));
        let mut parser = parser::Parser::new(scanner);
        let res = D::parse(&mut parser);
        res
    }
    
    fn nbox<T>(x : T) -> Box<T> { Box::new(x) }
    fn ast_ide(s : &str) -> NAst { Box::new(N::Loc(Box::new(Loc::Ide(String::from(s))))) }
    fn ast_num(x : u32) -> NAst { Box::new(N::Num(x)) }    
    fn bnumexpr(x : NAst) -> BAst { nbox(B::NumExpr(x)) }
    fn add(e1 : NAst, e2 : NAst) -> NAst { nbox(N::Add(e1, e2)) }
    fn btrue() -> BAst { nbox(B::NumExpr(nbox(N::True))) }
    
    fn assign() -> Box<S> { 
        nbox(S::Assign(nbox(
            Loc::Ide(String::from("y"))),
            bnumexpr(ast_ide("x"))
        )) 
    }
    
    fn prog(decls : Vec<Box<D>>, stmts : Vec<Box<S>>) -> Option<Box<Program>> { 
        Some(nbox( Program {
            block : Box::new(Block {
                decls : decls,
                stmts : stmts,
            })
        })) 
    }
    
    #[test]
    fn numexpr_test() {
        let s = "1+3";  // Add
        let ast = nbox(N::Add(ast_num(1), ast_num(3)));
        assert_eq!(ast, parse_expr(s));
        
        let s = "-1*y"; // Mul, Minus
        let ast = nbox(N::Mul(
            nbox(N::Minus(ast_num(1))),
            ast_ide("y")
        ));
        assert_eq!(ast, parse_expr(s));
        
        let s = "(1/!y)"; // Not, Div, SubExpr
        let ast = nbox(N::Expr(nbox(B::NumExpr(nbox(N::Div(
            ast_num(1),
            nbox(N::Not(ast_ide("y")))
        ))))));
        assert_eq!(ast, parse_expr(s));
    }
    
    #[test]
    fn bool_test() {
        let btrue = nbox(B::NumExpr(nbox(N::True)));
        let bfalse = nbox(B::NumExpr(nbox(N::False)));
        
        // True, False, Or
        let s = "True || False"; 
        let ast = nbox(B::Or(
            btrue, bfalse
        ));
        assert_eq!(ast, parse_bool(s));
        
        // And, Eq        
        let s = "y && (3==5)";
        let ast = nbox(B::And(
            bnumexpr(ast_ide("y")),
            bnumexpr(nbox(N::Expr(
                nbox(B::Eq(
                    bnumexpr(ast_num(3)), 
                    bnumexpr(ast_num(5))
                ))
            )))
        ));
        assert_eq!(ast, parse_bool(s));
        
        // Neq, Leq        
        let s = "(3!=5) <= y";
        let ast = nbox(B::Relop(
            Relop::Leq,
            nbox(N::Expr(
                nbox(B::Neq(
                    bnumexpr(ast_num(3)), 
                    bnumexpr(ast_num(5))
                ))
            )),
            ast_ide("y")            
        ));
        assert_eq!(ast, parse_bool(s));
    }
    
    #[test]
    fn decl_test() {   
        let s = "int x;";        
        let ast = nbox(D {
            type_id : nbox(Type::Int),
            id : nbox(String::from("x")),
        });
        assert_eq!(ast, parse_decl(s));
        
        let s = "float[3] x;";
        let ast = nbox(D {
            type_id : nbox(Type::Array(nbox(Type::Float), vec![3])),
            id : nbox(String::from("x")),
        });
        assert_eq!(ast, parse_decl(s));
    }
    
    #[test]
    fn stmt_test() {
        // Assign                
        let s = "y=x;";
        let ast = assign();
        assert_eq!(ast, parse_stmt(s));
        
        // IfThenElse
        let s = "if(True) y=x; else y=x;";
        let ast = nbox(S::IfElse(
            btrue(),
            assign(),
            assign()
        ));
        assert_eq!(ast, parse_stmt(s));
        
        // While
        let s = "while(True) y=x;";
        let ast = nbox(S::While(
            btrue(),
            assign(),
        ));
        assert_eq!(ast, parse_stmt(s));
    }
    
    #[test]
    fn prog_test() {
        let s = "{int x; y=x;}";
        let ast = prog(
            vec![nbox(D {
                type_id : nbox(Type::Int),
                id : nbox(String::from("x")),
            })],
            vec![nbox(S::Assign(
                nbox(Loc::Ide(String::from("y"))),
                bnumexpr(ast_ide("x"))
            ))]
        );
        assert_eq!(ast, parse_string(s));
    }
}
