use crate::symbol_table::SymbolTable;
use schema::hack::*;

#[derive(Debug, Clone)]
pub enum Instruction {
    A(u16),
    C {
        a: bool,
        comp: [bool; 6],
        dest: [bool; 3],
        jump: [bool; 3],
    },
}

fn dest_bit(mnemonic: DestMnemonic) -> [bool; 3] {
    match mnemonic {
        DestMnemonic::Null => [false, false, false],
        DestMnemonic::M => [false, false, true],
        DestMnemonic::D => [false, true, false],
        DestMnemonic::MD => [false, true, true],
        DestMnemonic::A => [true, false, false],
        DestMnemonic::AM => [true, false, true],
        DestMnemonic::AD => [true, true, false],
        DestMnemonic::AMD => [true, true, true],
    }
}

fn a_comp_bit(mnemonic: CompMnemonic) -> (bool, [bool; 6]) {
    match mnemonic {
        CompMnemonic::Zero => (false, [true, false, true, false, true, false]),
        CompMnemonic::One => (false, [true, true, true, true, true, true]),
        CompMnemonic::MinusOne => (false, [true, true, true, false, true, false]),
        CompMnemonic::D => (false, [false, false, true, true, false, false]),
        CompMnemonic::A => (false, [true, true, false, false, false, false]),
        CompMnemonic::NegateD => (false, [false, false, true, true, false, true]),
        CompMnemonic::NegateA => (false, [true, true, false, false, false, true]),
        CompMnemonic::MinusD => (false, [false, false, true, true, true, true]),
        CompMnemonic::MinusA => (false, [true, true, false, false, true, true]),
        CompMnemonic::DPlusOne => (false, [false, true, true, true, true, true]),
        CompMnemonic::APlusOne => (false, [true, true, false, true, true, true]),
        CompMnemonic::DMinusOne => (false, [false, false, true, true, true, false]),
        CompMnemonic::AMinusOne => (false, [true, true, false, false, true, false]),
        CompMnemonic::DPlusA => (false, [false, false, false, false, true, false]),
        CompMnemonic::DMinusA => (false, [false, true, false, false, true, true]),
        CompMnemonic::AMinusD => (false, [false, false, false, true, true, true]),
        CompMnemonic::DAndA => (false, [false, false, false, false, false, false]),
        CompMnemonic::DOrA => (false, [false, true, false, true, false, true]),
        CompMnemonic::M => (true, [true, true, false, false, false, false]),
        CompMnemonic::NegateM => (true, [true, true, false, false, false, true]),
        CompMnemonic::MinusM => (true, [true, true, false, false, true, true]),
        CompMnemonic::MPlusOne => (true, [true, true, false, true, true, true]),
        CompMnemonic::MMinusOne => (true, [true, true, false, false, true, false]),
        CompMnemonic::DPlusM => (true, [false, false, false, false, true, false]),
        CompMnemonic::DMinusM => (true, [false, true, false, false, true, true]),
        CompMnemonic::MMinusD => (true, [false, false, false, true, true, true]),
        CompMnemonic::DAndM => (true, [false, false, false, false, false, false]),
        CompMnemonic::DOrM => (true, [false, true, false, true, false, true]),
    }
}

fn jump_bit(mnemonic: JumpMnemonic) -> [bool; 3] {
    match mnemonic {
        JumpMnemonic::Null => [false, false, false],
        JumpMnemonic::JGT => [false, false, true],
        JumpMnemonic::JEQ => [false, true, false],
        JumpMnemonic::JGE => [false, true, true],
        JumpMnemonic::JLT => [true, false, false],
        JumpMnemonic::JNE => [true, false, true],
        JumpMnemonic::JLE => [true, true, false],
        JumpMnemonic::JMP => [true, true, true],
    }
}

fn bit_to_char(bit: bool) -> char {
    if bit {
        '1'
    } else {
        '0'
    }
}

fn bits_to_string(bits: impl Iterator<Item = bool>) -> String {
    String::from_iter(bits.map(bit_to_char))
}

impl Instruction {
    fn from_a_command(a_command: ACommand, symbol_table: &SymbolTable) -> anyhow::Result<Self> {
        let address = match a_command {
            ACommand::Address(address) => address,
            ACommand::Symbol(symbol) => symbol_table
                .get(&symbol)
                .ok_or_else(|| anyhow::anyhow!("{:?} was not found in SymbolTable.", symbol))?,
        };
        Ok(Self::A(address))
    }
    fn from_c_command(c_command: CCommand) -> Self {
        let dest_bit = c_command
            .dest
            .map(dest_bit)
            .unwrap_or([false, false, false]);

        let (a_bit, comp_bit) = a_comp_bit(c_command.comp);
        let jump_bit = c_command
            .jump
            .map(jump_bit)
            .unwrap_or([false, false, false]);
        Self::C {
            a: a_bit,
            comp: comp_bit,
            dest: dest_bit,
            jump: jump_bit,
        }
    }
    pub fn into_bit_string(self) -> String {
        match self {
            Self::A(address) => format!("0{:015b}", address),
            Self::C {
                a,
                comp,
                dest,
                jump,
            } => {
                format!(
                    "111{}{}{}{}",
                    bit_to_char(a),
                    bits_to_string(comp.into_iter()),
                    bits_to_string(dest.into_iter()),
                    bits_to_string(jump.into_iter())
                )
            }
        }
    }
}

pub fn construct(
    symbol_table: &SymbolTable,
    commands: Vec<Command>,
) -> anyhow::Result<Vec<Instruction>> {
    commands
        .into_iter()
        .filter_map(|command| match command {
            Command::A(a_command) => Some(Instruction::from_a_command(a_command, symbol_table)),
            Command::C(c_command) => Some(Ok(Instruction::from_c_command(c_command))),
            Command::L(_) => None,
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

pub fn generate(instructions: Vec<Instruction>) -> String {
    instructions
        .into_iter()
        .map(Instruction::into_bit_string)
        .collect::<Vec<_>>()
        .join("\n")
}
