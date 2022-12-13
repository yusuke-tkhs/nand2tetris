mod from_schema;
mod to_assembler;

pub(crate) use to_assembler::assembler_code::genarate_assembler_code;
pub(crate) use to_assembler::assembler_code::AssemblerCodeBlock;

// ファイルはモジュールと仮定する
#[allow(dead_code)]
pub struct Module {
    name: String,
    functions: Vec<Function>,
}

#[allow(dead_code)]
pub struct Function {
    name: String,
    local_variable_count: u16,
    commands: Vec<Command>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccessCommand),
    Call { name: String, args_count: u16 },
    Return,
    Label(String),
    Goto(String),
    IfGoto(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticCommand {
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

pub enum UnaryOperator {
    Negative, // -y
    Not,      // !y
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Mathmatical(BinaryMathmaticalOperator), // 算術演算子
    Comparison(BinaryComparisonOperator),   // 比較演算子
    Logical(BinaryLogicalOperator),         // 論理演算子
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryMathmaticalOperator {
    Addition,     // x + y
    Sububraction, // x - y
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryComparisonOperator {
    Equal,       // x == y
    GreaterThan, // x > y
    LessThan,    // x < y
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryLogicalOperator {
    And, // x && y
    Or,  // x || y
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryAccessCommand {
    Push(PushSource),
    Pop(PopTarget),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PushSource {
    Constant(u16),       // 定数（仮想セグメント）
    StaticVariable(u16), // スタティック変数
    DirectAddress {
        // 直接アドレス指定
        mapping_type: DirectMappingType,
        offset: u16,
    },
    IndirectAddress {
        // 間接アドレス指定
        mapping_type: InDirectMappingType,
        offset: u16,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PopTarget {
    StaticVariable(u16), // スタティック変数
    DirectAddress {
        // 直接アドレス指定
        mapping_type: DirectMappingType,
        offset: u16,
    },
    IndirectAddress {
        // 間接アドレス指定
        mapping_type: InDirectMappingType,
        offset: u16,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectMappingType {
    Pointer,
    Temp,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InDirectMappingType {
    Argument,
    Local,
    This,
    That,
}
