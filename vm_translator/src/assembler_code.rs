use crate::file_context::FileContext;
use crate::semantics;
use schema::hack;

mod arithmetic;
mod memory_access;
mod program_flow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblerCodeBlock {
    pub comment: Option<AssemblerCodeComment>,
    pub commands: Vec<hack::Command>,
}

impl AssemblerCodeBlock {
    pub fn new(comment: &str, commands: &[hack::Command]) -> Self {
        Self {
            comment: Some(AssemblerCodeComment::new(comment)),
            commands: commands.to_vec(),
        }
    }
    pub fn new_header_comment(comment: &str) -> Self {
        Self {
            comment: Some(AssemblerCodeComment::new(format!("[{comment}]").as_str())),
            commands: Default::default(),
        }
    }
}

pub fn construct_code_block(
    commands: Vec<semantics::Command>,
    file_context: &mut FileContext,
) -> anyhow::Result<Vec<AssemblerCodeBlock>> {
    Ok(commands
        .into_iter()
        .flat_map(|command: semantics::Command| match command {
            semantics::Command::Arithmetic(arithmetic_command) => {
                arithmetic::construct(arithmetic_command, file_context)
            }
            semantics::Command::MemoryAccess(memory_access) => {
                memory_access::construct(memory_access, file_context)
            }
            semantics::Command::Label(label) => vec![program_flow::construct_label(label)],
            semantics::Command::Goto(label) => vec![program_flow::construct_goto(label)],
            semantics::Command::IfGoto(label) => program_flow::construct_if_goto(label),
            semantics::Command::Call { .. } => todo!(),
            semantics::Command::Return => todo!(),
        })
        .collect::<Vec<_>>())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblerCodeComment(String);

impl AssemblerCodeComment {
    pub fn new(content: &str) -> Self {
        Self(content.to_string())
    }
    pub fn to_str(&self) -> String {
        format!("// {}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AssemblerCodeLine {
    Command(hack::Command, Option<AssemblerCodeComment>), // @SP // ROM 0 みたいな行
    Comment(AssemblerCodeComment),                        // [Push static 3] みたいな行
}

impl AssemblerCodeLine {
    pub fn into_code_str(self) -> String {
        match self {
            Self::Command(command, comment) => {
                command_to_code(command)
                    + &comment
                        .map(|comment| format!(" {}", comment.to_str()))
                        .unwrap_or_default()
            }
            Self::Comment(comment) => comment.to_str(),
        }
    }
}

// 命令番号コメント付きの行に変換する
fn construct_code_lines(blocks: Vec<AssemblerCodeBlock>) -> Vec<AssemblerCodeLine> {
    let mut command_counter: u64 = 0;
    let mut lines: Vec<AssemblerCodeLine> = Vec::new();
    for block in blocks {
        if let Some(comment) = block.comment {
            lines.push(AssemblerCodeLine::Comment(comment))
        };
        for command in block.commands {
            if matches!(&command, hack::Command::A(_) | hack::Command::C(_)) {
                lines.push(AssemblerCodeLine::Command(
                    command,
                    Some(AssemblerCodeComment::new(
                        format!("ROM {command_counter}").as_str(),
                    )),
                ));
                command_counter += 1;
            } else {
                lines.push(AssemblerCodeLine::Command(command, None));
            }
        }
    }
    lines
}

pub fn genarate_code_str(blocks: Vec<AssemblerCodeBlock>) -> String {
    construct_code_lines(blocks)
        .into_iter()
        .map(AssemblerCodeLine::into_code_str)
        .collect::<Vec<_>>()
        .join("\n")
}

fn command_to_code(command: hack::Command) -> String {
    match command {
        hack::Command::A(a_command) => match a_command {
            hack::ACommand::Address(value) => format!("@{}", value),
            hack::ACommand::Symbol(symbol) => format!("@{}", symbol.get()),
        },
        hack::Command::C(c_command) => [
            c_command
                .dest
                .map(|d| format!("{}=", dest(d)))
                .unwrap_or_default(),
            comp(c_command.comp).to_string(),
            c_command
                .jump
                .map(|j| format!(";{}", jump(j)))
                .unwrap_or_default(),
        ]
        .concat(),
        hack::Command::L(symbol) => format!("({})", symbol.get()),
    }
}

fn dest(mnemonic: hack::DestMnemonic) -> &'static str {
    match mnemonic {
        hack::DestMnemonic::Null => "null",
        hack::DestMnemonic::M => "M",
        hack::DestMnemonic::D => "D",
        hack::DestMnemonic::MD => "MD",
        hack::DestMnemonic::A => "A",
        hack::DestMnemonic::AM => "AM",
        hack::DestMnemonic::AD => "AD",
        hack::DestMnemonic::AMD => "AMD",
    }
}

fn comp(mnemonic: hack::CompMnemonic) -> &'static str {
    match mnemonic {
        hack::CompMnemonic::Zero => "0",
        hack::CompMnemonic::One => "1",
        hack::CompMnemonic::MinusOne => "-1",
        hack::CompMnemonic::D => "D",
        hack::CompMnemonic::A => "A",
        hack::CompMnemonic::NegateD => "!D",
        hack::CompMnemonic::NegateA => "!A",
        hack::CompMnemonic::MinusD => "-D",
        hack::CompMnemonic::MinusA => "-A",
        hack::CompMnemonic::DPlusOne => "D+1",
        hack::CompMnemonic::APlusOne => "A+1",
        hack::CompMnemonic::DMinusOne => "D-1",
        hack::CompMnemonic::AMinusOne => "A-1",
        hack::CompMnemonic::DPlusA => "D+A",
        hack::CompMnemonic::DMinusA => "D-A",
        hack::CompMnemonic::AMinusD => "A-D",
        hack::CompMnemonic::DAndA => "D&A",
        hack::CompMnemonic::DOrA => "D|A",
        hack::CompMnemonic::M => "M",
        hack::CompMnemonic::NegateM => "!M",
        hack::CompMnemonic::MinusM => "-M",
        hack::CompMnemonic::MPlusOne => "M+1",
        hack::CompMnemonic::MMinusOne => "M-1",
        hack::CompMnemonic::DPlusM => "D+M",
        hack::CompMnemonic::DMinusM => "D-M",
        hack::CompMnemonic::MMinusD => "M-D",
        hack::CompMnemonic::DAndM => "D&M",
        hack::CompMnemonic::DOrM => "D|M",
    }
}

fn jump(mnemonic: hack::JumpMnemonic) -> &'static str {
    match mnemonic {
        hack::JumpMnemonic::Null => "null",
        hack::JumpMnemonic::JGT => "JGT",
        hack::JumpMnemonic::JEQ => "JEQ",
        hack::JumpMnemonic::JGE => "JGE",
        hack::JumpMnemonic::JLT => "JLT",
        hack::JumpMnemonic::JNE => "JNE",
        hack::JumpMnemonic::JLE => "JLE",
        hack::JumpMnemonic::JMP => "JMP",
    }
}
