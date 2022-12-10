mod parser;

pub use parser::parse;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccessCommand),
    Function { name: Label, args_count: u16 },
    Call { name: Label, args_count: u16 },
    Return,
    Label(Label),
    Goto(Label),
    IfGoto(Label),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label(String);

const AvailableCharsInLabel: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_.:";

impl std::str::FromStr for Label {
    type Err = anyhow::Error;
    fn from_str(label: &str) -> Result<Self, Self::Err> {
        use crate::constant::DIGIT_CHAR;
        if DIGIT_CHAR.chars().any(|c| label.starts_with(c)) {
            anyhow::bail!("label should not starts with digit")
        }
        if let Some(invalid_char) = label.chars().find(|label_c| {
            DIGIT_CHAR.chars().all(|c| c != *label_c)
                && AvailableCharsInLabel.chars().all(|c| c != *label_c)
        }) {
            anyhow::bail!("invalid char is used for label: {}", invalid_char)
        } else {
            Ok(Self(label.to_string()))
        }
    }
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
