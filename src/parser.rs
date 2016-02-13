/*
    Parser for imperative language.
*/
use lexer::{Scanner, Token, Tag, TokenInfo};
use ast::statement::Program;
use code_generator::CodeGenerator;
use std;

pub struct Parser {
    pub lookahead : Token,
    pub scanner : Scanner,
    pub ast_root : Option<Box<Program>>,
}

pub trait ParseNode {
    fn parse(& mut Parser) -> Box<Self>;
}


impl Parser {
    pub fn new(mut scanner : Scanner) -> Self {
        Parser {
            lookahead : scanner.scan(),
            scanner : scanner,
            ast_root : None,
        }
    }

    pub fn parse(&mut self) {
        self.ast_root = Some(Program::parse(self));
    }    
    
    pub fn get_root(&mut self) -> Box<Program>{
        std::mem::replace(&mut self.ast_root, None).unwrap()
    }

    pub fn match_lookahead(&mut self, tag : Tag) -> Token {
        if self.lookahead.tag == tag {
            self.shift_lookahead()
        } else {
            panic!("Expected {:?}. Found: {:?}", tag, (&self.lookahead.tag, &self.lookahead.info))
        }
    }

    pub fn shift_lookahead(&mut self) -> Token {
        let x = Token::new(Tag::Error, TokenInfo::NoInfo);
        let out = std::mem::replace(&mut self.lookahead, x);
        self.lookahead = self.scanner.scan();
        out
    }
}
