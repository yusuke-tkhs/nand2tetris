use crate::file_context::FileContext;
use schema::vm;

// pub struct Function {
//     name: vm::Label,
//     args_count: u16,
//     commands: Vec<Command>,
//     has_return: bool,
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccessCommand),
    // Call{name: vm::Label, args_count: u16},
    Label(vm::Label),
    Goto(vm::Label),
    IfGoto(vm::Label),
}

impl Command {
    pub fn try_from_command(
        src: vm::Command,
        file_context: &mut FileContext,
    ) -> anyhow::Result<Self> {
        Ok(match src {
            vm::Command::Arithmetic(arithmetic_command) => Self::Arithmetic(
                ArithmeticCommand::from_arithmetic_command(arithmetic_command, file_context),
            ),
            vm::Command::MemoryAccess(memory_access_command) => Self::MemoryAccess({
                match memory_access_command.access_type {
                    vm::AccessType::Push => {
                        MemoryAccessCommand::Push(PushSource::from_memory_access_command(
                            memory_access_command,
                            file_context.file_name(),
                        ))
                    }
                    vm::AccessType::Pop => {
                        MemoryAccessCommand::Pop(PopTarget::try_from_memory_access_command(
                            memory_access_command,
                            file_context.file_name(),
                        )?)
                    }
                }
            }),
            vm::Command::Label(label) => Command::Label(label),
            vm::Command::Goto(label) => Command::Goto(label),
            vm::Command::IfGoto(label) => Command::IfGoto(label),
            _ => unimplemented!(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticCommand {
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

impl ArithmeticCommand {
    fn from_arithmetic_command(
        command: vm::ArithmeticCommand,
        file_context: &mut FileContext,
    ) -> Self {
        match command {
            vm::ArithmeticCommand::Add => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Mathmatical(BinaryMathmaticalOperator::Addition),
            ),
            vm::ArithmeticCommand::Sub => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Mathmatical(BinaryMathmaticalOperator::Sububraction),
            ),
            vm::ArithmeticCommand::Neg => ArithmeticCommand::UnaryOperator(UnaryOperator::Negative),
            vm::ArithmeticCommand::Eq => {
                ArithmeticCommand::BinaryOperator(BinaryOperator::Comparison(
                    BinaryComparisonOperator::Equal,
                    file_context.publish_unique_key(),
                ))
            }
            vm::ArithmeticCommand::Gt => {
                ArithmeticCommand::BinaryOperator(BinaryOperator::Comparison(
                    BinaryComparisonOperator::GreaterThan,
                    file_context.publish_unique_key(),
                ))
            }
            vm::ArithmeticCommand::Lt => {
                ArithmeticCommand::BinaryOperator(BinaryOperator::Comparison(
                    BinaryComparisonOperator::LessThan,
                    file_context.publish_unique_key(),
                ))
            }
            vm::ArithmeticCommand::And => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Logical(BinaryLogicalOperator::And),
            ),
            vm::ArithmeticCommand::Or => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Logical(BinaryLogicalOperator::Or),
            ),
            vm::ArithmeticCommand::Not => ArithmeticCommand::UnaryOperator(UnaryOperator::Not),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]

pub enum UnaryOperator {
    Negative, // -y
    Not,      // !y
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    Mathmatical(BinaryMathmaticalOperator),       // 算術演算子
    Comparison(BinaryComparisonOperator, String), // 比較演算子, ユニークキー
    Logical(BinaryLogicalOperator),               // 論理演算子
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
    Constant(u16),         // 定数（仮想セグメント）
    SymbolMapping(String), // シンボルアクセス
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

impl PushSource {
    fn from_memory_access_command(src: vm::MemoryAccessCommand, file_name: String) -> Self {
        match src.segment {
            vm::Segment::Argument => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Argument,
                offset: src.index.get(),
            },
            vm::Segment::Local => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Local,
                offset: src.index.get(),
            },
            vm::Segment::Static => {
                Self::SymbolMapping(format!("Static_{file_name}_{}", src.index.get()))
            }
            vm::Segment::Constant => Self::Constant(src.index.get()),
            vm::Segment::This => Self::IndirectAddress {
                mapping_type: InDirectMappingType::This,
                offset: src.index.get(),
            },
            vm::Segment::That => Self::IndirectAddress {
                mapping_type: InDirectMappingType::That,
                offset: src.index.get(),
            },
            vm::Segment::Pointer => Self::DirectAddress {
                mapping_type: DirectMappingType::Pointer,
                offset: src.index.get(),
            },
            vm::Segment::Temp => Self::DirectAddress {
                mapping_type: DirectMappingType::Temp,
                offset: src.index.get(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PopTarget {
    SymbolMapping(String), // シンボルアクセス
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

impl PopTarget {
    fn try_from_memory_access_command(
        src: vm::MemoryAccessCommand,
        file_name: String,
    ) -> anyhow::Result<Self> {
        Ok(match src.segment {
            vm::Segment::Argument => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Argument,
                offset: src.index.get(),
            },
            vm::Segment::Local => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Local,
                offset: src.index.get(),
            },
            vm::Segment::Static => {
                Self::SymbolMapping(format!("Static_{file_name}_{}", src.index.get()))
            }
            vm::Segment::Constant => anyhow::bail!("catnnot pop to constant"),
            vm::Segment::This => Self::IndirectAddress {
                mapping_type: InDirectMappingType::This,
                offset: src.index.get(),
            },
            vm::Segment::That => Self::IndirectAddress {
                mapping_type: InDirectMappingType::That,
                offset: src.index.get(),
            },
            vm::Segment::Pointer => Self::DirectAddress {
                mapping_type: DirectMappingType::Pointer,
                offset: src.index.get(),
            },
            vm::Segment::Temp => Self::DirectAddress {
                mapping_type: DirectMappingType::Temp,
                offset: src.index.get(),
            },
        })
    }
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
