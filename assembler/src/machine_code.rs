use crate::symbol_table::SymbolTable;
use schema::hack;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Instruction {
    A([bool; 15]),
    C {
        a: bool,
        comp: [bool; 6],
        dest: [bool; 3],
        jump: [bool; 3],
    },
}

#[allow(dead_code)]
pub fn construct(_symbol_table: &SymbolTable, _commands: &[hack::Command]) -> Vec<Instruction> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn generate(_instructions: Vec<Instruction>) -> String {
    unimplemented!()
}
