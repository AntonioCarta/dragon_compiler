use std::collections::linked_list::LinkedList;
use parser::Parser;
use symbol_table::SymbolTable;

pub enum OpCode {
    Mov,
    // Boolean operators.
    And,
    Or,
    Not,
    // Num operators.
    Add,
    Sub,
    Mul,
    Div,
    Minus,
    // Jump.
    Goto,
    JmpZ,
    JmpNZ,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Address {
    place : i32,
}

impl Address {
    fn null_address() -> Self {
        Address { place : 0, }
    }
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
    temp_num  : i32,
}

impl CodeGenerator {
    pub fn new(parser : Parser) -> Self{
        CodeGenerator {
            parser    : parser,
            sym_table : SymbolTable::new(),
            code      : IntermediateRepresentation::new(),
            temp_num  : 0,
        }
    }
    
    pub fn emit(&mut self, op : OpCode, res : Address, x : Address, y : Address) {
        let instr = AddressCode {
            op  : op,
            res : res, 
            x   : x,
            y   : y,
        };
        self.code.append(instr);
    }
    
    pub fn new_temp(&mut self) -> Address {
        self.temp_num += 1;
        Address {
            place : self.temp_num,
        }
    }
}