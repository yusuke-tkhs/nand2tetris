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
pub enum MemoryAccessCommand {
    Push(PushSourceSegment, Index),
    Pop(MemorySegment, Index),
}

// StackにPushする元のメモリセグメント
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PushSourceSegment {
    Memory(MemorySegment),
    Constant,
}

// Constant は物理的な領域を持たない疑似セグメントなので、
// アルゴリズムの都合でこことは分けたモデルにする
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemorySegment {
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
    pub fn get(&self) -> u16 {
        self.0
    }
}
