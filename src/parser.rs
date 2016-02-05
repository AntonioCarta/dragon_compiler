/*
    Parser for imperative language.
    Implement Drop for parse tree without unbounded recursion.
*/
use lexer::{Scanner, Token, Tag, TokenInfo};
use std;

struct Parser {
    lookahead : Token,
    scanner : Scanner,
    root : Option<Box<Program>>,
}

trait ParseNode {
    fn parse(&mut Parser) -> Box<Self>;
}

struct Program {
    block : Box<Block>,
}

impl ParseNode for Program {
    fn parse(parser : &mut Parser) -> Box<Self> {
        Box::new(Program {
            block : Block::parse(parser),
        })
    }
}

struct Block {
    decls : Vec<Box<Decl>>,
    stmts : Vec<Box<Statement>>,
}

//TODO: Parse for Block
impl ParseNode for Block {
    fn parse(parser : &mut Parser) -> Box<Self> {
        let mut decls = Vec::new();
        let mut stmts = Vec::new();
        parser.match_lookahead(Tag::LArrParen);
        match parser.lookahead.tag {
            Tag::Type => decls.push(Decl::parse(parser)),
            _              => stmts.push(Statement::parse(parser)),
        };
        parser.match_lookahead(Tag::RArrParen);
        Box::new(Block {
            decls : decls,
            stmts : stmts,
        })
    }
}

struct Decl {
    typeId : Box<Type>,
    id : Box<String>,
}

impl ParseNode for Decl {
    fn parse(parser : &mut Parser) -> Box<Self> {
        let tid = Type::parse(parser);
        match parser.lookahead.tag  {
            Tag::Ide => {
                    let t = parser.shift_lookahead();
                    parser.match_lookahead(Tag::SemiColon);
                    Box::new(Decl {
                        typeId : tid,
                        id : Box::new(String::new()), //BUG TODO: save correct string.
                    })},
            _ => panic!()//Syntax error
        }
    }
}

enum Type {
    Array(Box<Type>, i32),
    Int,
    Float,
}

//TODO: just parsing the basic case.
impl ParseNode for Type {
    fn parse(parser : &mut Parser) -> Box<Self> {
        match (&parser.lookahead.tag, &parser.lookahead.info) {
            (&Tag::Type, &TokenInfo::Int) =>{
                parser.shift_lookahead();
                Box::new(Type::Int)
            },
            (&Tag::Type, &TokenInfo::Float) => {
                parser.shift_lookahead();
                Box::new(Type::Float)
            },
            _   => panic!()
        }
    }
}

enum Statement {
    Assign(Box<Loc>, Box<BoolExpr>),
    If(Box<BoolExpr>, Box<Statement>),
    IfElse(Box<BoolExpr>, Box<Statement>, Box<Statement>),
    While(Box<BoolExpr>, Box<Statement>),
    Break,
    BlockStmt(Box<Block>),
}

impl ParseNode for Statement {
    fn parse(parser : &mut Parser) -> Box<Self> {
        match parser.lookahead.tag {
            Tag::Ide => {
                let l = Loc::parse(parser);
                parser.match_lookahead(Tag::Assign);
                let b = BoolExpr::parse(parser);
                parser.match_lookahead(Tag::SemiColon);
                Box::new(Statement::Assign(l, b))
            },
            Tag::If => {
                parser.shift_lookahead();
                parser.match_lookahead(Tag::LParen);
                let b = BoolExpr::parse(parser);
                parser.match_lookahead(Tag::RParen);
                let s = Statement::parse(parser);
                if parser.lookahead.tag == Tag::Else {
                    parser.shift_lookahead();
                    let s2 = Statement::parse(parser);
                    Box::new(Statement::IfElse(b, s, s2))
                } else {
                    Box::new(Statement::If(b, s))
                }
            },
            Tag::While => {
                parser.shift_lookahead();
                parser.match_lookahead(Tag::LParen);
                let b = BoolExpr::parse(parser);
                parser.match_lookahead(Tag::RParen);
                let s = Statement::parse(parser);
                Box::new(Statement::While(b, s))
            },
            Tag::Break => {
                parser.shift_lookahead();
                parser.match_lookahead(Tag::SemiColon);
                Box::new(Statement::Break)
            },
            Tag::LArrParen => {
                Box::new(Statement::BlockStmt(Block::parse(parser)))
            }
            _ => panic!()
        }
    }
}

enum Loc {
    Index(Box<Loc>, Box<BoolExpr>),
    ID(String),
}

impl ParseNode for Loc {
    fn parse(parser : &mut Parser) -> Box<Self> {
        unimplemented!();
    }
}

enum BoolExpr {
    BoolNotImplemented,
}

impl ParseNode for BoolExpr {
    fn parse(parser : &mut Parser) -> Box<Self> {
        unimplemented!();
    }
}

impl Parser {
    fn new(mut scanner : Scanner) -> Self {
        Parser {
            lookahead : scanner.scan(),
            scanner : scanner,
            root : None,
        }
    }

    fn parse(&mut self) {
        //self.root = Some(Box::New(Program::parse(&self.scanner)));
    }

    fn match_lookahead(&mut self, tag : Tag) -> Token {
        if self.lookahead.tag == tag {
            self.shift_lookahead()
        } else {
            panic!()
        }
    }

    fn shift_lookahead(&mut self) -> Token {
        let mut x = Token::new(Tag::Error, TokenInfo::NoInfo);
        let out = std::mem::replace(&mut self.lookahead, x);
        self.lookahead = self.scanner.scan();
        out
    }
}
