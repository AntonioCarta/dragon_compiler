use std::collections::HashMap

enum Tag {
    Char(i32),
    BaseType(Type),
    RelOP(Rel),
    And,
    Break,
    Do,
    Else,
    Equal,
    False,
    Id,
    If,
    Index,
    Minus,
    Num,
    Or,
    Real,
    Temp,
    True,
    While,
}

struct Token {
    tag : Tag,
    lexeme: String;
}

let words = new HashMap();


/* Scan the input until it finds a token. */
fn scan() -> Token {

}
