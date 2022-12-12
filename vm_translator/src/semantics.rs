use schema::vm;

// ファイルはモジュールと仮定する
#[allow(dead_code)]
pub struct Module {
    name: String,
    functions: Vec<Function>,
}

#[allow(dead_code)]
impl Module {
    fn try_from_commands(
        module_name: String,
        vm_commands: Vec<vm::Command>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            name: module_name,
            functions: Function::try_from_commands(vm_commands)?,
        })
    }
}

#[allow(dead_code)]
pub struct Function {
    name: String,
    args_count: u16,
    commands: Vec<Command>,
}

#[allow(dead_code)]
impl Function {
    fn try_from_commands(vm_commands: Vec<vm::Command>) -> anyhow::Result<Vec<Self>> {
        let each_function_vm_commands = separate(vm_commands, |vm_command| {
            matches!(vm_command, vm::Command::Function { .. })
        });
        each_function_vm_commands
            .into_iter()
            .map(|commands| {
                let Some((
                    vm::Command::Function { name, args_count },
                    rest_commands
                )) = commands.split_first() else {
                    anyhow::bail!("all commands should be written in function!");
                };
                Ok(Self {
                    name: name.get().to_string(),
                    args_count: *args_count,
                    commands: rest_commands
                        .iter()
                        .cloned()
                        .map(Command::try_from_command)
                        .collect::<anyhow::Result<Vec<_>>>()?,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()
    }
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

impl Command {
    pub fn try_from_command(src: vm::Command) -> anyhow::Result<Self> {
        Ok(match src {
            vm::Command::Function { .. } => {
                anyhow::bail!("function could not converted to semantic command")
            }
            vm::Command::Arithmetic(arithmetic_command) => Self::Arithmetic(
                ArithmeticCommand::from_arithmetic_command(arithmetic_command),
            ),
            vm::Command::MemoryAccess(memory_access_command) => Self::MemoryAccess({
                match memory_access_command.access_type {
                    vm::AccessType::Push => MemoryAccessCommand::Push(
                        PushSource::from_memory_access_command(memory_access_command),
                    ),
                    vm::AccessType::Pop => MemoryAccessCommand::Pop(
                        PopTarget::try_from_memory_access_command(memory_access_command)?,
                    ),
                }
            }),
            vm::Command::Call { name, args_count } => Command::Call {
                name: name.get_string(),
                args_count,
            },
            vm::Command::Return => Command::Return,
            vm::Command::Label(label) => Command::Label(label.get_string()),
            vm::Command::Goto(label) => Command::Goto(label.get_string()),
            vm::Command::IfGoto(label) => Command::IfGoto(label.get_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticCommand {
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator),
}

impl ArithmeticCommand {
    fn from_arithmetic_command(command: vm::ArithmeticCommand) -> Self {
        match command {
            vm::ArithmeticCommand::Add => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Mathmatical(BinaryMathmaticalOperator::Addition),
            ),
            vm::ArithmeticCommand::Sub => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Mathmatical(BinaryMathmaticalOperator::Sububraction),
            ),
            vm::ArithmeticCommand::Neg => ArithmeticCommand::UnaryOperator(UnaryOperator::Negative),
            vm::ArithmeticCommand::Eq => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Comparison(BinaryComparisonOperator::Equal),
            ),
            vm::ArithmeticCommand::Gt => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Comparison(BinaryComparisonOperator::GreaterThan),
            ),
            vm::ArithmeticCommand::Lt => ArithmeticCommand::BinaryOperator(
                BinaryOperator::Comparison(BinaryComparisonOperator::LessThan),
            ),
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

impl PushSource {
    fn from_memory_access_command(src: vm::MemoryAccessCommand) -> Self {
        match src.segment {
            vm::Segment::Argument => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Argument,
                offset: src.index.get(),
            },
            vm::Segment::Local => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Local,
                offset: src.index.get(),
            },
            vm::Segment::Static => Self::StaticVariable(src.index.get()),
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

impl PopTarget {
    fn try_from_memory_access_command(src: vm::MemoryAccessCommand) -> anyhow::Result<Self> {
        Ok(match src.segment {
            vm::Segment::Argument => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Argument,
                offset: src.index.get(),
            },
            vm::Segment::Local => Self::IndirectAddress {
                mapping_type: InDirectMappingType::Local,
                offset: src.index.get(),
            },
            vm::Segment::Static => Self::StaticVariable(src.index.get()),
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

// pred が true を返す度に新しいVecとなるように分割する
fn separate<T>(vec: Vec<T>, pred: fn(&T) -> bool) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = vec![];
    for v in vec {
        if pred(&v) {
            res.push(vec![v]);
        } else if let Some(last) = res.last_mut() {
            last.push(v);
        } else {
            res.push(vec![v]);
        }
    }
    res
}

#[test]
fn test_separete() {
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(
        separate(nums, |n| n % 3 == 0),
        vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7],]
    );
    let nums = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(
        separate(nums, |n| n % 3 == 0),
        vec![vec![1, 2], vec![3, 4, 5], vec![6, 7],]
    )
}
