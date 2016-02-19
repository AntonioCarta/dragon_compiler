use code_generator::{OpCode, AddressMode, AddressCode, Address, IntermediateRepresentation};

pub struct Interpreter {
    code      : Vec<AddressCode>,
    stack     : Vec<i32>,
    regs      : Vec<i32>,
}

impl Interpreter {
    pub fn new(code : IntermediateRepresentation) -> Self{
        Interpreter {
            code  : code.instructions,
            stack : Vec::new(),
            regs  : Vec::new(),    
        }
    }
    
    pub fn execute(&mut self) {
        let mut i = 0;
        while i < self.code.len() {
            let instr = &self.code[i];
            let res = instr.res.place as usize;
            let x = self.regs[instr.x.place as usize];
            let y = self.regs[instr.y.place as usize];
            // Update instruction counter.
            i = i + 1;
            match instr.op {
                OpCode::Mov     => self.regs[res] = x,
                // Boolean operators.
                OpCode::And     => self.regs[res] = Interpreter::and(x, y),
                OpCode::Or      => self.regs[res] = Interpreter::or(x, y),
                OpCode::Not     => self.regs[res] = Interpreter::not(x),
                OpCode::IsPos   => self.regs[res] = {if x > 0 {1} else {0}},
                OpCode::IsNeg   => self.regs[res] = {if x < 0 {1} else {0}},
                // Num operators.
                OpCode::Add     => self.regs[res] = x + y,
                OpCode::Sub     => self.regs[res] = x - y,
                OpCode::Mul     => self.regs[res] = x * y,
                OpCode::Div     => self.regs[res] = x / y,
                OpCode::Minus   => self.regs[res] = -x,
                // Jump.
                OpCode::Goto    => i = instr.res.place as usize,
                OpCode::JmpZ    => if x == 0 { i = instr.res.place as usize },
            }
        }
    }
    
    fn and(x : i32, y : i32) -> i32 {
        if x==1 && y==1 {
            1
        } else { 0 }
    }
    
    fn or(x : i32, y : i32) -> i32 {
        if x==1 || y==1 {
            1
        } else { 0 }
    }
    
    fn not(x : i32) -> i32 {
        if x==1 {
            0
        } else { 1 }
    }
}