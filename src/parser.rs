/*
    Parser for imperative language.
*/
use lexer::{Scanner, Token, Tag, TokenInfo};
use std;

pub struct Parser {
    lookahead : Token,
    scanner : Scanner,
    pub ast_root : Option<Box<Program>>,
}

pub trait ParseNode {
    fn parse(& mut Parser) -> Box<Self>;
}

#[derive(PartialEq, Debug)]
pub struct Program {
    pub block : Box<Block>,
}

impl ParseNode for Program {
    fn parse(parser : &mut Parser) -> Box<Self> {
        //program -> block
        Box::new(Program {            
            block : Block::parse(parser),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Block {
    pub decls : Vec<Box<Decl>>,
    pub stmts : Vec<Box<Statement>>,
}

impl ParseNode for Block {
    fn parse(parser : &mut Parser) -> Box<Self> {
        let mut decls = Vec::new();
        let mut stmts = Vec::new();
        //block -> decls stmts
        parser.match_lookahead(Tag::OpenBlock);
        loop {
            match parser.lookahead.tag {
                // NOTE: this way we can mix stmts and declarations.
                Tag::Type => decls.push(Decl::parse(parser)),
                Tag::CloseBlock => break,
                _              => stmts.push(Statement::parse(parser)),
            };
        }
        parser.match_lookahead(Tag::CloseBlock);
        Box::new(Block {
            decls : decls,
            stmts : stmts,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Decl {
    pub type_id : Box<Type>,
    pub id : Box<String>,
}

impl ParseNode for Decl {
    fn parse(parser : & mut Parser) -> Box<Self> {
        //decl -> type ID;
        let tid = Type::parse(parser);
        match parser.lookahead.tag  {
            Tag::Ide => {
                    let s = {
                        if let TokenInfo::Ide(x) = parser.shift_lookahead().info {
                            x
                        } else { unreachable!("Wrong TokenInfo for Ide.") }
                    };
                    parser.match_lookahead(Tag::SemiColon);
                    Box::new(Decl {
                        type_id : tid,
                        id : Box::new(s), 
                    })},
            _ => panic!("Expecting identifier after type inside a declaration.")
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Array(Box<Type>, Vec<u32>),
    Int,
    Float,
}

impl ParseNode for Type {
    fn parse(parser : &mut Parser) -> Box<Self> {
        match parser.lookahead.tag {
            Tag::Type =>{
                let base = match parser.shift_lookahead().info {
                    TokenInfo::Int   => Box::new(Type::Int),
                    TokenInfo::Float => Box::new(Type::Float),
                    _ => unreachable!("Wrong info for Type token.")
                };
                let mut v = Vec::new();                
                while parser.lookahead.tag == Tag::LArrParen {
                    // type -> type[num]
                    parser.shift_lookahead();
                    let n = parser.match_lookahead(Tag::Num);
                    if let TokenInfo::Num(x) = n.info {
                        v.push(x);
                    } else { unreachable!("Wrong info for num inside token"); }
                    parser.match_lookahead(Tag::RArrParen);
                }
                if v.len() == 0 {
                    // type -> basic
                    base
                } else {
                    // type -> type[num]
                    Box::new(Type::Array(base, v))
                }
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
                // stmt -> loc = bool
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
                    // stmt -> if (bool) stmt else stmt
                    parser.shift_lookahead();
                    let s2 = Statement::parse(parser);
                    Box::new(Statement::IfElse(b, s, s2))
                } else {
                    // stmt -> if (bool) stmt
                    Box::new(Statement::If(b, s))
                }
            },
            Tag::While => {
                // stmt -> while (bool) stmt
                parser.shift_lookahead();
                parser.match_lookahead(Tag::LParen);
                let b = BoolExpr::parse(parser);
                parser.match_lookahead(Tag::RParen);
                let s = Statement::parse(parser);
                Box::new(Statement::While(b, s))
            },
            Tag::Break => {
                // stmt -> break;
                parser.shift_lookahead();
                parser.match_lookahead(Tag::SemiColon);
                Box::new(Statement::Break)
            },
            Tag::LArrParen => {
                // stmt -> block
                Box::new(Statement::BlockStmt(Block::parse(parser)))
            }
            _ => panic!("Expected a valid statement.")
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
            while parser.lookahead.tag == Tag::LArrParen {
                parser.shift_lookahead();
                let b = BoolExpr::parse(parser);
                v.push(b);
                parser.match_lookahead(Tag::RArrParen);
            }
            if let TokenInfo::Ide(s) = inf {
                if v.len() > 0 {
                    // loc -> loc[bool]
                    Box::new(Loc::Index(s, v))
                } else {
                    // loc -> ID
                    Box::new(Loc::Ide(s))
                }
            } else {
                unreachable!("Wrong token info inside identifier.")
            }
        } else {
            panic!("Expected ide inside lvalue. Found: {:?}", parser.lookahead.tag)
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
        // bool -> join
        let mut x = BoolExpr::join(parser);
        while parser.lookahead.info == TokenInfo::Or {
            // bool -> join || bool
            parser.shift_lookahead();
            x = Box::new(BoolExpr::Or(x, BoolExpr::join(parser)));
        }
        x
    }
}

impl BoolExpr {
    fn join(parser : &mut Parser) -> Box<Self> {
        // join -> equ
        let mut x = BoolExpr::equality(parser);
        while parser.lookahead.info == TokenInfo::And {
            // join -> join && equality
            parser.shift_lookahead();
            x = Box::new(BoolExpr::And(x, BoolExpr::equality(parser)));
        }
        x
    }

    fn equality(parser : &mut Parser) -> Box<Self> {
        // equ -> rel
        let mut x = BoolExpr::rel(parser);
        while parser.lookahead.tag == Tag::RelOp {
            if parser.lookahead.info == TokenInfo::Equ {
                // equ -> rel == equ
                parser.shift_lookahead();
                x = Box::new(BoolExpr::Eq(x, BoolExpr::rel(parser)));
            } else if parser.lookahead.info == TokenInfo::Neq {
                // equ -> rel != equ
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
                // rel -> expr >= expr
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Ge, x, NumExpr::parse(parser)))
            }
            TokenInfo::Gr => {
                // rel -> expr > expr
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Gr, x, NumExpr::parse(parser)))
            }
            TokenInfo::Leq => {
                // rel -> expr <= expr
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Leq, x, NumExpr::parse(parser)))
            }
            TokenInfo::Les => {
                // rel -> expr < expr
                parser.shift_lookahead();
                Box::new(BoolExpr::Relop(Relop::Les, x, NumExpr::parse(parser)))
            }
            _ => Box::new(BoolExpr::NumExpr(x)) // rel -> expr
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
                    // expr -> term + expr
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Add(x, NumExpr::term(parser)));
                },
                TokenInfo::Sub => {
                    // expr -> term - expr
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Sub(x, NumExpr::term(parser)));
                },
                _ => break, // expr -> term
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
                    // term -> unary * term
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Mul(x, NumExpr::unary(parser)));
                },
                TokenInfo::Div => {
                    // term -> unary / term
                    parser.shift_lookahead();
                    x = Box::new(NumExpr::Div(x, NumExpr::unary(parser)));
                },
                _ => break, // term -> unary
            }
        }
        x
    }

    fn unary(parser : &mut Parser) -> Box<Self> {
        match parser.lookahead.info {
            TokenInfo::Sub => {
                // unary -> -unary
                parser.shift_lookahead();
                Box::new(NumExpr::Minus(NumExpr::unary(parser)))
            }, 
            TokenInfo::Not => {
                // unary -> !unary
                parser.shift_lookahead();
                Box::new(NumExpr::Not(NumExpr::unary(parser)))
            },   
            _ => NumExpr::factor(parser), // unary -> factor
        }
    }

    fn factor(parser : &mut Parser) -> Box<Self> {
        match parser.lookahead.tag {
            // factor -> loc 
            Tag::Ide => Box::new(NumExpr::Loc(Loc::parse(parser))),             
            Tag::Num => {
                // factor -> num
                if let TokenInfo::Num(x) = parser.shift_lookahead().info {
                    Box::new(NumExpr::Num(x))
                } else { unreachable!("Wrong token info for num.") }
            },            
            Tag::True => {
                // factor -> True
                parser.shift_lookahead();
                Box::new(NumExpr::True)
            }, 
            Tag::False => {
                // factor -> False
                parser.shift_lookahead();
                Box::new(NumExpr::False)
            },            
            Tag::LParen => {
                // factor -> (bool)
                parser.shift_lookahead();
                let b = BoolExpr::parse(parser);
                parser.match_lookahead(Tag::RParen);
                Box::new(NumExpr::Expr(b))
            },
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

    fn match_lookahead_msg(&mut self, tag : Tag, error_msg : &str) -> Token {
        if self.lookahead.tag == tag {
            self.shift_lookahead()
        } else {
            panic!("{} found: {:?}", error_msg, self.lookahead.tag)
        }
    }

    fn match_lookahead(&mut self, tag : Tag) -> Token {
        if self.lookahead.tag == tag {
            self.shift_lookahead()
        } else {
            panic!("Expected {:?}. Found: {:?}", tag, (&self.lookahead.tag, &self.lookahead.info))
        }
    }

    fn shift_lookahead(&mut self) -> Token {
        let x = Token::new(Tag::Error, TokenInfo::NoInfo);
        let out = std::mem::replace(&mut self.lookahead, x);
        self.lookahead = self.scanner.scan();
        out
    }
}
