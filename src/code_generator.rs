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
pub enum AddressMode {
    Register, Constant, Label,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Address {
    mode  : AddressMode,
    // Can be used as label when used as operand for jumps.
    place : i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Label {
    place : i32,
}

impl Address {
    pub fn null_address() -> Self {
        Address { 
            mode  : AddressMode::Label,
            place : 0, 
        }
    }
    
    pub fn new_constant(x : i32) -> Self {
        Address {
            mode  : AddressMode::Constant,
            place : x,
        }
    }
}

pub struct AddressCode {
    id  : i32,
    op  : OpCode,
    res : Address, 
    x   : Address,
    y   : Address,
}

struct IntermediateRepresentation {
    instructions : Vec<AddressCode>,
}

impl IntermediateRepresentation {
    fn new() -> Self {
        IntermediateRepresentation {
            instructions : Vec::new(),
        }
    }
    
    fn append(&mut self, addr_code : AddressCode) {
        self.instructions.push(addr_code);
    }
    
    fn get_last(&self) -> usize {
        let n = self.instructions.len();
        (n - 1)
    }
}

pub struct CodeGenerator {
    pub sym_table : SymbolTable, 
    parser    : Parser,    
    code      : IntermediateRepresentation,
    temp_num  : i32,
    instr_num : i32,
}

impl CodeGenerator {
    pub fn new(parser : Parser) -> Self{
        CodeGenerator {
            parser    : parser,
            sym_table : SymbolTable::new(),
            code      : IntermediateRepresentation::new(),
            temp_num  : 0,
            instr_num : 0,
        }
    }
    
    pub fn emit(&mut self, op : OpCode, res : Address, x : Address, y : Address) {
        self.instr_num += 1;
        let instr = AddressCode {
            op  : op,
            res : res, 
            x   : x,
            y   : y,
            id  : self.instr_num,
        };
        self.code.append(instr);
    }
    
    pub fn emit_label(&self) -> Label {
        Label {
            place : self.instr_num + 1,
        }
    }
    
    // Return the instruction because we might need to backpatch it later.
    pub fn emit_jump(&mut self, op : OpCode, lbl : Label, addr : Address) -> usize {
        let jump = Address { 
            mode  : AddressMode::Label,
            place : lbl.place, 
        };
        self.instr_num += 1;
        let instr = AddressCode {
            op  : op,
            res : jump, 
            x   : addr,
            y   : addr,
            id  : self.instr_num,
        };
        self.code.append(instr);
        self.code.get_last()
    }
    
    pub fn patch_jump(&mut self, addr : usize, lbl : Label) {
        self.code.instructions[addr].res.place = lbl.place;
    }
    
    pub fn new_temp(&mut self) -> Address {
        self.temp_num += 1;
        Address {
            mode  : AddressMode::Register,
            place : self.temp_num,
        }
    }
    
    pub fn generate_code(&mut self) {
        self.parser.parse();
        let root = self.parser.get_root();
        root.generate_code(self);        
    }
}