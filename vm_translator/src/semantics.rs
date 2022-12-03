use schema::vm;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Arithmetic(vm::ArithmeticCommand),
    MemoryAccess(MemoryAccess),
}

impl Command {
    pub fn try_from_command(src: vm::Command, file_name: String) -> anyhow::Result<Self> {
        Ok(match src {
            vm::Command::Arithmetic(arithmetic_command) => Self::Arithmetic(arithmetic_command),
            vm::Command::MemoryAccess(memory_access_command) => Self::MemoryAccess({
                match memory_access_command.access_type {
                    vm::AccessType::Push => MemoryAccess::Push(
                        PushSource::from_memory_access_command(memory_access_command, file_name),
                    ),
                    vm::AccessType::Pop => {
                        MemoryAccess::Pop(MemorySegment::try_from_memory_access_command(
                            memory_access_command,
                            file_name,
                        )?)
                    }
                }
            }),
        })
    }
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
