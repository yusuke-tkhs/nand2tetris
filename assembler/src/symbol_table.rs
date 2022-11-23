use std::collections::HashMap;

use schema::hack;

// ROM or RAM アドレス。
#[derive(Debug, Clone)]

pub struct Address(pub u16);

#[derive(Debug, Clone)]
pub struct SymbolTable(HashMap<hack::Symbol, Address>);

pub(crate) fn create(_commands: &[hack::Command]) -> SymbolTable {
    unimplemented!()
}
