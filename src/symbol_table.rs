use std::collections::HashMap;
use ast::statement::Type;
use code_generator::Address;
use std::iter::Iterator;

pub struct IdeInfo {
    pub typeinfo : Type,   
    pub address  : Address,
}

struct Frame {
    width : u32,
    table :HashMap<String, IdeInfo>,
}

pub struct SymbolTable {
    frame_stack : Vec<Frame>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            frame_stack : Vec::new(),
        }    
    } 
    pub fn push_frame(&mut self) {
        let f = Frame {
            width : 0,
            table : HashMap::new(),    
        };
        self.frame_stack.push(f);
    }
    
    pub fn pop_frame(&mut self) {
        self.frame_stack.pop();
    }
    
    pub fn put(&mut self, name : String, typeinfo : Type, address : Address) {
        let info = IdeInfo {
            typeinfo : typeinfo,
            address  : address,
        };
        let n = self.frame_stack.len();
        let array_width = info.typeinfo.dim_width.iter().fold(0, |sum, x| sum + x);
        let ide_width = info.typeinfo.element_width * array_width;
        self.frame_stack[n-1].table.insert(name, info);        
        self.frame_stack[n-1].width += ide_width;
    }
    // BUG: should search other frame if not found.
    pub fn get_ide(&self, name : &str) -> Option<&IdeInfo> {
        self.frame_stack[self.frame_stack.len()-1]
            .table.get(name)
    }
    
    pub fn get_frame_width(&self) -> u32 {
        self.frame_stack[self.frame_stack.len() - 1].width
    }
}
