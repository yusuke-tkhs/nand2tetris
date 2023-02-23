mod parser;

use crate::parser::parsable_enum;
pub use parser::parse;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccessCommand),
    Function {
        name: Label,
        local_variable_count: u16,
    },
    Call {
        name: Label,
        args_count: u16,
    },
    Return,
    Label(Label),
    Goto(Label),
    IfGoto(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(String);

impl Label {
    pub fn new(str: &str) -> Self {
        Self(str.to_string())
    }
    pub fn get(&self) -> &str {
        &self.0
    }
    pub fn get_string(&self) -> String {
        self.0.to_string()
    }
}

parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ArithmeticCommand {
        Add: "add",
        Sub: "sub",
        Neg: "neg",
        Eq: "eq",
        Gt: "gt",
        Lt: "lt",
        And: "and",
        Or: "or",
        Not: "not",
    }
}

parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum AccessType {
        Push: "push",
        Pop: "pop",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryAccessCommand {
    pub access_type: AccessType,
    pub segment: Segment,
    pub index: Index,
}

parsable_enum! {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Segment {
        Argument: "argument",
        Local: "local",
        Static: "static",
        Constant: "constant",
        This: "this",
        That: "that",
        Pointer: "pointer",
        Temp: "temp",
    }
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
