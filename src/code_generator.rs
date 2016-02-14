use parser::Parser;
use symbol_table::SymbolTable;

pub enum OpCode {
    Mov,
    // Boolean operators.
    And,
    Or,
    Not,
    IsPos,
    IsNeg,
    // Num operators.
    Add,
    Sub,
    Mul,
    Div,
    Minus,
    // Jump.
    Goto,
    JmpZ,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AddressMode {
    Register,       // Register containing a value.
    FramePointer,   // Register containing an address to stack variable.
    Constant,       // Constant value not in a register.
    Label,          // ID of an instruction for JUMP and CALL.
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Address {
    mode  : AddressMode,
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
    pub sym_table   : SymbolTable, 
    parser          : Parser,    
    code            : IntermediateRepresentation,
    temp_num        : i32,
    instr_num       : i32,
    
    stack_pointer   : Address,
}

impl CodeGenerator {
    pub fn new(parser : Parser) -> Self{
        let mut cg = CodeGenerator {
            parser    : parser,
            sym_table : SymbolTable::new(),
            code      : IntermediateRepresentation::new(),
            temp_num  : 0,
            instr_num : 0,
            
            stack_pointer : Address::null_address(),
        };
        cg.stack_pointer = cg.new_temp();
        cg
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
    
    pub fn push_frame(&mut self) {
        let w = self.sym_table.get_frame_width();
        let stp = self.stack_pointer.clone();
        self.sym_table.push_frame();
        self.emit(OpCode::Add, stp, stp, Address::new_constant(w as i32));
    }
    
    pub fn pop_frame(&mut self) {
        let w = self.sym_table.get_frame_width();
        let stp = self.stack_pointer.clone();
        self.sym_table.pop_frame();
        self.emit(OpCode::Sub, stp, stp, Address::new_constant(w as i32));
    }
    
    pub fn emit_label(&self) -> Label {
        Label {
            place : self.instr_num + 1,
        }
    }
    
    // Return the instruction id because we might need to backpatch it later.
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
    
    pub fn new_pointer(&mut self) -> Address {
        self.temp_num += 1;
        Address {
            mode  : AddressMode::FramePointer,
            place : self.temp_num,
        }
    }
    
    pub fn generate_code(&mut self) {
        self.parser.parse();
        let root = self.parser.get_root();
        root.generate_code(self);        
    }
}