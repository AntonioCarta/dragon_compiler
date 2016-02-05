/*
    Parser for imperative language.
    Implement Drop for parse tree without unbounded recursion.
*/
use lexer::{Scanner, Token, Tag, TokenInfo};
use std;

pub struct Parser {
    lookahead : Token,
    scanner : Scanner,
    ast_root : Option<Box<Program>>,
}

pub trait ParseNode {
    fn parse(&mut Parser) -> Box<Self>;
}

#[derive(PartialEq, Debug)]
pub struct Program {
    block : Box<Block>,
}

impl ParseNode for Program {
    fn parse(parser : &mut Parser) -> Box<Self> {
        Box::new(Program {
            block : Block::parse(parser),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Block {
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

#[derive(PartialEq, Debug)]
pub struct Decl {
    typeId : Box<Type>,
    id : Box<String>,
}

impl ParseNode for Decl {
    fn parse(parser : &mut Parser) -> Box<Self> {
        let tid = Type::parse(parser);
        match parser.lookahead.tag  {
            Tag::Ide => {
                    let s = {
                        if let TokenInfo::Ide(x) = parser.shift_lookahead().info {
                            x
                        } else { panic!("PWrong TokenInfo for Ide.") }
                    };
                    parser.match_lookahead(Tag::SemiColon);
                    Box::new(Decl {
                        typeId : tid,
                        id : Box::new(s), //BUG TODO: save correct string.
                    })},
            _ => panic!()//Syntax error
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Array(Box<Type>, Vec<i32>),
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
            _   => panic!("Wrong Token for Type")
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Statement {
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

#[derive(PartialEq, Debug)]
pub enum Loc {
    Index(String, Vec<Box<BoolExpr>>),
    Ide(String),
}

impl ParseNode for Loc {
    fn parse(parser : &mut Parser) -> Box<Self> {
        if parser.lookahead.tag == Tag::Ide {
            let inf = parser.shift_lookahead().info;
            let mut v = Vec::new();
            while parser.shift_lookahead().tag == Tag::LArrParen {
                // TODO: n in First(BoolExpr)
                if let TokenInfo::Num(_) = parser.lookahead.info {
                    let b = BoolExpr::parse(parser);
                    v.push(b);
                }
                parser.match_lookahead(Tag::RArrParen);
            }
            if let TokenInfo::Ide(s) = inf {
                if v.len() > 0 {
                    Box::new(Loc::Index(s, v))
                } else {
                    Box::new(Loc::Ide(s))
                }
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Relop {
    Ge, Gr,
    Leq, Les,
}

#[derive(PartialEq, Debug)]
pub enum BoolExpr {
    Or(Box<BoolExpr>, Box<BoolExpr>),
    And(Box<BoolExpr>, Box<BoolExpr>),
    Eq(Box<BoolExpr>, Box<BoolExpr>),
    Neq(Box<BoolExpr>, Box<BoolExpr>),
    Relop(Relop, Box<NumExpr>, Box<NumExpr>),
    NumExpr(Box<NumExpr>),
}

impl ParseNode for BoolExpr {
    fn parse(parser : &mut Parser) -> Box<Self> {
        let mut x = BoolExpr::join(parser);
        while parser.lookahead.info == TokenInfo::Or {
            parser.shift_lookahead();
            x = Box::new(BoolExpr::Or(x, BoolExpr::join(parser)));
        }
        x
    }
}

impl BoolExpr {
    fn join(parser : &mut Parser) -> Box<Self> {
        let mut x = BoolExpr::equality(parser);
        while parser.lookahead.info == TokenInfo::And {
            parser.shift_lookahead();
            x = Box::new(BoolExpr::And(x, BoolExpr::equality(parser)));
        }
        x
    }

    fn equality(parser : &mut Parser) -> Box<Self> {
        let mut x = BoolExpr::rel(parser);
        while parser.lookahead.tag == Tag::RelOp {
            if parser.lookahead.info == TokenInfo::Equ {
                parser.shift_lookahead();
                x = Box::new(BoolExpr::Eq(x, BoolExpr::rel(parser)));
            } else if parser.lookahead.info == TokenInfo::Neq {
                parser.shift_lookahead();
                x = Box::new(BoolExpr::Neq(x, BoolExpr::rel(parser)));
            }
        }
        x
    }

    fn rel(parser : &mut Parser) -> Box<Self> {
        let x = NumExpr::parse(parser);

        match parser.lookahead.info {
            TokenInfo::Ge => {
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Ge, x, NumExpr::parse(parser)))
            }
            TokenInfo::Gr => {
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Gr, x, NumExpr::parse(parser)))
            }
            TokenInfo::Leq => {
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Leq, x, NumExpr::parse(parser)))
            }
            TokenInfo::Les => {
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Les, x, NumExpr::parse(parser)))
            }
            _ => Box::new(BoolExpr::NumExpr(x))
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum NumExpr {
    Add(Box<NumExpr>, Box<NumExpr>),
    Sub(Box<NumExpr>, Box<NumExpr>),
    Mul(Box<NumExpr>, Box<NumExpr>),
    Div(Box<NumExpr>, Box<NumExpr>),
    Not(Box<NumExpr>),
    Minus(Box<NumExpr>),
    Expr(Box<BoolExpr>),
    Loc(Box<Loc>),
    Num(u32),
    True,
    False,
}

impl ParseNode for NumExpr {
    fn parse(parser : &mut Parser) -> Box<Self> {
        let mut x = NumExpr::term(parser);
        loop {
            match parser.lookahead.info {
                TokenInfo::Add => {
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Add(x, NumExpr::term(parser)));
                },
                TokenInfo::Sub => {
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Sub(x, NumExpr::term(parser)));
                },
                _ => break,
            }
        }
        x
    }
}

impl NumExpr {
    fn term(parser : &mut Parser) -> Box<Self> {
        let mut x = NumExpr::unary(parser);
        loop {
            match parser.lookahead.info {
                TokenInfo::Mul => {
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Mul(x, NumExpr::unary(parser)));
                },
                TokenInfo::Div => {
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Add(x, NumExpr::unary(parser)));
                },
                _ => break,
            }
        }
        x
    }

    fn unary(parser : &mut Parser) -> Box<Self> {
        match parser.lookahead.info {
            TokenInfo::Sub => Box::new(NumExpr::Minus(NumExpr::factor(parser))),
            TokenInfo::Not => Box::new(NumExpr::Not(NumExpr::factor(parser))),
            _ => NumExpr::factor(parser),
        }
    }

    fn factor(parser : &mut Parser) -> Box<Self> {
        match parser.lookahead.tag {
            Tag::Ide => Box::new(NumExpr::Loc(Loc::parse(parser))),
            Tag::Num => {
                if let TokenInfo::Num(x) = parser.lookahead.info {
                    Box::new(NumExpr::Num(x))
                } else { panic!() }
            },
            Tag::True => Box::new(NumExpr::True),
            Tag::False => Box::new(NumExpr::False),
            Tag::LParen => {
                parser.shift_lookahead();
                let b = BoolExpr::parse(parser);
                parser.match_lookahead(Tag::RParen);
                Box::new(NumExpr::Expr(b))
            }
            _ => panic!("Wrong token for factor: {:?}", parser.lookahead.tag),
        }
    }
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

    fn match_lookahead(&mut self, tag : Tag) -> Token {
        if self.lookahead.tag == tag {
            self.shift_lookahead()
        } else {
            panic!("Lookahead doesn't match: {:?}", self.lookahead.tag)
        }
    }

    fn shift_lookahead(&mut self) -> Token {
        let x = Token::new(Tag::Error, TokenInfo::NoInfo);
        let out = std::mem::replace(&mut self.lookahead, x);
        self.lookahead = self.scanner.scan();
        out
    }
}
