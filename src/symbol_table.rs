use std::collections::HashMap;
use ast::statement::Type;
use code_generator::Address;

pub struct IdeInfo {
    pub typeinfo : Type,   
    pub address  : Address,
}

pub struct SymbolTable {
    frame_stack : Vec<HashMap<String, IdeInfo>>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            frame_stack : Vec::new(),
        }    
    } 
    pub fn push_frame(&mut self) {
        self.frame_stack.push(HashMap::new());
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
        self.frame_stack[n-1].insert(name, info);
    }
    
    pub fn get_ide(&self, name : &str) -> Option<&IdeInfo> {
        self.frame_stack[self.frame_stack.len()-1]
            .get(name)
    }
}
