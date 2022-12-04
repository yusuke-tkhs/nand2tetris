use super::AssemblerCodeBlock;
use crate::semantics;
use schema::hack;

pub fn construct(memory_access: semantics::MemoryAccessCommand) -> Vec<AssemblerCodeBlock> {
    match memory_access {
        semantics::MemoryAccessCommand::Push(push_source) => {
            // スタックにPushする値をセグメントから情報から求めてDレジスタに書き込むアセンブラ命令群
            match push_source {
                semantics::PushSource::Constant(v) => {
                    // 定数値をDレジスタに書き込みそれをStackにPushする
                    vec![
                        AssemblerCodeBlock::new_header_comment(&format!("Push constant {v}")),
                        load_constant_to_d(v),
                        write_d_to_stack(),
                    ]
                }
                semantics::PushSource::SymbolMapping(symbol_name) => {
                    vec![
                        AssemblerCodeBlock::new_header_comment(&format!(
                            "Push value in symbol '{symbol_name}'"
                        )),
                        load_value_to_d_by_symbol_address(symbol_name),
                        write_d_to_stack(),
                    ]
                }
                semantics::PushSource::DirectAddress {
                    mapping_type,
                    offset,
                } => {
                    vec![
                        AssemblerCodeBlock::new_header_comment(&format!(
                            "Push value in directly mapped memory segment '{mapping_type:?}' + offset {offset}"
                        )),
                        load_direct_address_to_d(mapping_type, offset),
                        load_value_specified_by_address_in_d(),
                        write_d_to_stack(),
                    ]
                }
                semantics::PushSource::IndirectAddress {
                    mapping_type,
                    offset,
                } => {
                    // 間接アドレス値をDレジスタにロードして、それをStackにPushする
                    vec![
                        AssemblerCodeBlock::new_header_comment(&format!(
                            "Push value in in-directly mapped memory segment '{mapping_type:?}' + offset({offset})"
                        )),
                        load_indirect_address_to_d(mapping_type, offset),
                        load_value_specified_by_address_in_d(),
                        write_d_to_stack(),
                    ]
                }
            }
        }
        semantics::MemoryAccessCommand::Pop(pop_target) => match pop_target {
            // stack からのPopを実現する命令群
            semantics::PopTarget::SymbolMapping(symbol_name) => {
                vec![
                    AssemblerCodeBlock::new_header_comment(&format!(
                        "Pop value to symbol '{symbol_name}'"
                    )),
                    load_symbol_value_to_d(symbol_name),
                    pop_to_address_written_in_d(),
                ]
            }
            semantics::PopTarget::DirectAddress {
                mapping_type,
                offset,
            } => {
                vec![
                    AssemblerCodeBlock::new_header_comment(&format!(
                        "Pop value to directly mapped memory segment '{mapping_type:?}' + offset {offset}"
                    )),
                    load_direct_address_to_d(mapping_type, offset),
                    pop_to_address_written_in_d(),
                ]
            }
            semantics::PopTarget::IndirectAddress {
                mapping_type,
                offset,
            } => {
                vec![
                    AssemblerCodeBlock::new_header_comment(&format!(
                        "Pop value to in-directly mapped memory segment '{mapping_type:?}' + offset({offset})"
                    )),
                    load_indirect_address_to_d(mapping_type, offset),
                    pop_to_address_written_in_d(),
                ]
            }
        },
    }
}

// スタックポインタが示すメモリ位置にDレジスタの値を書き込むコマンド
fn write_d_to_stack() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "push D register value to stack",
        &[
            // @SP
            // A=M
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
            // スタックポインタの値をインクリメント
            // @SP
            // M=M+1
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::MPlusOne,
                jump: None,
            }),
        ],
    )
}

// 定数値をDレジスタに書き込む
// @index
// D=A
fn load_constant_to_d(value: u16) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        format!("load constant value {value} to D register").as_str(),
        &[
            hack::Command::A(hack::ACommand::Address(value)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::A,
                jump: None,
            }),
        ],
    )
}

