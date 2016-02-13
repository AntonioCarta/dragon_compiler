use std::collections::linked_list::LinkedList;
use parser::Parser;
use symbol_table::SymbolTable;

enum OpCode {
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

struct Address {
    place : i32,
}

struct AddressCode {
    op  : OpCode,
    res : Address, 
    x   : Address,
    y   : Address,
}

struct IntermediateRepresentation {
    instructions : LinkedList<AddressCode>,
}

impl IntermediateRepresentation {
    fn new() -> Self {
        IntermediateRepresentation {
            instructions : LinkedList::new(),
        }
    }
    
    fn append(&mut self, addr_code : AddressCode) {
        self.instructions.push_back(addr_code);
    }
}

pub struct CodeGenerator {
    pub sym_table : SymbolTable, 
    parser    : Parser,    
    code      : IntermediateRepresentation,
}

impl CodeGenerator {
    fn new(parser : Parser) -> Self{
        CodeGenerator {
            parser    : parser,
            sym_table : SymbolTable::new(),
            code      : IntermediateRepresentation::new(),
        }
    }
    
    fn emit(&mut self, op : OpCode, res : Address, x : Address, y : Address) {
        let instr = AddressCode {
            op  : op,
            res : res, 
            x   : x,
            y   : y,
        };
        self.code.append(instr);
    }
}