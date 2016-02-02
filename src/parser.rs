/*
    Parser for imperative language.
    Implement Drop for parse tree without unbounded recursion.
*/
use lexer::{Scanner, Token};

struct Parser {
    lookahead : Token,
    scanner : Scanner,
    root : Option<Box<Program>>,
}

trait ParseNode {
    fn parse(&Parser) -> Box<Self>;
}

struct Program {
    block : Box<Block>,
}

impl ParseNode for Program {
    fn parse(parser : &Parser) -> Box<Self> {
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
    fn parse(parser : &Parser) -> Box<Self> {
        let decls = Vec::new();
        let stmts = Vec::new();

        while parser.lookahead != Token::Eof {
            match parser.lookahead {
                Type(btype) => decls.push(Type()),
                _           => decls.push(Statement()),
            };
        }
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

enum Type {
    Array(Box<Type>, i32),
    Basic, // TODO: what is a basic type?
}

enum Statement {
    Assign(Box<Loc>, Box<BoolExpr>),
    If(Box<BoolExpr>, Box<Statement>),
    IfElse(Box<BoolExpr>, Box<Statement>, Box<Statement>),
    While(Box<BoolExpr>, Box<Statement>),
    Break,
    Block,
}

enum Loc {
    Index(Box<Loc>, Box<BoolExpr>),
    ID(String),
}

enum BoolExpr {
    BoolNotImplemented,
}


impl Parser {
    fn new(scanner : Scanner) -> Self {
        Parser {
            lookahead : scanner.scan(),
            scanner : scanner,
            root : None,
        }
    }

    fn parse(&self) {
        //self.root = Some(Box::New(Program::parse(&self.scanner)));
    }
}
