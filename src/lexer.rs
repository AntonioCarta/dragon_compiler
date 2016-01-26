use std::collections::HashMap

enum boolop {
    Or,
    And
}

enum Relop {
    Ge, Gr,
    Leq, les,
    Equ, Neq,
}

enum Unary {
    Minus,
    Not,
}

enum Numop {
    Add, Sub,
    Mul, Div,
}

enum Token {
    /* Reserved words. */
    If,
    Else,
    While,
    Break,
    /* Separators. */
    CloseBlock,
    OpenBlock,
    SemiColon,
    LArrParen, RArrParen, LParen, RParen,
    Assign,
    /* Operators. */
    BoolOP(BoolOP),
    RelOP(RelOP),
    Unary(Unary),
    NumOP(Numop),
    /* Identifiers ad Numbers. */
    Ide(String),
    Num(i32),
    Real(float),
}

/* This HashMap should save some space.
    If we scan a token already seen, we don't allocate new memory.
    This way we don't end up we thousands of If and While tokens.
*/
let createdTokens = new HashMap();


/* Scan the input until it finds a token. */
fn scan() -> Token {

}
