use crate::semantics;
use schema::hack;

macro_rules! concat_commands {
    ($head: expr$(,$tail:expr)*$(,)?) => {
        $head
        .into_iter()
        $(.chain($tail.into_iter()))*
        .collect::<Vec<_>>()
    };
}
#[macro_use]
mod memory_access;

#[macro_use]
mod arithmetic;

pub fn construct(commands: Vec<semantics::Command>) -> anyhow::Result<Vec<hack::Command>> {
    Ok(commands
        .into_iter()
        .flat_map(|command: semantics::Command| -> Vec<hack::Command> {
            match command {
                semantics::Command::Arithmetic(arithmetic_command) => {
                    arithmetic::construct(arithmetic_command)
                }
                semantics::Command::MemoryAccess(memory_access) => {
                    memory_access::construct(memory_access)
                }
            }
        })
        .collect::<Vec<_>>())
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

pub fn generate(commands: Vec<hack::Command>) -> String {
    commands
        .into_iter()
        .map(|command| match command {
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
        })
        .collect::<Vec<_>>()
        .join("\n")
}
