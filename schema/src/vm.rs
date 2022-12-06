mod parser;

pub use parser::parse;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccessCommand),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticCommand {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessType {
    Push,
    Pop,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryAccessCommand {
    pub access_type: AccessType,
    pub segment: Segment,
    pub index: Index,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(u16);
impl Index {
    pub fn new(v: u16) -> Self {
        Self(v)
    }
    pub fn get(&self) -> u16 {
        self.0
    }
}
