#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestMnemonic {
    Null,
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompMnemonic {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NegateD,
    NegateA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NegateM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JumpMnemonic {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct CCommand {
    pub dest: Option<DestMnemonic>,
    pub comp: CompMnemonic,
    pub jump: Option<JumpMnemonic>,
}

// 定義済みシンボル、ラベル、変数のいずれかを意味する。
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Symbol(pub String);

impl Symbol {
    pub fn new(str: &str) -> Self {
        Self(str.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ACommand {
    Address(u16),
    Symbol(Symbol), // @value で数字以外のもの。
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    A(ACommand),
    C(CCommand),
    L(Symbol),
}
