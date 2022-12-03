use crate::file_context::FileContext;
use schema::vm;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    MemoryAccess(MemoryAccess),
}

impl Command {
    // pub fn try_from_commands(src: vm::Command, file_name: String) -> anyhow::Result<Self> {

    // }
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
                        MemoryAccess::Push(PushSource::from_memory_access_command(
                            memory_access_command,
                            file_context.file_name(),
                        ))
                    }
                    vm::AccessType::Pop => {
                        MemoryAccess::Pop(MemorySegment::try_from_memory_access_command(
                            memory_access_command,
                            file_context.file_name(),
                        )?)
                    }
                }
            }),
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
pub enum MemoryAccess {
    Push(PushSource),
    Pop(MemorySegment),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PushSource {
    MemorySegment(MemorySegment),
    Constant(u16),
}

impl PushSource {
    fn from_memory_access_command(src: vm::MemoryAccessCommand, file_name: String) -> Self {
        match src.segment {
            vm::Segment::Constant => PushSource::Constant(src.index.get()),
            _ => {
                let Ok(memory_segment) = MemorySegment::try_from_memory_access_command(src, file_name) else {
                    unreachable!()
                };
                PushSource::MemorySegment(memory_segment)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemorySegment {
    ByCustomSymbol(String),
    ByBaseAddressAndOffset {
        base_kind: MemorySegmentBaseKind,
        offset: u16,
    },
}

impl MemorySegment {
    fn try_from_memory_access_command(
        src: vm::MemoryAccessCommand,
        file_name: String,
    ) -> anyhow::Result<Self> {
        Ok(match src.segment {
            vm::Segment::Constant => anyhow::bail!("Constant is not memory segment"),
            vm::Segment::Static => {
                MemorySegment::ByCustomSymbol(format!("{}.{}", file_name, src.index.get()))
            }
            _ => {
                let Ok(base_kind) = MemorySegmentBaseKind::try_from(src.segment) else {
                    unreachable!()
                };
                MemorySegment::ByBaseAddressAndOffset {
                    base_kind,
                    offset: src.index.get(),
                }
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemorySegmentBaseKind {
    Argument,
    Local,
    This,
    That,
    Pointer,
    Temp,
}

impl TryFrom<vm::Segment> for MemorySegmentBaseKind {
    type Error = anyhow::Error;

    fn try_from(value: vm::Segment) -> anyhow::Result<Self> {
        Ok(match value {
            vm::Segment::Constant => anyhow::bail!("Constant has no memory segment base"),
            vm::Segment::Argument => MemorySegmentBaseKind::Argument,
            vm::Segment::Local => MemorySegmentBaseKind::Local,
            vm::Segment::Static => anyhow::bail!("Static has no memory segment base"),
            vm::Segment::This => MemorySegmentBaseKind::This,
            vm::Segment::That => MemorySegmentBaseKind::That,
            vm::Segment::Pointer => MemorySegmentBaseKind::Pointer,
            vm::Segment::Temp => MemorySegmentBaseKind::Temp,
        })
    }
}