// Dレジスタに保存されたアドレス値が指し示す
// メモリ位置の値をDレジスタに書き込む
fn load_value_specified_by_address_in_d() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "load value specified by address in D register to D register",
        &[
            // Dレジスタに保存されたアドレスが指すpush元メモリ位置にある値を、Dレジスタに書き込む
            // A=D
            // D=M
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
        ],
    )
}

// symbol 値のアドレスが指し示す値をDレジスタに書き込む
fn load_value_to_d_by_symbol_address(symbol_name: String) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "load value to D register by symbol address",
        &[
            // @symbol
            // D=M
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(
                symbol_name.as_str(),
            ))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
        ],
    )
}

// symbol 値自体をDレジスタに書き込む
fn load_symbol_value_to_d(symbol_name: String) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "load symbol value to D register",
        &[
            // @symbol
            // D=A
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(
                symbol_name.as_str(),
            ))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::A,
                jump: None,
            }),
        ],
    )
}

// base + offset で求まる直接アドレス値をDレジスタに書き込む
fn load_direct_address_to_d(
    mapping_type: semantics::DirectMappingType,
    offset: u16,
) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "compute direct address by base + offset, and load to D register",
        &[
            // ベースアドレス取得
            // @R3 or R5
            // D=A
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(
                match mapping_type {
                    semantics::DirectMappingType::Pointer => "R3",
                    semantics::DirectMappingType::Temp => "R5",
                },
            ))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::A,
                jump: None,
            }),
            // ベースアドレスにインデックスを加算してpop先アドレスを求める
            // @offset
            // D=D+A
            hack::Command::A(hack::ACommand::Address(offset)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::DPlusA,
                jump: None,
            }),
        ],
    )
}

// base + offset で求まる間接アドレス値をDレジスタに書き込む
fn load_indirect_address_to_d(
    mapping_type: semantics::InDirectMappingType,
    offset: u16,
) -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "compute address by base + offset, and load to D register",
        &[
            // セグメントのベースアドレス取得
            // @ARG
            // D=M
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new(
                match mapping_type {
                    semantics::InDirectMappingType::Argument => "ARG",
                    semantics::InDirectMappingType::Local => "LCL",
                    semantics::InDirectMappingType::This => "THIS",
                    semantics::InDirectMappingType::That => "THAT",
                },
            ))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            // ベースアドレスにインデックスを加算してpop先アドレスを求める
            // @offset
            // D=D+A
            hack::Command::A(hack::ACommand::Address(offset)),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::DPlusA,
                jump: None,
            }),
        ],
    )
}

// Dレジスタに保存されたアドレスのメモリ位置にStackから値をPopする
fn pop_to_address_written_in_d() -> AssemblerCodeBlock {
    AssemblerCodeBlock::new(
        "pop value to memory address specified by D register",
        &[
            // Dレジスタに保存されているpop先アドレスをRAM[13]に退避
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R13"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
            // スタックポインタの値 - 1の位置の値（スタックの最後の要素）の値をDレジスタに格納する
            // @SP
            // A=M-1
            // D=M
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::MMinusOne,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::D),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            // // スタックポインタの値をデクリメントする
            // @SP
            // M=M-1
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("SP"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::MMinusOne,
                jump: None,
            }),
            // // R13からpop先アドレスをロードし、それが指すメモリ位置にDレジスタの値を格納する
            // @R13
            // A=M
            // M=D
            hack::Command::A(hack::ACommand::Symbol(hack::Symbol::new("R13"))),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::A),
                comp: hack::CompMnemonic::M,
                jump: None,
            }),
            hack::Command::C(hack::CCommand {
                dest: Some(hack::DestMnemonic::M),
                comp: hack::CompMnemonic::D,
                jump: None,
            }),
        ],
    )
}
