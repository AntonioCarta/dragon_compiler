use parser;

enum OpCode {
    Arr,
    // Boolean operators.
    And,
    Or,
    Not,
    // Num operators.
    Add,
    Sub,
    Mul,
    Div,
    // Jump.
    Goto,
    JmpZ,
    JmpNZ,
}
/*
struct AddressCode {
    op  : OpCode,
    res : &str, 
    x   : &str,
    y   : &str,
}*/