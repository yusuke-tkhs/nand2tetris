use super::assembler_code::AssemblerCodeBlock;
use super::memory_access::{load_value_to_d_by_symbol_address, pop_to_address_written_in_d};
use schema::hack;

pub(super) fn construct() -> Vec<AssemblerCodeBlock> {
    std::iter::once(AssemblerCodeBlock::new(
        "return from current function",
        &[
            // LCLをR14に退避
            // @LCL
            // D=M
            // @R14
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("LCL"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R14"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
            // リターンアドレス (LCL - 5) をR15に退避
            // @5
            // A=D-A // リターンアドレスが格納されているメモリアドレスを求めてAレジスタにセットする
            // D=M // リターンアドレスの値をDレジスタに保存
            // @R15
            // M=D
            hack::Command::A(hack::ACommand::Address(5)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::DMinusA,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R15"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
        ],
    ))
    .into_iter()
    .chain(
        [
            // ARG のアドレスにこの関数の戻り値（Stackの末尾にある）をいれる
            load_value_to_d_by_symbol_address("ARG".to_string()),
            pop_to_address_written_in_d(),
            set_sp_arg_plus_one(),
            restore_caller_value("THAT", "R14", -1),
            restore_caller_value("THIS", "R14", -2),
            restore_caller_value("ARG", "R14", -3),
            restore_caller_value("LCL", "R14", -4),
            AssemblerCodeBlock::new(
                "jump to caller",
                &[
                    // @R15
                    // A=M
                    // 0;JMP
                    hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R15"))),
                    hack::Command::C(hack::CCommand {
                        dest: Some(hack::DestMnemonic::A),
                        comp: hack::CompMnemonic::M,
                        jump: None,
                    }),
                    hack::Command::C(hack::CCommand {
                        dest: None,
                        comp: hack::CompMnemonic::Zero,
                        jump: Some(hack::JumpMnemonic::JMP),
                    }),
                ],
            ),
        ]
        .into_iter(),
    )
    .collect()
}

fn restore_caller_value(
    restore_target_symbol: &str,
    base_symbol: &str,
    offset: i32,
) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        &format!("restore caller value: {restore_target_symbol}"),
        &[
            // @base_symbol
            // D=M
            // @offset
            // A=D-A or D+A
            // D=M
            // @restore_target_symbol
            // A=M
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(base_symbol))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Address(offset.unsigned_abs() as u16)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: if offset > 0 {
                    hack::CompMnemonic::DPlusA
                } else {
                    hack::CompMnemonic::DMinusA
                },
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(
                restore_target_symbol,
            ))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
        ],
    )
}

fn set_sp_arg_plus_one() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "set SP = ARG + 1",
        &[
            // @ARG
            // D=M+1
            // @restore_target_symbol
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("ARG"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::MPlusOne,
                jump: None,
            }),
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
        ],
    )
}
