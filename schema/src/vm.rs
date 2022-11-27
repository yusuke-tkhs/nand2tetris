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

// StackにPushする元のメモリセグメント
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PushSourceSegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

// StackからPopする先のメモリセグメント
// Constant は物理的な領域を持たない疑似セグメントなので、
// ここにPopすることはできないはず。なのでPopセグメントにはConstantは無い。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PopTargetSegment {
    Argument,
    Local,
    Static,
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryAccessCommand {
    Push(PushSourceSegment, Index),
    Pop(PopTargetSegment, Index),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccessCommand),
}
