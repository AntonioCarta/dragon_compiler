use parser::{ParseNode, Parser};
use lexer::{TokenInfo, Tag};
use ast::expression::{BoolExpr, Loc};
use code_generator::CodeGenerator;

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

impl Block {
    fn generate_code(&self, code_gen : &mut CodeGenerator) {
        code_gen.sym_table.push_frame();
        for d in &self.decls {
            d.generate_code(code_gen);
        }
        for v in &self.stmts {
            v.generate_code(code_gen);
        }
        code_gen.sym_table.pop_frame();
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

impl Decl {
    fn generate_code(&self, code_gen : &mut CodeGenerator) {
        code_gen.sym_table.put((*self.id).clone(), (*self.type_id).clone());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum BasicType { Int, Float }

#[derive(PartialEq, Debug, Clone)]
pub struct Type {
    basic_type : BasicType,
    array_dim  : u32,
}

impl ParseNode for Type {
    fn parse(parser : &mut Parser) -> Box<Self> {
        match parser.lookahead.tag {
            Tag::Type =>{
                let base = match parser.shift_lookahead().info {
                    TokenInfo::Int   => BasicType::Int,
                    TokenInfo::Float => BasicType::Float,
                    _ => unreachable!("Wrong info for Type token.")
                };
                let mut w : u32 = 4;                
                while parser.lookahead.tag == Tag::LArrParen {
                    // type -> type[num]
                    parser.shift_lookahead();
                    let n = parser.match_lookahead(Tag::Num);
                    if let TokenInfo::Num(x) = n.info {
                        w = w*x;
                    } else { unreachable!("Wrong info for num inside token"); }
                    parser.match_lookahead(Tag::RArrParen);
                }
                
                Box::new(Type {
                    basic_type : base,
                    array_dim  : w,
                })
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

impl Statement {
    fn generate_code(&self, code_gen : &mut CodeGenerator) {
        unimplemented!();
    }
}