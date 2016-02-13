use code_generator::CodeGenerator;
use parser::{ParseNode, Parser};
use lexer::{TokenInfo, Tag};

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
    
    fn generate_code(&self, code_gen : &mut CodeGenerator) {
        unimplemented!()
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

    fn generate_code(&self, code_gen : &mut CodeGenerator) {
        unimplemented!()
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

    fn generate_code(&self, code_gen : &mut CodeGenerator) {
        unimplemented!()
    }
}
